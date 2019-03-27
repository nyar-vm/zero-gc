use std::marker::PhantomData;


use crate::GcPointer;

/// A typed gc object.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Gc<T> {
    /// Pointer to the head
    head: GcPointer,
    /// Pointer to the data
    size: usize,
    /// Phantom data
    typing: PhantomData<T>,
}

impl<T> Gc<T> {
    /// Create a new gc object from the data
    pub fn new(value: T) -> Self {
        Self {
            head: GcPointer::make(&value),
            size: std::mem::size_of::<T>(),
            typing: PhantomData::default(),
        }
    }
}

/// The world.
#[derive(Copy, Clone, Debug)]
pub struct TheWorld {}

/// The world control.
#[derive(Copy, Clone, Debug)]
pub struct TheWorldControl {
    /// The world
    pub initiating_heap_occupancy_percent: u8,
}