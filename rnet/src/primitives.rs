use crate::types::GeneratorContext;
use crate::{FromNet, Net, ToNet};

macro_rules! define_primitives {
    ($($a:ty => $b:literal,)*) => {
        $(
            unsafe impl Net for $a {
                type Raw = $a;
                const TRIVIAL: bool = true;

                fn gen_type(_ctx: &mut GeneratorContext) -> Box<str> {
                    $b.into()
                }

                fn gen_raw_type(_ctx: &mut GeneratorContext) -> Box<str> {
                    $b.into()
                }
            }

            unsafe impl FromNet for $a {
                unsafe fn from_raw(arg: Self::Raw) -> Self {
                    arg
                }

                fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
                    arg.into()
                }
            }

            unsafe impl ToNet for $a {
                fn into_raw(self) -> Self::Raw {
                    self
                }
                fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
                    arg.into()
                }
            }
        )*
    };
}

define_primitives! {
    u8 => "byte",
    i8 => "sbyte",
    u16 => "ushort",
    i16 => "short",
    u32 => "uint",
    i32 => "int",
    u64 => "ulong",
    i64 => "long",
    usize => "UIntPtr",
    isize => "IntPtr",
    f32 => "float",
    f64 => "double",
}

unsafe impl Net for bool {
    type Raw = bool;
    const TRIVIAL: bool = true;

    fn gen_type(_ctx: &mut GeneratorContext) -> Box<str> {
        "bool".into()
    }

    fn gen_raw_type(_ctx: &mut GeneratorContext) -> Box<str> {
        "byte".into()
    }
}

unsafe impl FromNet for bool {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        arg
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("({} ? (byte)1 : (byte)0)", arg).into()
    }
}

unsafe impl ToNet for bool {
    fn into_raw(self) -> Self::Raw {
        self
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("({} != 0)", arg).into()
    }
}
