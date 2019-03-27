#![feature(allocator_api, slice_from_ptr_range)]
#![feature(ptr_metadata)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
mod gc_head;
mod barrier;

pub use crate::errors::{GcError, GcResult};

pub use crate::barrier::{GcObject, GcPointer};
pub use crate::gc_head::{Gc, TheWorld, TheWorldControl};