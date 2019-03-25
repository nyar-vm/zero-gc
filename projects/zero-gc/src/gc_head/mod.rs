use std::alloc::{Allocator, AllocError, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// Gc pointer
pub struct Gc<T> {
    /// Pointer to the head
    ptr: usize,
    /// Pointer to the data
    data: *mut u8,
    /// Phantom data
    typing: PhantomData<T>,
}


pub trait Region {
    fn allocate(&mut self, size: usize) -> *mut u8;
    fn free(&mut self, ptr: *mut u8);
}

impl Region for SmallRegion {
    fn allocate(&mut self, size: usize) -> *mut u8 {
        todo!()
    }

    fn free(&mut self, ptr: *mut u8) {
        todo!()
    }
}

pub struct TheWorld {}

pub union TheRegion {
    small: SmallRegion,
    medium: MediumRegion,
}

unsafe impl Allocator for TheWorld {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        todo!()
    }
}


/// 容量固定为1MB，用于放置小于128KB的小对象。
pub struct SmallRegion {
    bytes: [u8; 1 * 1024 * 1024],
}

/// 容量固定为16MB，用于放置小于4MB的中等对象。
pub struct MediumRegion {
    bytes: [u8; 16 * 1024 * 1024],
}