use std::{marker::PhantomData, ptr};

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct RawPtr(pub *mut ());

impl Default for RawPtr {
    fn default() -> Self {
        Self(ptr::null_mut())
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct RawSlice {
    pub ptr: *mut (),
    pub len: usize,
}

impl Default for RawSlice {
    fn default() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
#[rustversion::since(1.72)]
pub struct RawOpaqueHandle {
    pub ptr: *mut (),
    pub drop_fn: Option<unsafe extern "C" fn(ptr: *mut ())>,
    pub type_id: u128,
}

#[rustversion::before(1.72)]
pub struct RawOpaqueHandle {
    pub ptr: *mut (),
    pub drop_fn: Option<unsafe extern "C" fn(ptr: *mut ())>,
    pub type_id: u64,
}

impl Default for RawOpaqueHandle {
    fn default() -> Self {
        Self {
            ptr: ptr::null_mut(),
            drop_fn: None,
            type_id: Default::default(),
        }
    }
}

#[doc(hidden)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TypeDesc {
    pub net_ty: fn(&mut GeneratorContext) -> Option<Box<str>>,
    pub base_ty: fn(&mut GeneratorContext) -> Option<Box<str>>,
    pub raw_ty: fn(&mut GeneratorContext) -> Option<Box<str>>,
    pub marshal_in: Option<fn(&mut GeneratorContext, &str) -> Box<str>>,
    pub marshal_out: Option<fn(&mut GeneratorContext, &str) -> Box<str>>,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArgDesc {
    pub name: &'static str,
    pub ty_: &'static TypeDesc,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FnDesc {
    pub name: &'static str,
    pub args: &'static [ArgDesc],
    pub ret_ty: &'static TypeDesc,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FieldDesc {
    pub name: &'static str,
    pub ty_: &'static TypeDesc,
}

#[doc(hidden)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructDesc {
    pub name: &'static str,
    pub fields: &'static [FieldDesc],
}

#[doc(hidden)]
#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct LibDesc {
    pub fns: &'static [FnDesc],
    pub structs: &'static [StructDesc],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RawDelegate {
    pub call_fn: *mut (),
    pub manage_fn: Option<extern "C" fn(*mut (), i32)>,
}

impl Default for RawDelegate {
    fn default() -> Self {
        Self {
            call_fn: ptr::null_mut(),
            manage_fn: None,
        }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct GeneratorContext<'a> {
    counter: u32,
    add_item_data: *mut (),
    add_item: fn(*mut (), item: &str),
    add_tuple_data: *mut (),
    add_tuple: fn(*mut (), elems: &[Box<str>]) -> Box<str>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> GeneratorContext<'a> {
    #[doc(hidden)]
    pub fn new<F: FnMut(&str), G: FnMut(&[Box<str>]) -> Box<str>>(
        add_item: &'a mut F,
        add_tuple: &'a mut G,
    ) -> Self {
        fn add_item_thunk<F: FnMut(&str)>(ptr: *mut (), item: &str) {
            let f = unsafe { &mut *(ptr as *mut F) };
            f(item);
        }
        fn add_tuple_thunk<G: FnMut(&[Box<str>]) -> Box<str>>(
            ptr: *mut (),
            elems: &[Box<str>],
        ) -> Box<str> {
            let g = unsafe { &mut *(ptr as *mut G) };
            g(elems)
        }
        Self {
            counter: 0,
            add_item_data: add_item as *mut F as *mut (),
            add_item: add_item_thunk::<F>,
            add_tuple_data: add_tuple as *mut G as *mut (),
            add_tuple: add_tuple_thunk::<G>,
            phantom: PhantomData,
        }
    }
    #[doc(hidden)]
    pub fn get_unique_identifier(&mut self, prefix: &str) -> String {
        self.counter += 1;
        format!("{}{}", prefix, self.counter)
    }
    #[doc(hidden)]
    pub fn add_item(&mut self, item: &str) {
        (self.add_item)(self.add_item_data, item)
    }
    #[doc(hidden)]
    pub fn add_tuple(&mut self, elems: &[Box<str>]) -> Box<str> {
        (self.add_tuple)(self.add_tuple_data, elems)
    }
}
