use std::marker::PhantomData;

use crate::types::{GeneratorContext, RawDelegate};
use crate::{FromNet, FromNetReturn, Net, ToNet, ToNetArg};

struct Delegate(RawDelegate);

impl Delegate {
    unsafe fn ptr<T>(&self) -> T {
        std::mem::transmute_copy(&self.0.call_fn)
    }
    fn into_raw(self) -> RawDelegate {
        let raw = self.0;
        std::mem::forget(self);
        raw
    }
}

impl Clone for Delegate {
    fn clone(&self) -> Self {
        // Increment reference count
        (self.0.manage_fn.unwrap())(self.0.call_fn, 1);
        Self(self.0)
    }
}

impl Drop for Delegate {
    fn drop(&mut self) {
        // Decrement reference count
        (self.0.manage_fn.unwrap())(self.0.call_fn, -1);
    }
}

macro_rules! define_delegates {
    ($(
        $name:ident<$($arg:ident),*> ($($argname:ident),*) = $lit:literal,
    )*) => {
        $(
            /// Rust type which can store and call a corresponding `Func<...>` or `Action<...>` delegate
            /// from .net.
            pub struct $name<TR $(, $arg)*>(Delegate, PhantomData<fn($($arg),*) -> TR>);
            impl<TR $(, $arg)*> Clone for $name<TR $(, $arg)*> {
                fn clone(&self) -> Self {
                    Self(self.0.clone(), self.1)
                }
            }
            impl<TR: FromNetReturn $(, $arg: ToNet)*> $name<TR $(, $arg)*> {
                /// Calls the contained delegate.
                #[allow(clippy::too_many_arguments)]
                pub fn call(&self $(, $argname: impl ToNetArg<Owned = $arg>)*) -> TR {
                    unsafe {
                        TR::from_raw_return(self.0.ptr::<fn($($arg::Raw),*) -> TR::RawReturn>()(
                            $($argname.to_owned_raw()),*
                        ))
                    }
                }
            }
            unsafe impl<TR: FromNetReturn $(, $arg: ToNet)*> Net for $name<TR $(, $arg)*> {
                type Raw = RawDelegate;

                fn gen_type() -> Box<str> {
                    if let Some(base_ty) = (TR::RETURN_DESC.base_ty)() {
                        if $lit.is_empty() {
                            format!("Func<{}>", base_ty)
                        } else {
                            format!(concat!("Func<", $lit , ", {}>"), $($arg::gen_type(),)* base_ty)
                        }
                    } else {
                        format!(concat!("Action<", $lit , ">"), $($arg::gen_type(),)*)
                    }.into()
                }

                fn gen_raw_type() -> Box<str> {
                    "_RawDelegate".into()
                }
            }
            unsafe impl<TR: FromNetReturn $(, $arg: ToNet)*> FromNet for $name<TR $(, $arg)*> {
                unsafe fn from_raw(arg: Self::Raw) -> Self {
                    Self(Delegate(arg), PhantomData)
                }

                fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
                    let ty_ = Self::gen_type();
                    let new_arg = ctx.get_unique_identifier("_arg");
                    let arg_names = format!(
                        $lit,
                        $(
                            format!("{}_{}", new_arg, stringify!($argname))
                        ),*
                    );
                    let arg_marshal = format!(
                        $lit,
                        $(
                            $arg::gen_marshal(ctx, &format!("{}_{}", new_arg, stringify!($argname)))
                        ),*
                    );
                    let call_code = format!(
                        concat!("{}({})"),
                        new_arg,
                        arg_marshal
                    );
                    let raw_ret_ty = if let Some(raw_ret_ty) = (TR::RETURN_DESC.raw_ty)() {
                        raw_ret_ty
                    } else {
                        "void".into()
                    };
                    let fn_body = (TR::RETURN_DESC.marshal_in.unwrap())(ctx, &call_code);

                    let local_delegate_name = ctx.get_unique_identifier("_LocalDelegate");
                    ctx.add_item(&format!(
                        concat!(
                            "[UnmanagedFunctionPointer(CallingConvention.Cdecl)] delegate {} {}(",
                                $lit,
                            ");",
                        ),
                        raw_ret_ty,
                        local_delegate_name,
                        $(format!(concat!("{} ", stringify!($argname)), $arg::gen_raw_type()),)*
                    ));

                    format!(
                        concat!(
                            "((Func<{}, _RawDelegate>)({new_arg} => _AllocDelegate(new {}(({}) => {}), {new_arg})))({})"
                        ),
                        ty_,
                        local_delegate_name,
                        arg_names,
                        fn_body,
                        arg,
                        new_arg=new_arg
                    )
                    .into()
                }
            }
            unsafe impl<TR: FromNetReturn $(, $arg: ToNet)*> ToNet for $name<TR $(, $arg)*> {
                fn into_raw(self) -> Self::Raw {
                    self.0.into_raw()
                }
                fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
                    format!("({})_FreeDelegate({})", Self::gen_type(), arg).into()
                }
            }
        )*
    }
}

define_delegates! {
    Delegate0<> () = "",
    Delegate1<T0> (arg0) = "{}",
    Delegate2<T0, T1> (arg0, arg1) = "{},{}",
    Delegate3<T0, T1, T2> (arg0, arg1, arg2) = "{},{},{}",
    Delegate4<T0, T1, T2, T3> (arg0, arg1, arg2, arg3) = "{},{},{},{}",
    Delegate5<T0, T1, T2, T3, T4> (arg0, arg1, arg2, arg3, arg4) = "{},{},{},{},{}",
    Delegate6<T0, T1, T2, T3, T4, T5> (arg0, arg1, arg2, arg3, arg4, arg5) = "{},{},{},{},{},{}",
    Delegate7<T0, T1, T2, T3, T4, T5, T6> (arg0, arg1, arg2, arg3, arg4, arg5, arg6) = "{},{},{},{},{},{},{}",
    Delegate8<T0, T1, T2, T3, T4, T5, T6, T7> (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) = "{},{},{},{},{},{},{},{}",
    Delegate9<T0, T1, T2, T3, T4, T5, T6, T7, T8> (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) = "{},{},{},{},{},{},{},{},{}",
}
