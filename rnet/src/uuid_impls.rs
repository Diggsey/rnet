use uuid::Uuid;

use crate::{hidden::GeneratorContext, FromNet, Net, ToNet};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Guid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

unsafe impl Net for Uuid {
    type Raw = Guid;

    fn gen_type(_ctx: &mut GeneratorContext) -> Box<str> {
        "Guid".into()
    }

    fn gen_raw_type(_ctx: &mut GeneratorContext) -> Box<str> {
        "Guid".into()
    }

    fn is_nullable(_ctx: &mut GeneratorContext) -> bool {
        false
    }
}

unsafe impl FromNet for Uuid {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        Uuid::from_fields_le(arg.a, arg.b, arg.c, &arg.d).unwrap()
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        arg.into()
    }
}

unsafe impl ToNet for Uuid {
    fn into_raw(self) -> Self::Raw {
        let (a, b, c, &d) = self.to_fields_le();
        Guid { a, b, c, d }
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        arg.into()
    }
}
