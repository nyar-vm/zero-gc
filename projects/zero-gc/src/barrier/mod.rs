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

pub struct LoadBarrier {
    raw: usize,
}

impl Debug for LoadBarrier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GcPointer")
            .field("raw", &format_args!("0x{:0x}", self.raw))
            .field("pointer", &format_args!("{:p}", self.as_pointer()))
            .field("finalize", &self.is_finalizable())
            .field("remapped", &self.is_remapped())
            .field("marked1", &self.is_marked1())
            .field("marked0", &self.is_marked0())
            .finish()
    }
}

impl Pointer for LoadBarrier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:0x}", self.raw)
    }
}

impl LoadBarrier {
    /// Create a new pointer from a usize.
    pub unsafe fn new(address: *const usize) -> Self {
        Self {
            raw: address as usize,
        }
    }
    pub fn make<T>(value: T) -> Self {
        Self {
            raw: &value as *const T as usize,
        }
    }
    /// Get the pointer as a usize.
    pub fn as_pointer(&self) -> *const usize {
        let masked = self.raw & 0x0000_0000_ffff_ffff;
        masked as *const usize
    }
    /// Cast the pointer to a specific type.
    pub unsafe fn cast<T>(&self) -> &mut T {
        &mut *(self.as_pointer() as *mut T)
    }
    /// The 19th bit of the pointer is used to indicate whether the object is finalizable.
    pub fn is_finalizable(&self) -> bool {
        self.raw & 0x80000 != 0
    }
    pub fn set_finalize(&mut self, finalize: bool) {
        if finalize {
            self.raw |= 0x80000;
        } else {
            self.raw &= !0x80000;
        }
    }
    /// The 20th bit of the pointer is used to indicate whether the object is remapped.
    pub fn is_remapped(&self) -> bool {
        self.raw & 0x40000 != 0
    }
    pub fn set_remapped(&mut self, remapped: bool) {
        if remapped {
            self.raw |= 0x40000;
        } else {
            self.raw &= !0x40000;
        }
    }
    /// The 21th bit of the pointer is used to indicate whether the object is marked.
    pub fn is_marked1(&self) -> bool {
        self.raw & 0x20000 != 0
    }
    pub fn set_marked1(&mut self, marked: bool) {
        if marked {
            self.raw |= 0x20000;
        } else {
            self.raw &= !0x20000;
        }
    }
    /// The 22th bit of the pointer is used to indicate whether the object is marked.
    pub fn is_marked0(&self) -> bool {
        self.raw & 0x10000 != 0
    }
    pub fn set_marked0(&mut self, marked0: bool) {
        if marked0 {
            self.raw |= 0x10000;
        } else {
            self.raw &= !0x10000;
        }
    }
}

#[test]
pub fn test() {
    let a: i64 = -2333;
    println!("{:p}", &a);
    let mut ptr = LoadBarrier::make(a);
    ptr.set_marked0(true);
    ptr.set_marked1(true);
    ptr.set_remapped(true);
    ptr.set_finalize(true);
    println!("{:#?}", ptr);
    let d = unsafe { ptr.cast::<i64>() };
    println!("{:?}", d)
}
