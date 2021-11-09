//! rnet-macros
//!
//! Procedural macros for `rnet`
#![deny(missing_docs)]

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Parser},
    visit_mut::{self, VisitMut},
    AttrStyle, Attribute, AttributeArgs, DataStruct, DeriveInput, Error, FnArg, Generics, Lifetime,
    Pat, Result, Signature, Token, Type, TypeReference, TypeTuple, Visibility,
};

#[derive(Clone)]
struct MaybeItemFn {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    block: TokenStream2,
}

/// This parses a `TokenStream` into a `MaybeItemFn`
/// (just like `ItemFn`, but skips parsing the body).
impl Parse for MaybeItemFn {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        let block: TokenStream2 = input.parse()?;
        Ok(Self {
            attrs,
            vis,
            sig,
            block,
        })
    }
}

impl ToTokens for MaybeItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(
            self.attrs
                .iter()
                .filter(|attr| matches!(attr.style, AttrStyle::Outer)),
        );
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

fn parse_attribute_args(input: ParseStream) -> Result<AttributeArgs> {
    let mut metas = Vec::new();

    loop {
        if input.is_empty() {
            break;
        }
        let value = input.parse()?;
        metas.push(value);
        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
    }

    Ok(metas)
}

struct LifetimeInjector {
    lifetime: Lifetime,
}

impl VisitMut for LifetimeInjector {
    fn visit_type_reference_mut(&mut self, i: &mut TypeReference) {
        visit_mut::visit_type_reference_mut(self, i);
        if i.lifetime.is_none() {
            i.lifetime = Some(self.lifetime.clone())
        }
    }
    fn visit_lifetime_mut(&mut self, i: &mut Lifetime) {
        visit_mut::visit_lifetime_mut(self, i);
        if i.ident == "_" {
            *i = self.lifetime.clone();
        }
    }
}

fn net_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream2> {
    let root = quote! { ::rnet::hidden };
    let _args = parse_attribute_args.parse(attr)?;
    let mut inner_fn = MaybeItemFn::parse.parse(item)?;

    let fn_name_str = inner_fn.sig.ident.to_string();
    let fn_name = format_ident!("rnet_export_{}", fn_name_str);
    inner_fn.sig.ident = Ident::new("inner", Span::call_site());

    let args = inner_fn
        .sig
        .inputs
        .iter()
        .enumerate()
        .map(|(i, arg)| match arg {
            FnArg::Receiver(_) => Err(Error::new_spanned(arg, "`self` parameter not supported")),
            FnArg::Typed(t) => Ok((
                match &*t.pat {
                    Pat::Ident(x) => x.ident.to_string(),
                    _ => format!("arg{}", i),
                },
                t.ty.clone(),
            )),
        })
        .collect::<Result<Vec<_>>>()?;

    let (arg_names, arg_types): (Vec<_>, Vec<_>) = args.into_iter().unzip();

    let mut lifetime_injector = LifetimeInjector {
        lifetime: Lifetime::new("'net", Span::call_site()),
    };
    let local_arg_types = arg_types.iter().cloned().map(|mut arg| {
        lifetime_injector.visit_type_mut(&mut *arg);
        arg
    });

    let ret_type = match &inner_fn.sig.output {
        syn::ReturnType::Default => Box::new(Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: Default::default(),
        })),
        syn::ReturnType::Type(_, t) => t.clone(),
    };

    let arg_idents: Vec<_> = (0..arg_types.len())
        .map(|i| format_ident!("arg{}", i))
        .collect();

    Ok(quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #fn_name<'net>(#(
            #arg_idents: <<#local_arg_types as #root::FromNetArg<'net>>::Owned as #root::Net>::Raw
        ),*) -> <#ret_type as #root::ToNetReturn>::RawReturn {
            #[#root::linkme::distributed_slice(#root::EXPORTED_FNS)]
            static EXPORTED_FN: #root::FnDesc = #root::FnDesc {
                name: #fn_name_str,
                args: &[#(
                    #root::ArgDesc {
                        name: #arg_names,
                        ty_: <<#arg_types as #root::FromNetArg>::Owned as #root::FromNet>::FROM_DESC,
                    }
                ),*],
                ret_ty: <#ret_type as #root::ToNetReturn>::RETURN_DESC,
            };

            #inner_fn

            #root::ToNetReturn::to_raw_return(inner(#(
                #root::FromNetArg::borrow_or_take(&mut Some(#root::FromNet::from_raw(#arg_idents)))
            ),*))
        }
    })
}

/// This attribute can be applied to a standalone function to allow it to be called
/// from .net.
#[proc_macro_attribute]
pub fn net(attr: TokenStream, item: TokenStream) -> TokenStream {
    match net_impl(attr, item.clone()) {
        Ok(res) => res.into(),
        Err(e) => {
            let mut res: TokenStream = e.into_compile_error().into();
            res.extend(item);
            res
        }
    }
}

fn derive_net_struct_impl(
    name: &Ident,
    generics: &Generics,
    data: &DataStruct,
) -> Result<TokenStream2> {
    let root = quote! { ::rnet::hidden };
    let name_str = name.to_string();
    let (field_name_ident, field_type): (Vec<_>, Vec<_>) = data
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field
                .ident
                .clone()
                .unwrap_or_else(|| format_ident!("elem{}", i));
            ((ident.to_string(), ident), &field.ty)
        })
        .unzip();
    let (field_name_str, field_name): (Vec<_>, Vec<_>) = field_name_ident.into_iter().unzip();
    let raw_name = format_ident!("Raw{}", name);
    let raw_name_str = format!("_Struct{}", name);

    Ok(quote! {
        const _: () = {
            #[#root::linkme::distributed_slice(#root::EXPORTED_STRUCTS)]
            static EXPORTED_STRUCT: #root::StructDesc = #root::StructDesc {
                name: #name_str,
                fields: &[#(
                    #root::FieldDesc {
                        name: #field_name_str,
                        ty_: &#root::TypeDesc {
                            marshal_in: Some(<#field_type as #root::FromNet>::gen_marshal),
                            marshal_out: Some(<#field_type as #root::ToNet>::gen_marshal),
                            ..*<#field_type as #root::Net>::DESC
                        },
                    }
                ),*],
            };

            #[repr(C)]
            pub struct #raw_name #generics {
                #(
                    pub #field_name: <#field_type as #root::Net>::Raw,
                )*
            }

            impl Copy for #raw_name {}
            impl Clone for #raw_name {
                fn clone(&self) -> Self {
                    *self
                }
            }
            impl Default for #raw_name {
                fn default() -> Self {
                    Self {
                        #(
                            #field_name: Default::default(),
                        )*
                    }
                }
            }

            unsafe impl #root::Net for #name {
                type Raw = #raw_name;

                fn gen_type(_ctx: &mut #root::GeneratorContext) -> Box<str> {
                    #name_str.into()
                }

                fn gen_raw_type(_ctx: &mut #root::GeneratorContext) -> Box<str> {
                    #raw_name_str.into()
                }
            }

            unsafe impl #root::FromNet for #name {
                unsafe fn from_raw(arg: Self::Raw) -> Self {
                    Self {
                        #(
                            #field_name: <#field_type as #root::FromNet>::from_raw(arg.#field_name),
                        )*
                    }
                }

                fn gen_marshal(ctx: &mut #root::GeneratorContext, arg: &str) -> Box<str> {
                    format!("{}.Encode({})", Self::gen_raw_type(ctx), arg).into()
                }
            }

            unsafe impl #root::ToNet for #name {
                fn into_raw(self) -> Self::Raw {
                    #raw_name {
                        #(
                            #field_name: #root::ToNet::into_raw(self.#field_name),
                        )*
                    }
                }

                fn gen_marshal(_ctx: &mut #root::GeneratorContext, arg: &str) -> Box<str> {
                    format!("({}).Decode()", arg).into()
                }
            }
        };
    })
}

fn derive_net_impl(item: TokenStream) -> Result<TokenStream2> {
    let derive_input = DeriveInput::parse.parse(item)?;

    match &derive_input.data {
        syn::Data::Struct(s) => {
            derive_net_struct_impl(&derive_input.ident, &derive_input.generics, s)
        }
        _ => Err(Error::new_spanned(
            derive_input,
            "Net derive can only be applied to structs",
        )),
    }
}

/// This derive will implement the `Net`, `ToNet`, and `FromNet`
/// traits for the given struct, allowing it to be passed to or
/// returned from .net code.
#[proc_macro_derive(Net)]
pub fn derive_net(item: TokenStream) -> TokenStream {
    match derive_net_impl(item) {
        Ok(res) => res.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
