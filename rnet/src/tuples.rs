use crate::types::GeneratorContext;
use crate::{FromNet, Net, ToNet};

macro_rules! define_tuple_impls {
    ($(
        $name:ident<$($arg:ident),*> = $lit:literal ($($n:tt)*),
    )*) => {
        $(
            #[derive(Copy, Clone, Default)]
            #[repr(C)]
            pub struct $name<$($arg),*>($(pub $arg),*);

            unsafe impl<$($arg: Net),*> Net for ($($arg,)*) {
                type Raw = $name<$($arg::Raw),*>;

                fn gen_type() -> Box<str> {
                    format!(concat!("(", $lit, ")"), $($arg::gen_type()),*).into()
                }

                fn gen_raw_type() -> Box<str> {
                    format!(concat!("_RawTuple<", $lit, ">"), $($arg::gen_raw_type()),*).into()
                }
            }

            unsafe impl<$($arg: FromNet),*> FromNet for ($($arg,)*) {
                unsafe fn from_raw(arg: Self::Raw) -> Self {
                    ($($arg::from_raw(arg.$n) ,)*)
                }

                fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
                    let raw_ty = Self::gen_raw_type();
                    let new_arg = ctx.get_unique_identifier("_arg");
                    let arg_marshal = format!($lit $(, format!("elem{} = {}", $n, $arg::gen_marshal(ctx, &format!("{}.Item{}", new_arg, $n + 1))))*);
                    format!(
                        "((Func<{}, {raw_ty}>)({} => new {raw_ty} {{ {} }}))({})",
                        Self::gen_type(),
                        new_arg,
                        arg_marshal,
                        arg,
                        raw_ty=raw_ty,
                    )
                    .into()
                }
            }

            unsafe impl<$($arg: ToNet),*> ToNet for ($($arg,)*) {
                fn to_raw(self) -> Self::Raw {
                    $name($($arg::to_raw(self.$n)),*)
                }

                fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
                    let new_arg = ctx.get_unique_identifier("_arg");
                    format!(
                        concat!(
                            "((Func<{}, {}>)({} => (", $lit, ")))({})"
                        ),
                        Self::gen_raw_type(),
                        Self::gen_type(),
                        new_arg,
                        $($arg::gen_marshal(ctx, &format!(concat!("{}.elem", stringify!($n)), new_arg)),)*
                        arg,
                    )
                    .into()
                }
            }

        )*
    };
}

define_tuple_impls! {
    RawTuple2<T0, T1> = "{},{}" (0 1),
    RawTuple3<T0, T1, T2> = "{},{},{}" (0 1 2),
    RawTuple4<T0, T1, T2, T3> = "{},{},{},{}" (0 1 2 3),
    RawTuple5<T0, T1, T2, T3, T4> = "{},{},{},{},{}" (0 1 2 3 4),
    RawTuple6<T0, T1, T2, T3, T4, T5> = "{},{},{},{},{},{}" (0 1 2 3 4 5),
    RawTuple7<T0, T1, T2, T3, T4, T5, T6> = "{},{},{},{},{},{},{}" (0 1 2 3 4 5 6),
    RawTuple8<T0, T1, T2, T3, T4, T5, T6, T7> = "{},{},{},{},{},{},{},{}" (0 1 2 3 4 5 6 7),
    RawTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8> = "{},{},{},{},{},{},{},{},{}" (0 1 2 3 4 5 6 7 8),
}
