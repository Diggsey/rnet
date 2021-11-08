use crate::{
    none_ty,
    types::{GeneratorContext, TypeDesc},
    Net,
};

/// This trait is implemented for Rust types which can be received
/// from .net code.
pub unsafe trait FromNet: Net + for<'a> FromNetArg<'a> {
    #[doc(hidden)]
    const FROM_DESC: TypeDesc = TypeDesc {
        marshal_in: Some(Self::gen_marshal),
        ..Self::DESC
    };
    #[doc(hidden)]
    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str>;
    #[doc(hidden)]
    unsafe fn from_raw(arg: Self::Raw) -> Self;
}

/// This trait is implemented for Rust types which can be used as arguments
/// to exported functions. This is a superset of types which implement
/// `FromNet`, and allows passing types with lifetime arguments, like `&str`.
pub unsafe trait FromNetArg<'a>: 'a {
    #[doc(hidden)]
    type Owned: FromNet;
    #[doc(hidden)]
    fn borrow_or_take(src: &'a mut Option<Self::Owned>) -> Self;
}

unsafe impl<'a, T: FromNet> FromNetArg<'a> for T {
    type Owned = Self;
    fn borrow_or_take(src: &'a mut Option<Self::Owned>) -> Self {
        // Safety: the two types are identical, but rustc doesn't realise it...
        let src: &'a mut Option<Self> = unsafe { std::mem::transmute(src) };
        src.take().unwrap()
    }
}

/// This trait is implemented for Rust types which can be returned from
/// .net delegates. This is a superset of types which implement `FromNet`,
/// and allows returning eg. the unit `()` type, whose .net equivalent
/// `void` cannot be used as a normal type.
pub unsafe trait FromNetReturn: 'static {
    #[doc(hidden)]
    const RETURN_DESC: TypeDesc;
    #[doc(hidden)]
    type RawReturn;
    #[doc(hidden)]
    unsafe fn from_raw_return(arg: Self::RawReturn) -> Self;
}

unsafe impl<T: FromNet> FromNetReturn for T {
    const RETURN_DESC: TypeDesc = T::FROM_DESC;
    type RawReturn = <Self as Net>::Raw;
    unsafe fn from_raw_return(arg: Self::RawReturn) -> Self {
        Self::from_raw(arg)
    }
}

unsafe impl FromNetReturn for () {
    const RETURN_DESC: TypeDesc = TypeDesc {
        net_ty: none_ty,
        base_ty: none_ty,
        raw_ty: none_ty,
        marshal_in: Some(|_, arg| arg.into()),
        marshal_out: None,
    };
    type RawReturn = ();

    unsafe fn from_raw_return(_arg: Self::RawReturn) -> Self {}
}
