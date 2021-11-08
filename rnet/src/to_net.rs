use crate::types::{GeneratorContext, TypeDesc};
use crate::{none_ty, Net};

/// This trait is implemented for Rust types which can be sent
/// to .net code.
pub unsafe trait ToNet: Net + ToNetReturn {
    #[doc(hidden)]
    const TO_DESC: TypeDesc = TypeDesc {
        marshal_out: Some(Self::gen_marshal),
        ..Self::DESC
    };
    #[doc(hidden)]
    fn into_raw(self) -> Self::Raw;
    #[doc(hidden)]
    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str>;
}

/// This trait is implemented for Rust types which can be used as arguments
/// to .net delegates. This is a superset of types which implement
/// `ToNet`, and allows passing types with lifetime arguments, like `&str`.
pub unsafe trait ToNetArg: Sized {
    #[doc(hidden)]
    type Owned: ToNet;
    #[doc(hidden)]
    fn to_owned(self) -> Self::Owned;
    #[doc(hidden)]
    fn to_owned_raw(self) -> <Self::Owned as Net>::Raw {
        self.to_owned().into_raw()
    }
}

unsafe impl<'a, T: ToNet> ToNetArg for T {
    type Owned = Self;
    fn to_owned(self) -> Self::Owned {
        self
    }
}

/// This trait is implemented for Rust types which can be returned from
/// exported functions. This is a superset of types which implement `ToNet`,
/// and allows returning eg. the unit `()` type, whose .net equivalent
/// `void` cannot be used as a normal type.
pub unsafe trait ToNetReturn {
    #[doc(hidden)]
    const RETURN_DESC: TypeDesc;
    #[doc(hidden)]
    type RawReturn;
    #[doc(hidden)]
    fn to_raw_return(self) -> Self::RawReturn;
}

unsafe impl<T: ToNet> ToNetReturn for T {
    const RETURN_DESC: TypeDesc = T::TO_DESC;
    type RawReturn = <Self as Net>::Raw;
    fn to_raw_return(self) -> Self::RawReturn {
        let raw = self.into_raw();

        // Transmute necessary because rustc can't figure out that the
        // types are the same...
        unsafe { std::mem::transmute_copy(&raw) }
    }
}

unsafe impl ToNetReturn for () {
    const RETURN_DESC: TypeDesc = TypeDesc {
        net_ty: none_ty,
        base_ty: none_ty,
        raw_ty: none_ty,
        marshal_in: None,
        marshal_out: Some(|_, arg| arg.into()),
    };
    type RawReturn = ();

    fn to_raw_return(self) -> Self::RawReturn {}
}
