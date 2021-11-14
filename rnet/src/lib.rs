//! # rnet
//!
//! Easily call into Rust from C# or other .net langauges.
//!
//! ## Usage
//!
//! 1. Add `rnet::root!();` to your crate.
//! 2. Use `#[derive(Net)]` on any structs to be shared with .net.
//! 3. Apply the `#[net]` attribute to any standalone functions
//!    which should be callable from .net.
//! 4. Build your rust project as a `cdylib`.
//! 5. Generate C# bindings for your project:
//!    ```
//!    cargo install rnet-gen
//!    rnet-gen "<path to .dll/.so/.dylib>" > "<path to generated file.cs>"
//!    ```
//! 6. Include the C# file in your .net project.
//! 7. Add a link to the compiled rust library to your .net project,
//!    and set it to "Copy if newer".
//! 8. Optional: Configure the above steps to run automatically as
//!    pre-build steps.
#![deny(missing_docs)]
use std::{error::Error, fmt::Display};

use hidden::GeneratorContext;
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
#[cfg(feature = "uuid")]
mod uuid_impls;
#[cfg(feature = "chrono")]
mod chrono_impls;

pub use delegates::*;
pub use from_net::{FromNet, FromNetArg, FromNetReturn};
pub use to_net::{ToNet, ToNetArg, ToNetReturn};

fn none_ty(_ctx: &mut GeneratorContext) -> Option<Box<str>> {
    None
}

/// This trait is implemented for Rust types which have
/// an equivalent type within .net.
pub unsafe trait Net: 'static {
    #[doc(hidden)]
    const DESC: &'static TypeDesc = &TypeDesc {
        net_ty: |ctx| Some(Self::gen_type(ctx)),
        base_ty: |ctx| Some(Self::gen_base_type(ctx)),
        raw_ty: |ctx| Some(Self::gen_raw_type(ctx)),
        marshal_in: None,
        marshal_out: None,
    };
    #[doc(hidden)]
    const TRIVIAL: bool = false;
    #[doc(hidden)]
    type Raw: Default + Copy;
    #[doc(hidden)]
    fn gen_type(ctx: &mut GeneratorContext) -> Box<str>;
    #[doc(hidden)]
    fn gen_base_type(ctx: &mut GeneratorContext) -> Box<str> {
        Self::gen_type(ctx)
    }
    #[doc(hidden)]
    fn gen_raw_type(ctx: &mut GeneratorContext) -> Box<str>;
    #[doc(hidden)]
    fn is_nullable(_ctx: &mut GeneratorContext) -> bool;
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

/// Must be called by the `cdylib` crate.
#[macro_export]
macro_rules! root {
    () => {
        const _: () = {
            #[doc(hidden)]
            #[no_mangle]
            pub extern "C" fn rnet_reflect(
                version: usize,
                desc: &mut $crate::hidden::LibDesc,
            ) -> bool {
                $crate::hidden::rnet_reflect(version, desc)
            }

            #[doc(hidden)]
            #[no_mangle]
            pub unsafe extern "C" fn rnet_alloc(size: usize, align: usize) -> *mut u8 {
                $crate::hidden::rnet_alloc(size, align)
            }

            #[doc(hidden)]
            #[no_mangle]
            pub unsafe extern "C" fn rnet_free(ptr: *mut u8, size: usize, align: usize) {
                $crate::hidden::rnet_free(ptr, size, align)
            }
        };
    };
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

    #[doc(hidden)]
    pub fn rnet_reflect(version: usize, desc: &mut LibDesc) -> bool {
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

    #[doc(hidden)]
    pub unsafe fn rnet_alloc(size: usize, align: usize) -> *mut u8 {
        alloc(Layout::from_size_align_unchecked(size, align))
    }

    #[doc(hidden)]
    pub unsafe fn rnet_free(ptr: *mut u8, size: usize, align: usize) {
        dealloc(ptr, Layout::from_size_align_unchecked(size, align))
    }
}
