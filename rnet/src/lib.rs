//! # rnet
//!
//! Easily call into Rust from C# or other .net langauges.
//!
//! ## Usage
//!
//! 1. Use `#[derive(Net)]` on any structs to be shared with .net.
//! 2. Apply the `#[net]` attribute to any standalone functions
//!    which should be callable from .net.
//! 3. Build your rust project as a `cdylib`.
//! 4. Generate C# bindings for your project:
//!    ```
//!    cargo install rnet-gen
//!    rnet-gen "<path to .dll/.so/.dylib>" > "<path to generated file.cs>"
//!    ```
//! 5. Include the C# file in your .net project.
//! 6. Add a link to the compiled rust library to your .net project,
//!    and set it to "Copy if newer".
//! 7. Optional: Configure the above steps to run automatically as
//!    pre-build steps.
#![deny(missing_docs)]
use std::{error::Error, fmt::Display};

pub use rnet_macros::{net, Net};
use types::TypeDesc;

mod borrowed;
mod deferred;
mod delegates;
mod from_net;
mod primitives;
mod std_impls;
mod to_net;
mod tuples;
mod types;
pub use delegates::*;
pub use from_net::{FromNet, FromNetArg, FromNetReturn};
pub use to_net::{ToNet, ToNetArg, ToNetReturn};

fn none_ty() -> Option<Box<str>> {
    None
}

/// This trait is implemented for Rust types which have
/// an equivalent type within .net.
pub unsafe trait Net: 'static {
    #[doc(hidden)]
    const DESC: TypeDesc = TypeDesc {
        net_ty: || Some(Self::gen_type()),
        base_ty: || Some(Self::gen_base_type()),
        raw_ty: || Some(Self::gen_raw_type()),
        marshal_in: None,
        marshal_out: None,
    };
    #[doc(hidden)]
    const TRIVIAL: bool = false;
    #[doc(hidden)]
    type Raw: Default + Copy;
    #[doc(hidden)]
    fn gen_type() -> Box<str>;
    #[doc(hidden)]
    fn gen_base_type() -> Box<str> {
        Self::gen_type()
    }
    #[doc(hidden)]
    fn gen_raw_type() -> Box<str>;
}

/// This error type can be used to capture exceptions from
/// .net when calling a delegate with the appropriate
/// `Result<T, NetException>` type.
#[derive(Debug)]
pub struct NetException(String);

impl Display for NetException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for NetException {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[doc(hidden)]
pub mod hidden {
    use std::alloc::{alloc, dealloc, Layout};

    pub use crate::from_net::{FromNet, FromNetArg};
    pub use crate::to_net::{ToNet, ToNetReturn};
    pub use crate::types::{
        ArgDesc, FieldDesc, FnDesc, GeneratorContext, LibDesc, StructDesc, TypeDesc,
    };
    pub use crate::Net;
    pub use linkme;

    #[linkme::distributed_slice]
    pub static EXPORTED_FNS: [FnDesc] = [..];

    #[linkme::distributed_slice]
    pub static EXPORTED_STRUCTS: [StructDesc] = [..];

    pub const VERSION: usize = 1;

    #[no_mangle]
    pub extern "C" fn rnet_reflect(version: usize, desc: &mut LibDesc) -> bool {
        if version == VERSION {
            *desc = LibDesc {
                fns: &EXPORTED_FNS,
                structs: &EXPORTED_STRUCTS,
            };
            true
        } else {
            false
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn rnet_alloc(size: usize, align: usize) -> *mut u8 {
        alloc(Layout::from_size_align_unchecked(size, align))
    }

    #[no_mangle]
    pub unsafe extern "C" fn rnet_free(ptr: *mut u8, size: usize, align: usize) {
        dealloc(ptr, Layout::from_size_align_unchecked(size, align))
    }
}
