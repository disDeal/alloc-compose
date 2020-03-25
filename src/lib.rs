//! Composable allocator structures for plugging together more powerful allocators.
//!
//! The design of composable allocators is inspired by
//! [`std::allocator` Is to Allocation what `std::vector` Is to Vexation][vid] by Andrei
//! Alexandrescu and the [Phobos Standard Library][phobos] of the [D Programming Language][D].
//!
//! [vid]: https://www.youtube.com/watch?v=LIb3L4vKZ7U
//! [phobos]: https://github.com/dlang/phobos
//! [D]: https://dlang.org/

#![no_std]
#![feature(const_generics)]
#![allow(incomplete_features)]

mod fallback_alloc;
mod null_alloc;
mod segregate_alloc;

use alloc::alloc::{AllocErr, AllocInit, AllocRef, Layout, MemoryBlock, ReallocPlacement};
use core::{mem, ptr};

pub use self::{
    fallback_alloc::FallbackAlloc,
    null_alloc::NullAlloc,
    segregate_alloc::SegregateAlloc,
};

/// Trait to determine if a given `MemoryBlock` is owned by an allocator.
pub trait Owns {
    /// Returns if the allocator *owns* the passed `MemoryBlock`.
    fn owns(&self, memory: &MemoryBlock) -> bool;
}

unsafe fn grow<A1: AllocRef, A2: AllocRef>(
    a1: A1,
    a2: A2,
    memory: &mut MemoryBlock,
    new_size: usize,
    placement: ReallocPlacement,
    init: AllocInit,
) -> Result<(), AllocErr> {
    if placement == ReallocPlacement::MayMove {
        let new_layout = Layout::from_size_align_unchecked(new_size, memory.align());
        let new_memory = a2.alloc(new_layout, init)?;
        ptr::copy_nonoverlapping(
            memory.ptr().as_ptr(),
            new_memory.ptr().as_ptr(),
            memory.size(),
        );
        a1.dealloc(mem::replace(memory, new_memory));
        Ok(())
    } else {
        Err(AllocErr)
    }
}

unsafe fn shrink<A1: AllocRef, A2: AllocRef>(
    a1: A1,
    a2: A2,
    memory: &mut MemoryBlock,
    new_size: usize,
    placement: ReallocPlacement,
) -> Result<(), AllocErr> {
    if placement == ReallocPlacement::MayMove {
        let new_layout = Layout::from_size_align_unchecked(new_size, memory.align());
        let new_memory = a2.alloc(new_layout, AllocInit::Uninitialized)?;
        ptr::copy_nonoverlapping(
            memory.ptr().as_ptr(),
            new_memory.ptr().as_ptr(),
            new_memory.size(),
        );
        a1.dealloc(mem::replace(memory, new_memory));
        Ok(())
    } else {
        Err(AllocErr)
    }
}