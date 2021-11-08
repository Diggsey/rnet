use std::{
    any::TypeId,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    mem::{align_of, size_of},
    slice,
    str::from_boxed_utf8_unchecked,
    sync::Arc,
};

use crate::{
    none_ty,
    tuples::{RawTuple2, RawTuple3},
    types::{GeneratorContext, RawOpaqueHandle, RawPtr, RawSlice},
    FromNet, FromNetReturn, Net, NetException, ToNet, ToNetReturn, TypeDesc,
};

unsafe impl<T: Net> Net for Box<[T]> {
    type Raw = RawSlice;

    fn gen_type() -> Box<str> {
        format!("List<{}>", T::gen_type()).into()
    }

    fn gen_base_type() -> Box<str> {
        format!("IReadOnlyCollection<{}>", T::gen_type()).into()
    }

    fn gen_raw_type() -> Box<str> {
        "_RawSlice".into()
    }
}

unsafe impl<T: FromNet> FromNet for Box<[T]> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        if T::TRIVIAL {
            Box::from_raw(slice::from_raw_parts_mut(arg.ptr as *mut T, arg.len))
        } else {
            let vec: Vec<_> =
                Box::from_raw(slice::from_raw_parts_mut(arg.ptr as *mut T::Raw, arg.len))
                    .into_vec();
            vec.into_iter().map(|item| T::from_raw(item)).collect()
        }
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_AllocSlice<{}, {}>({}, {}, {}, {} => {})",
            T::gen_type(),
            T::gen_raw_type(),
            arg,
            size_of::<T::Raw>(),
            align_of::<T::Raw>(),
            new_arg,
            T::gen_marshal(ctx, &new_arg)
        )
        .into()
    }
}

unsafe impl<T: ToNet> ToNet for Box<[T]> {
    fn to_raw(self) -> Self::Raw {
        let len = self.len();
        let ptr = if T::TRIVIAL {
            Box::into_raw(self) as *mut _
        } else {
            let boxed_slice: Box<[T::Raw]> = self
                .into_vec()
                .into_iter()
                .map(|item| T::to_raw(item))
                .collect();
            Box::into_raw(boxed_slice) as *mut _
        };
        RawSlice { ptr, len }
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_FreeSlice<{item}, {}, List<{item}>>({}, {}, {}, {} => {})",
            T::gen_raw_type(),
            arg,
            size_of::<T::Raw>(),
            align_of::<T::Raw>(),
            new_arg,
            T::gen_marshal(ctx, &new_arg),
            item = T::gen_type(),
        )
        .into()
    }
}

unsafe impl Net for Box<str> {
    type Raw = RawSlice;

    fn gen_type() -> Box<str> {
        "String".into()
    }

    fn gen_raw_type() -> Box<str> {
        "_RawSlice".into()
    }
}

unsafe impl FromNet for Box<str> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        from_boxed_utf8_unchecked(<Box<[u8]> as FromNet>::from_raw(arg))
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("_AllocStr({})", arg).into()
    }
}

unsafe impl ToNet for Box<str> {
    fn to_raw(self) -> Self::Raw {
        <Box<[u8]> as ToNet>::to_raw(self.into_boxed_bytes())
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("_FreeStr({})", arg).into()
    }
}

unsafe impl<T: Net> Net for Box<T> {
    type Raw = RawPtr;

    fn gen_type() -> Box<str> {
        format!("Nullable<{}>", T::gen_type()).into()
    }

    fn gen_raw_type() -> Box<str> {
        "IntPtr".into()
    }
}

unsafe impl<T: FromNet> FromNet for Box<T> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        Box::from_raw(arg.0 as *mut T)
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!(
            "_AllocBox({}, {}, {})",
            T::gen_marshal(ctx, arg),
            size_of::<T>(),
            align_of::<T>()
        )
        .into()
    }
}

unsafe impl<T: ToNet> ToNet for Box<T> {
    fn to_raw(self) -> Self::Raw {
        RawPtr(Box::into_raw(self) as *mut ())
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        T::gen_marshal(
            ctx,
            &format!("_FreeBox({}, {}, {})", arg, size_of::<T>(), align_of::<T>()),
        )
    }
}

fn int_type_id<T: 'static>() -> u64 {
    unsafe { std::mem::transmute(TypeId::of::<T>()) }
}

unsafe impl<T: Send + Sync + 'static> Net for Arc<T> {
    type Raw = RawOpaqueHandle;

    fn gen_type() -> Box<str> {
        "_OpaqueHandle".into()
    }

    fn gen_raw_type() -> Box<str> {
        "_RawOpaqueHandle".into()
    }
}

unsafe impl<T: Send + Sync + 'static> FromNet for Arc<T> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        Arc::increment_strong_count(arg.ptr as *const _);
        Arc::from_raw(arg.ptr as *const _)
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("({}).ToInner({})", arg, int_type_id::<T>()).into()
    }
}

unsafe impl<T: Send + Sync + 'static> ToNet for Arc<T> {
    fn to_raw(self) -> Self::Raw {
        unsafe extern "C" fn rnet_drop_arc<T>(ptr: *mut ()) {
            Arc::from_raw(ptr as *const T);
        }

        RawOpaqueHandle {
            ptr: Arc::into_raw(self) as *mut (),
            drop_fn: Some(rnet_drop_arc::<T>),
            type_id: int_type_id::<T>(),
        }
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("new _OpaqueHandle({})", arg).into()
    }
}

unsafe impl<T: Net> Net for Option<T> {
    type Raw = RawTuple2<T::Raw, bool>;

    fn gen_type() -> Box<str> {
        format!("Nullable<{}>", T::gen_type()).into()
    }

    fn gen_raw_type() -> Box<str> {
        format!("_RawTuple<{}, {}>", T::gen_raw_type(), bool::gen_raw_type()).into()
    }
}

unsafe impl<T: FromNet> FromNet for Option<T> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        if arg.1 {
            Some(T::from_raw(arg.0))
        } else {
            None
        }
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_EncodeOption({}, {} => {})",
            arg,
            new_arg,
            T::gen_marshal(ctx, &new_arg),
        )
        .into()
    }
}

unsafe impl<T: ToNet> ToNet for Option<T> {
    fn to_raw(self) -> Self::Raw {
        if let Some(v) = self {
            RawTuple2(v.to_raw(), true)
        } else {
            RawTuple2(Default::default(), false)
        }
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_DecodeOption({}, {} => {})",
            arg,
            new_arg,
            T::gen_marshal(ctx, &new_arg)
        )
        .into()
    }
}

fn marshal_result_out<T: ToNet>(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
    let new_arg = ctx.get_unique_identifier("_arg");
    format!(
        "_DecodeResult({}, {} => {})",
        arg,
        new_arg,
        T::gen_marshal(ctx, &new_arg)
    )
    .into()
}

fn marshal_void_result_out(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
    format!("_DecodeVoidResult({})", arg).into()
}

unsafe impl<T: ToNet, E: Display> ToNetReturn for Result<T, E> {
    const RETURN_DESC: TypeDesc = TypeDesc {
        raw_ty: || Some(format!("_RawTuple<{}, _RawSlice, byte>", T::gen_raw_type()).into()),
        marshal_out: Some(marshal_result_out::<T>),
        ..T::TO_DESC
    };

    type RawReturn = RawTuple3<T::Raw, RawSlice, bool>;

    fn to_raw_return(self) -> Self::RawReturn {
        match self {
            Ok(x) => RawTuple3(x.to_raw(), Default::default(), true),
            Err(e) => RawTuple3(Default::default(), e.to_string().to_raw(), false),
        }
    }
}

unsafe impl<E: Display> ToNetReturn for Result<(), E> {
    const RETURN_DESC: TypeDesc = TypeDesc {
        net_ty: none_ty,
        base_ty: none_ty,
        raw_ty: || Some("_RawTuple<_RawSlice, byte>".into()),
        marshal_in: None,
        marshal_out: Some(marshal_void_result_out),
    };

    type RawReturn = RawTuple2<RawSlice, bool>;

    fn to_raw_return(self) -> Self::RawReturn {
        match self {
            Ok(()) => RawTuple2(Default::default(), true),
            Err(e) => RawTuple2(e.to_string().to_raw(), false),
        }
    }
}

fn marshal_result_in<T: FromNet>(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
    format!("_EncodeResult(() => {})", T::gen_marshal(ctx, arg)).into()
}

fn marshal_void_result_in(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
    format!("_EncodeResult(() => {})", arg).into()
}

unsafe impl<T: FromNet> FromNetReturn for Result<T, NetException> {
    const RETURN_DESC: TypeDesc = TypeDesc {
        raw_ty: || Some(format!("_RawTuple<{}, _RawSlice, byte>", T::gen_raw_type()).into()),
        marshal_in: Some(marshal_result_in::<T>),
        ..T::FROM_DESC
    };
    type RawReturn = RawTuple3<T::Raw, RawSlice, bool>;
    unsafe fn from_raw_return(arg: Self::RawReturn) -> Self {
        if arg.2 {
            Ok(T::from_raw(arg.0))
        } else {
            Err(NetException(String::from_raw(arg.1)))
        }
    }
}

unsafe impl FromNetReturn for Result<(), NetException> {
    const RETURN_DESC: TypeDesc = TypeDesc {
        net_ty: none_ty,
        base_ty: none_ty,
        raw_ty: || Some("_RawTuple<_RawSlice, byte>".into()),
        marshal_in: Some(marshal_void_result_in),
        marshal_out: None,
    };
    type RawReturn = RawTuple2<RawSlice, bool>;
    unsafe fn from_raw_return(arg: Self::RawReturn) -> Self {
        if arg.1 {
            Ok(())
        } else {
            Err(NetException(String::from_raw(arg.0)))
        }
    }
}

unsafe impl<K: Net + Eq + Hash, V: Net> Net for HashMap<K, V> {
    type Raw = RawSlice;

    fn gen_type() -> Box<str> {
        format!("Dictionary<{},{}>", K::gen_type(), V::gen_type()).into()
    }

    fn gen_base_type() -> Box<str> {
        format!("IReadOnlyDictionary<{}, {}>", K::gen_type(), V::gen_type()).into()
    }

    fn gen_raw_type() -> Box<str> {
        "_RawSlice".into()
    }
}

unsafe impl<K: FromNet + Eq + Hash, V: FromNet> FromNet for HashMap<K, V> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        let vec: Vec<_> = Box::from_raw(slice::from_raw_parts_mut(
            arg.ptr as *mut <(K, V) as Net>::Raw,
            arg.len,
        ))
        .into_vec();
        vec.into_iter()
            .map(|item| <(K, V)>::from_raw(item))
            .collect()
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_AllocDict<{}, {}, {}, {}>({}, {}, {}, {} => {})",
            K::gen_type(),
            V::gen_type(),
            K::gen_raw_type(),
            V::gen_raw_type(),
            arg,
            size_of::<<(K, V) as Net>::Raw>(),
            align_of::<<(K, V) as Net>::Raw>(),
            new_arg,
            <(K, V)>::gen_marshal(ctx, &new_arg)
        )
        .into()
    }
}

unsafe impl<K: ToNet + Eq + Hash, V: ToNet> ToNet for HashMap<K, V> {
    fn to_raw(self) -> Self::Raw {
        let len = self.len();
        let ptr = {
            let boxed_slice: Box<[<(K, V) as Net>::Raw]> = self
                .into_iter()
                .map(|item| <(K, V)>::to_raw(item))
                .collect();
            Box::into_raw(boxed_slice) as *mut _
        };
        RawSlice { ptr, len }
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_FreeDict<{key}, {value}, {}, {}, Dictionary<{key}, {value}>>({}, {}, {}, {} => {})",
            K::gen_raw_type(),
            V::gen_raw_type(),
            arg,
            size_of::<<(K, V) as Net>::Raw>(),
            align_of::<<(K, V) as Net>::Raw>(),
            new_arg,
            <(K, V)>::gen_marshal(ctx, &new_arg),
            key = K::gen_type(),
            value = V::gen_type(),
        )
        .into()
    }
}

unsafe impl<K: Net + Ord, V: Net> Net for BTreeMap<K, V> {
    type Raw = RawSlice;

    fn gen_type() -> Box<str> {
        format!("SortedDictionary<{},{}>", K::gen_type(), V::gen_type()).into()
    }

    fn gen_base_type() -> Box<str> {
        format!("IReadOnlyDictionary<{}, {}>", K::gen_type(), V::gen_type()).into()
    }

    fn gen_raw_type() -> Box<str> {
        "_RawSlice".into()
    }
}

unsafe impl<K: FromNet + Ord, V: FromNet> FromNet for BTreeMap<K, V> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        let vec: Vec<_> = Box::from_raw(slice::from_raw_parts_mut(
            arg.ptr as *mut <(K, V) as Net>::Raw,
            arg.len,
        ))
        .into_vec();
        vec.into_iter()
            .map(|item| <(K, V)>::from_raw(item))
            .collect()
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_AllocDict<{}, {}, {}, {}>({}, {}, {}, {} => {})",
            K::gen_type(),
            V::gen_type(),
            K::gen_raw_type(),
            V::gen_raw_type(),
            arg,
            size_of::<<(K, V) as Net>::Raw>(),
            align_of::<<(K, V) as Net>::Raw>(),
            new_arg,
            <(K, V)>::gen_marshal(ctx, &new_arg)
        )
        .into()
    }
}

unsafe impl<K: ToNet + Ord, V: ToNet> ToNet for BTreeMap<K, V> {
    fn to_raw(self) -> Self::Raw {
        let len = self.len();
        let ptr = {
            let boxed_slice: Box<[<(K, V) as Net>::Raw]> = self
                .into_iter()
                .map(|item| <(K, V)>::to_raw(item))
                .collect();
            Box::into_raw(boxed_slice) as *mut _
        };
        RawSlice { ptr, len }
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_FreeDict<{key}, {value}, {}, {}, SortedDictionary<{key}, {value}>>({}, {}, {}, {} => {})",
            K::gen_raw_type(),
            V::gen_raw_type(),
            arg,
            size_of::<<(K, V) as Net>::Raw>(),
            align_of::<<(K, V) as Net>::Raw>(),
            new_arg,
            <(K, V)>::gen_marshal(ctx, &new_arg),
            key = K::gen_type(),
            value = V::gen_type(),
        )
        .into()
    }
}

unsafe impl<T: Net + Eq + Hash> Net for HashSet<T> {
    type Raw = RawSlice;

    fn gen_type() -> Box<str> {
        format!("HashSet<{}>", T::gen_type()).into()
    }

    fn gen_base_type() -> Box<str> {
        format!("IReadOnlyCollection<{}>", T::gen_type()).into()
    }

    fn gen_raw_type() -> Box<str> {
        "_RawSlice".into()
    }
}

unsafe impl<T: FromNet + Eq + Hash> FromNet for HashSet<T> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        let vec: Vec<_> =
            Box::from_raw(slice::from_raw_parts_mut(arg.ptr as *mut T::Raw, arg.len)).into_vec();
        vec.into_iter().map(|item| T::from_raw(item)).collect()
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_AllocSlice<{}, {}>({}, {}, {}, {} => {})",
            T::gen_type(),
            T::gen_raw_type(),
            arg,
            size_of::<T::Raw>(),
            align_of::<T::Raw>(),
            new_arg,
            T::gen_marshal(ctx, &new_arg)
        )
        .into()
    }
}

unsafe impl<T: ToNet + Eq + Hash> ToNet for HashSet<T> {
    fn to_raw(self) -> Self::Raw {
        let len = self.len();
        let ptr = {
            let boxed_slice: Box<[T::Raw]> = self.into_iter().map(|item| T::to_raw(item)).collect();
            Box::into_raw(boxed_slice) as *mut _
        };
        RawSlice { ptr, len }
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_FreeSlice<{item}, {}, HashSet<{item}>>({}, {}, {}, {} => {})",
            T::gen_raw_type(),
            arg,
            size_of::<T::Raw>(),
            align_of::<T::Raw>(),
            new_arg,
            T::gen_marshal(ctx, &new_arg),
            item = T::gen_type(),
        )
        .into()
    }
}

unsafe impl<T: Net + Ord> Net for BTreeSet<T> {
    type Raw = RawSlice;

    fn gen_type() -> Box<str> {
        format!("SortedSet<{}>", T::gen_type()).into()
    }

    fn gen_base_type() -> Box<str> {
        format!("IReadOnlyCollection<{}>", T::gen_type()).into()
    }

    fn gen_raw_type() -> Box<str> {
        "_RawSlice".into()
    }
}

unsafe impl<T: FromNet + Ord> FromNet for BTreeSet<T> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        let vec: Vec<_> =
            Box::from_raw(slice::from_raw_parts_mut(arg.ptr as *mut T::Raw, arg.len)).into_vec();
        vec.into_iter().map(|item| T::from_raw(item)).collect()
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_AllocSlice<{}, {}>({}, {}, {}, {} => {})",
            T::gen_type(),
            T::gen_raw_type(),
            arg,
            size_of::<T::Raw>(),
            align_of::<T::Raw>(),
            new_arg,
            T::gen_marshal(ctx, &new_arg)
        )
        .into()
    }
}

unsafe impl<T: ToNet + Ord> ToNet for BTreeSet<T> {
    fn to_raw(self) -> Self::Raw {
        let len = self.len();
        let ptr = {
            let boxed_slice: Box<[T::Raw]> = self.into_iter().map(|item| T::to_raw(item)).collect();
            Box::into_raw(boxed_slice) as *mut _
        };
        RawSlice { ptr, len }
    }

    fn gen_marshal(ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        let new_arg = ctx.get_unique_identifier("_arg");
        format!(
            "_FreeSlice<{item}, {}, SortedSet<{item}>>({}, {}, {}, {} => {})",
            T::gen_raw_type(),
            arg,
            size_of::<T::Raw>(),
            align_of::<T::Raw>(),
            new_arg,
            T::gen_marshal(ctx, &new_arg),
            item = T::gen_type(),
        )
        .into()
    }
}
