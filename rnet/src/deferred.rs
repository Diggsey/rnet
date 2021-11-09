use std::path::PathBuf;

use crate::types::GeneratorContext;
use crate::{FromNet, Net, ToNet};

macro_rules! defer_net {
    () => {};
    ([$($generics:tt)*] $a:ty: $b:ty => {
        [$($generics_from:tt)*] from: |$c:ident| $d:expr,
        [$($generics_to:tt)*] to: |$e:ident| $f:expr,
    }, $($rest:tt)*) => {
        unsafe impl $($generics)* Net for $a {
            type Raw = <$b as Net>::Raw;

            fn gen_type(ctx: &mut GeneratorContext) -> Box<str> {
                <$b as Net>::gen_type(ctx)
            }

            fn gen_base_type(ctx: &mut GeneratorContext) -> Box<str> {
                <$b as Net>::gen_base_type(ctx)
            }

            fn gen_raw_type(ctx: &mut GeneratorContext) -> Box<str> {
                <$b as Net>::gen_raw_type(ctx)
            }
        }
        unsafe impl $($generics_from)* FromNet for $a {
            unsafe fn from_raw(arg: Self::Raw) -> Self {
                let $c = <$b as FromNet>::from_raw(arg);
                $d
            }

            fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
                <$b as FromNet>::gen_marshal(ctx, arg)
            }
        }
        unsafe impl $($generics_to)* ToNet for $a {
            fn into_raw(self) -> Self::Raw {
                let $e = self;
                <$b as ToNet>::into_raw($f)
            }

            fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
                <$b as ToNet>::gen_marshal(ctx, arg)
            }
        }

        defer_net! {
            $($rest)*
        }
    };
    ($a:ty: $b:ty => {
        from: |$c:ident| $d:expr,
        to: |$e:ident| $f:expr,
    }, $($rest:tt)*) => {
        defer_net! {
            [] $a: $b => {
                [] from: |$c| $d,
                [] to: |$e| $f,
            },
            $($rest)*
        }
    }
}

defer_net! {
    String: Box<str> => {
        from: |x| x.into_string(),
        to: |x| x.into_boxed_str(),
    },
    PathBuf: String => {
        from: |x| x.into(),
        to: |x| x.into_os_string().into_string().expect("Path was not valid UTF-8"),
    },
    [<T: Net>] Vec<T>: Box<[T]> => {
        [<T: FromNet>] from: |x| x.into_vec(),
        [<T: ToNet>] to: |x| x.into_boxed_slice(),
    },
}
