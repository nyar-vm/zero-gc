use std::alloc::{Allocator, AllocError, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// Gc pointer
pub struct Gc<T> {
    /// Pointer to the head
    ptr: *mut u64,
    /// Pointer to the data
    bytes: *mut u8,
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

#[derive(Copy, Clone)]
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
// 小型Region（Small Region）：容量固定为2MB，用于放置小于256KB的小对象。
// 中型Region（Medium Region）：容量固定为32MB，用于放置大于等于256KB但小于4MB的对象。
// 大型Region（Large Region）：容量不固定，可以动态变化，但必须为2MB的整数倍，用于放置4MB或以上的大对象。每个大型Region中只会存放一个大对象，这也预示着虽然名字叫作“大型Region”，但它的实际容量完全有可能小于中型Region，最小容量可低至4MB。大型Region在ZGC的实现中是不会被重分配的，因为复制一个大对象的代价非常高昂。

/// 容量固定为1MB，用于放置小于128KB的小对象。
#[derive(Copy, Clone)]
pub struct SmallRegion {
    bytes: [u8; 1 * 1024 * 1024],
}

/// 容量固定为 32MB，用于放置小于4MB的中等对象。
#[derive(Copy, Clone)]
pub struct MediumRegion {
    bytes: [u8; 32 * 1024 * 1024],
}

pub struct OwnedRegion {
    bytes: Vec<u8>,
}

pub struct WorldControl {
    initiating_heap_occupancy_percent: u8,
}