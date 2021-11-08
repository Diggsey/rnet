use crate::{FromNet, FromNetArg, ToNet, ToNetArg};

unsafe impl<'a> FromNetArg<'a> for &'a str {
    type Owned = Box<str>;

    fn borrow_or_take(src: &'a mut Option<Self::Owned>) -> Self {
        src.as_deref().unwrap()
    }
}

unsafe impl<'a, T: FromNet> FromNetArg<'a> for &'a [T] {
    type Owned = Box<[T]>;

    fn borrow_or_take(src: &'a mut Option<Self::Owned>) -> Self {
        src.as_deref().unwrap()
    }
}

unsafe impl<'a> ToNetArg for &'a str {
    type Owned = Box<str>;

    fn to_owned(self) -> Self::Owned {
        self.into()
    }
}

unsafe impl<'a, T: ToNet + Clone> ToNetArg for &'a [T] {
    type Owned = Box<[T]>;

    fn to_owned(self) -> Self::Owned {
        self.iter().cloned().collect()
    }
}
