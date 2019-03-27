// bitflags! {
//     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//     pub struct LoadBarrier: u8 {
//         const Finalizable = 0b00000001;
//         const Remapped = 0b00000010;
//         const Marked1 = 0b00000100;
//         const Marked0 = 0b00001000;
//     }
// }

use std::fmt::{Debug, Formatter, Pointer};
use std::ptr::from_raw_parts;

/// A gc pointer.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GcPointer {
    raw: usize,
}

/// A untyped gc object.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GcObject {
    head: GcPointer,
    size: usize,
}

impl Debug for GcPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pointer")
            .field("raw", &format_args!("0x{:016x}", self.raw))
            .field("pointer", &format_args!("{:p}", self.as_pointer()))
            .field("finalize", &self.is_finalizable())
            .field("remapped", &self.is_remapped())
            .field("marked1", &self.is_marked1())
            .field("marked0", &self.is_marked0())
            .finish()
    }
}

impl Debug for GcObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Object")
            .field("head", &self.head)
            .field("size", &self.size)
            .finish()
    }
}

impl Pointer for GcPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:0x}", self.raw)
    }
}

impl GcPointer {
    /// Create a new gc pointer from a raw pointer.
    pub unsafe fn new(address: *const ()) -> Self {
        Self {
            raw: address as usize,
        }
    }
    /// Create a new gc pointer from the data
    pub fn make<T>(value: &T) -> Self {
        Self {
            raw: value as *const T as usize,
        }
    }
    /// Get the last 42 bits of the pointer.
    pub fn as_pointer(&self) -> *const usize {
        let masked = self.raw & 0x0000_02ff_ffff_ffff;
        masked as *const usize
    }
    /// Cast the pointer to a specific type.
    pub unsafe fn cast<T>(&self) -> &mut T {
        &mut *(self.as_pointer() as *mut T)
    }

    /// The 19th bit of the pointer is used to indicate whether the object is finalizable.
    pub fn is_finalizable(&self) -> bool {
        self.raw & 0x0000_0400_0000_0000 != 0
    }
    /// The 19th bit of the pointer is used to indicate whether the object is finalizable.
    pub fn set_finalize(&mut self, finalize: bool) {
        if finalize {
            self.raw |= 0x0000_0400_0000_0000;
        } else {
            self.raw &= 0xffff_fbff_ffff_ffff;
        }
    }
    /// The 20th bit of the pointer is used to indicate whether the object is remapped.
    pub fn is_remapped(&self) -> bool {
        self.raw & 0x0000_0800_0000_0000 != 0
    }
    /// The 20th bit of the pointer is used to indicate whether the object is remapped.
    pub fn set_remapped(&mut self, remapped: bool) {
        if remapped {
            self.raw |= 0x0000_0800_0000_0000;
        } else {
            self.raw &= 0xffff_f7ff_ffff_ffff;
        }
    }
    /// The 21th bit of the pointer is used to indicate whether the object is marked.
    pub fn is_marked1(&self) -> bool {
        self.raw & 0x0000_1000_0000_0000 != 0
    }
    /// The 21th bit of the pointer is used to indicate whether the object is marked.
    pub fn set_marked1(&mut self, marked: bool) {
        if marked {
            self.raw |= 0x0000_1000_0000_0000;
        } else {
            self.raw &= 0xffff_efff_ffff_ffff;
        }
    }
    /// The 22th bit of the pointer is used to indicate whether the object is marked.
    pub fn is_marked0(&self) -> bool {
        self.raw & 0x0000_2000_0000_0000 != 0
    }
    /// The 22th bit of the pointer is used to indicate whether the object is marked.
    pub fn set_marked0(&mut self, marked0: bool) {
        if marked0 {
            self.raw |= 0x0000_2000_0000_0000;
        } else {
            self.raw &= 0xffff_dfff_ffff_ffff;
        }
    }
}

impl GcObject {
    /// Make an owned object to gc object.
    pub fn make<T>(value: T) -> Self {
        let header = GcPointer::make(&value);
        let size = std::mem::size_of::<T>();
        Self {
            head: header,
            size,
        }
    }
    /// Get the last 42 bits of the pointer.
    pub fn as_pointer(&self) -> *const usize {
        self.head.as_pointer()
    }
    /// Get the size of the object.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = from_raw_parts(self.as_pointer() as *const (), self.size);
            &*ptr
        }
    }
    /// Cast the pointer to a specific type.
    pub unsafe fn cast<T>(self) -> T {
        debug_assert!(self.size == std::mem::size_of::<T>());
        let ptr = self.head.as_pointer() as *const T;
        ptr.read()
    }
}
