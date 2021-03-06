//! Structures to collect allocator statistics.
//!
//! Please see the [`Proxy`] documentation for examples.
//!
//! [`Proxy`]: crate::Proxy

use crate::CallbackRef;
use core::{
    alloc::{AllocErr, AllocInit, Layout, MemoryBlock, ReallocPlacement},
    cell::Cell,
    ptr::NonNull,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
};

#[repr(usize)]
#[derive(Copy, Clone, PartialEq)]
enum Stat {
    Allocs = 0,
    Deallocs = 1,
    Grows = 2,
    Shrinks = 3,
    Owns = 4,
}
const STAT_COUNT: usize = 4;

/// A primitive counter for collectiong statistics.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Counter {
    stats: [Cell<u64>; STAT_COUNT],
}

impl PartialEq<AtomicCounter> for Counter {
    fn eq(&self, other: &AtomicCounter) -> bool {
        self.stats
            .iter()
            .zip(other.stats.iter())
            .all(|(lhs, rhs)| lhs.get() == rhs.load(Relaxed))
    }
}

impl Counter {
    fn increment_stat(&self, stat: Stat, additional: u64) {
        self.stats[stat as usize].set(self.stats[stat as usize].get() + additional)
    }
    fn get(&self, stat: Stat) -> u64 {
        self.stats[stat as usize].get()
    }
}

/// An atomic counter for collectiong statistics which can be shared between threads.
#[derive(Debug, Default)]
pub struct AtomicCounter {
    stats: [AtomicU64; STAT_COUNT],
}

impl PartialEq for AtomicCounter {
    fn eq(&self, other: &Self) -> bool {
        self.stats
            .iter()
            .zip(other.stats.iter())
            .all(|(lhs, rhs)| lhs.load(Relaxed) == rhs.load(Relaxed))
    }
}

impl PartialEq<Counter> for AtomicCounter {
    fn eq(&self, other: &Counter) -> bool {
        self.stats
            .iter()
            .zip(other.stats.iter())
            .all(|(lhs, rhs)| lhs.load(Relaxed) == rhs.get())
    }
}

impl AtomicCounter {
    fn increment_stat(&self, stat: Stat, additional: u64) {
        self.stats[stat as usize].fetch_add(additional, Relaxed);
    }
    fn get(&self, stat: Stat) -> u64 {
        self.stats[stat as usize].load(Relaxed)
    }
}

macro_rules! impl_callback_ref {
    ($tt:tt) => {
        impl $tt {
            /// Returns the number of `alloc` calls.
            #[inline]
            pub fn num_allocs(&self) -> u64 {
                self.get(Stat::Allocs)
            }
            /// Returns the number of `dealloc` calls.
            #[inline]
            pub fn num_deallocs(&self) -> u64 {
                self.get(Stat::Deallocs)
            }

            /// Returns the number of `grow` calls.
            #[inline]
            pub fn num_grows(&self) -> u64 {
                self.get(Stat::Grows)
            }

            /// Returns the number of `shrink` calls.
            #[inline]
            pub fn num_shrinks(&self) -> u64 {
                self.get(Stat::Shrinks)
            }

            /// Returns the number of `owns` calls.
            #[inline]
            pub fn num_owns(&self) -> u64 {
                self.get(Stat::Owns)
            }
        }

        unsafe impl CallbackRef for $tt {
            #[inline]
            fn alloc(
                &self,
                _layout: Layout,
                _init: AllocInit,
                _result: Result<MemoryBlock, AllocErr>,
            ) {
                self.increment_stat(Stat::Allocs, 1)
            }

            #[inline]
            fn dealloc(&self, _ptr: NonNull<u8>, _layout: Layout) {
                self.increment_stat(Stat::Deallocs, 1);
            }

            fn grow(
                &self,
                _ptr: NonNull<u8>,
                _layout: Layout,
                _new_size: usize,
                _placement: ReallocPlacement,
                _init: AllocInit,
                _result: Result<MemoryBlock, AllocErr>,
            ) {
                self.increment_stat(Stat::Grows, 1)
            }

            #[inline]
            fn shrink(
                &self,
                _ptr: NonNull<u8>,
                _layout: Layout,
                _new_size: usize,
                _placement: ReallocPlacement,
                _result: Result<MemoryBlock, AllocErr>,
            ) {
                self.increment_stat(Stat::Shrinks, 1)
            }

            #[inline]
            fn owns(&self, _success: bool) {
                self.increment_stat(Stat::Owns, 1)
            }
        }
    };
}

impl_callback_ref!(Counter);
impl_callback_ref!(AtomicCounter);

#[repr(usize)]
#[derive(Copy, Clone, PartialEq)]
enum FilteredStat {
    AllocsUninitializedOk = 0,
    AllocsUninitializedErr = 1,
    AllocsZeroedOk = 2,
    AllocsZeroedErr = 3,
    Deallocs = 4,
    GrowsMayMoveUninitializedOk = 5,
    GrowsMayMoveUninitializedErr = 6,
    GrowsInPlaceUninitializedOk = 7,
    GrowsInPlaceUninitializedErr = 8,
    GrowsMayMoveZeroedOk = 9,
    GrowsMayMoveZeroedErr = 10,
    GrowsInPlaceZeroedOk = 11,
    GrowsInPlaceZeroedErr = 12,
    ShrinksMayMoveOk = 13,
    ShrinksMayMoveErr = 14,
    ShrinksInPlaceOk = 15,
    ShrinksInPlaceErr = 16,
    OwnsTrue = 17,
    OwnsFalse = 18,
}
const FILTERED_STAT_COUNT: usize = 19;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AllocInitFilter {
    None,
    Uninitialized,
    Zeroed,
}

impl From<AllocInit> for AllocInitFilter {
    fn from(init: AllocInit) -> Self {
        match init {
            AllocInit::Uninitialized => Self::Uninitialized,
            AllocInit::Zeroed => Self::Zeroed,
        }
    }
}

impl From<Option<AllocInit>> for AllocInitFilter {
    fn from(init: Option<AllocInit>) -> Self {
        if let Some(init) = init {
            Self::from(init)
        } else {
            Self::None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReallocPlacementFilter {
    None,
    MayMove,
    InPlace,
}

impl From<ReallocPlacement> for ReallocPlacementFilter {
    fn from(placement: ReallocPlacement) -> Self {
        match placement {
            ReallocPlacement::MayMove => Self::MayMove,
            ReallocPlacement::InPlace => Self::InPlace,
        }
    }
}

impl From<Option<ReallocPlacement>> for ReallocPlacementFilter {
    fn from(placement: Option<ReallocPlacement>) -> Self {
        if let Some(placement) = placement {
            Self::from(placement)
        } else {
            Self::None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResultFilter {
    None,
    Ok,
    Err,
}

/// A counter for collectiong and filtering statistics.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct FilteredCounter {
    stats: [Cell<u64>; FILTERED_STAT_COUNT],
}

impl FilteredCounter {
    fn increment_stat(&self, stat: FilteredStat, additional: u64) {
        self.stats[stat as usize].set(self.stats[stat as usize].get() + additional)
    }
    fn get(&self, stat: FilteredStat) -> u64 {
        self.stats[stat as usize].get()
    }
}

impl PartialEq<FilteredAtomicCounter> for FilteredCounter {
    fn eq(&self, other: &FilteredAtomicCounter) -> bool {
        self.stats
            .iter()
            .zip(other.stats.iter())
            .all(|(lhs, rhs)| lhs.get() == rhs.load(Relaxed))
    }
}

/// An atomic counter for collectiong and filtering statistics which can be shared between threads.
#[derive(Debug, Default)]
pub struct FilteredAtomicCounter {
    stats: [AtomicU64; FILTERED_STAT_COUNT],
}

impl FilteredAtomicCounter {
    fn increment_stat(&self, stat: FilteredStat, additional: u64) {
        self.stats[stat as usize].fetch_add(additional, Relaxed);
    }
    fn get(&self, stat: FilteredStat) -> u64 {
        self.stats[stat as usize].load(Relaxed)
    }
}

impl PartialEq for FilteredAtomicCounter {
    fn eq(&self, other: &Self) -> bool {
        self.stats
            .iter()
            .zip(other.stats.iter())
            .all(|(lhs, rhs)| lhs.load(Relaxed) == rhs.load(Relaxed))
    }
}

impl PartialEq<FilteredCounter> for FilteredAtomicCounter {
    fn eq(&self, other: &FilteredCounter) -> bool {
        self.stats
            .iter()
            .zip(other.stats.iter())
            .all(|(lhs, rhs)| lhs.load(Relaxed) == rhs.get())
    }
}

macro_rules! impl_filtered_callback_ref {
    ($tt:tt) => {
        impl $tt {
            /// Returns the total number of `alloc` calls.
            #[inline]
            pub fn num_allocs(&self) -> u64 {
                self.num_allocs_filter(AllocInitFilter::None, ResultFilter::None)
            }

            /// Returns the filtered number of `alloc` calls.
            pub fn num_allocs_filter(
                &self,
                init: impl Into<AllocInitFilter>,
                result: ResultFilter,
            ) -> u64 {
                match (init.into(), result) {
                    (AllocInitFilter::Uninitialized, ResultFilter::Ok) => {
                        self.get(FilteredStat::AllocsUninitializedOk)
                    }
                    (AllocInitFilter::Uninitialized, ResultFilter::Err) => {
                        self.get(FilteredStat::AllocsUninitializedErr)
                    }
                    (AllocInitFilter::Zeroed, ResultFilter::Ok) => {
                        self.get(FilteredStat::AllocsZeroedOk)
                    }
                    (AllocInitFilter::Zeroed, ResultFilter::Err) => {
                        self.get(FilteredStat::AllocsZeroedErr)
                    }
                    (AllocInitFilter::None, _) => {
                        self.num_allocs_filter(AllocInitFilter::Uninitialized, result)
                            + self.num_allocs_filter(AllocInitFilter::Zeroed, result)
                    }
                    (i, ResultFilter::None) => {
                        self.num_allocs_filter(i, ResultFilter::Ok)
                            + self.num_allocs_filter(i, ResultFilter::Err)
                    }
                }
            }

            /// Returns the total number of `dealloc` calls.
            #[inline]
            pub fn num_deallocs(&self) -> u64 {
                self.get(FilteredStat::Deallocs)
            }

            /// Returns the total number of `grow` calls.
            #[inline]
            pub fn num_grows(&self) -> u64 {
                self.num_grows_filter(
                    ReallocPlacementFilter::None,
                    AllocInitFilter::None,
                    ResultFilter::None,
                )
            }

            /// Returns the filtered number of `grow` calls.
            pub fn num_grows_filter(
                &self,
                placement: impl Into<ReallocPlacementFilter>,
                init: impl Into<AllocInitFilter>,
                result: ResultFilter,
            ) -> u64 {
                match (placement.into(), init.into(), result) {
                    (
                        ReallocPlacementFilter::MayMove,
                        AllocInitFilter::Uninitialized,
                        ResultFilter::Ok,
                    ) => self.get(FilteredStat::GrowsMayMoveUninitializedOk),
                    (
                        ReallocPlacementFilter::MayMove,
                        AllocInitFilter::Uninitialized,
                        ResultFilter::Err,
                    ) => self.get(FilteredStat::GrowsMayMoveUninitializedErr),
                    (
                        ReallocPlacementFilter::MayMove,
                        AllocInitFilter::Zeroed,
                        ResultFilter::Ok,
                    ) => self.get(FilteredStat::GrowsMayMoveZeroedOk),
                    (
                        ReallocPlacementFilter::MayMove,
                        AllocInitFilter::Zeroed,
                        ResultFilter::Err,
                    ) => self.get(FilteredStat::GrowsMayMoveZeroedErr),
                    (
                        ReallocPlacementFilter::InPlace,
                        AllocInitFilter::Uninitialized,
                        ResultFilter::Ok,
                    ) => self.get(FilteredStat::GrowsInPlaceUninitializedOk),
                    (
                        ReallocPlacementFilter::InPlace,
                        AllocInitFilter::Uninitialized,
                        ResultFilter::Err,
                    ) => self.get(FilteredStat::GrowsInPlaceUninitializedErr),
                    (
                        ReallocPlacementFilter::InPlace,
                        AllocInitFilter::Zeroed,
                        ResultFilter::Ok,
                    ) => self.get(FilteredStat::GrowsInPlaceZeroedOk),
                    (
                        ReallocPlacementFilter::InPlace,
                        AllocInitFilter::Zeroed,
                        ResultFilter::Err,
                    ) => self.get(FilteredStat::GrowsInPlaceZeroedErr),
                    (ReallocPlacementFilter::None, i, _) => {
                        self.num_grows_filter(ReallocPlacementFilter::MayMove, i, result)
                            + self.num_grows_filter(ReallocPlacementFilter::InPlace, i, result)
                    }
                    (p, AllocInitFilter::None, _) => {
                        self.num_grows_filter(p, AllocInitFilter::Uninitialized, result)
                            + self.num_grows_filter(p, AllocInitFilter::Zeroed, result)
                    }
                    (p, i, ResultFilter::None) => {
                        self.num_grows_filter(p, i, ResultFilter::Ok)
                            + self.num_grows_filter(p, i, ResultFilter::Err)
                    }
                }
            }

            /// Returns the total number of `shrink` calls.
            #[inline]
            pub fn num_shrinks(&self) -> u64 {
                self.num_shrinks_filter(ReallocPlacementFilter::None, ResultFilter::None)
            }

            /// Returns the filtered number of `shrink` calls.
            pub fn num_shrinks_filter(
                &self,
                placement: impl Into<ReallocPlacementFilter>,
                result: ResultFilter,
            ) -> u64 {
                match (placement.into(), result) {
                    (ReallocPlacementFilter::MayMove, ResultFilter::Ok) => {
                        self.get(FilteredStat::ShrinksMayMoveOk)
                    }
                    (ReallocPlacementFilter::MayMove, ResultFilter::Err) => {
                        self.get(FilteredStat::ShrinksMayMoveErr)
                    }
                    (ReallocPlacementFilter::InPlace, ResultFilter::Ok) => {
                        self.get(FilteredStat::ShrinksInPlaceOk)
                    }
                    (ReallocPlacementFilter::InPlace, ResultFilter::Err) => {
                        self.get(FilteredStat::ShrinksInPlaceErr)
                    }
                    (ReallocPlacementFilter::None, _) => {
                        self.num_shrinks_filter(ReallocPlacementFilter::MayMove, result)
                            + self.num_shrinks_filter(ReallocPlacementFilter::InPlace, result)
                    }
                    (p, ResultFilter::None) => {
                        self.num_shrinks_filter(p, ResultFilter::Ok)
                            + self.num_shrinks_filter(p, ResultFilter::Err)
                    }
                }
            }

            /// Returns the total number of `owns` calls.
            #[inline]
            pub fn num_owns(&self) -> u64 {
                self.num_owns_filter(true) + self.num_owns_filter(false)
            }

            /// Returns the filtered number of `owns` calls.
            pub fn num_owns_filter(&self, success: bool) -> u64 {
                if success {
                    self.get(FilteredStat::OwnsTrue)
                } else {
                    self.get(FilteredStat::OwnsFalse)
                }
            }
        }

        unsafe impl CallbackRef for $tt {
            #[inline]
            fn alloc(
                &self,
                _layout: Layout,
                init: AllocInit,
                result: Result<MemoryBlock, AllocErr>,
            ) {
                match (init, result.is_ok()) {
                    (AllocInit::Uninitialized, true) => {
                        self.increment_stat(FilteredStat::AllocsUninitializedOk, 1)
                    }
                    (AllocInit::Uninitialized, false) => {
                        self.increment_stat(FilteredStat::AllocsUninitializedErr, 1)
                    }
                    (AllocInit::Zeroed, true) => {
                        self.increment_stat(FilteredStat::AllocsZeroedOk, 1)
                    }
                    (AllocInit::Zeroed, false) => {
                        self.increment_stat(FilteredStat::AllocsZeroedErr, 1)
                    }
                }
            }

            #[inline]
            fn dealloc(&self, _ptr: NonNull<u8>, _layout: Layout) {
                self.increment_stat(FilteredStat::Deallocs, 1);
            }

            fn grow(
                &self,
                _ptr: NonNull<u8>,
                _layout: Layout,
                _new_size: usize,
                placement: ReallocPlacement,
                init: AllocInit,
                result: Result<MemoryBlock, AllocErr>,
            ) {
                match (placement, init, result.is_ok()) {
                    (ReallocPlacement::MayMove, AllocInit::Uninitialized, true) => {
                        self.increment_stat(FilteredStat::GrowsMayMoveUninitializedOk, 1)
                    }
                    (ReallocPlacement::MayMove, AllocInit::Uninitialized, false) => {
                        self.increment_stat(FilteredStat::GrowsMayMoveUninitializedErr, 1)
                    }
                    (ReallocPlacement::MayMove, AllocInit::Zeroed, true) => {
                        self.increment_stat(FilteredStat::GrowsMayMoveZeroedOk, 1)
                    }
                    (ReallocPlacement::MayMove, AllocInit::Zeroed, false) => {
                        self.increment_stat(FilteredStat::GrowsMayMoveZeroedErr, 1)
                    }
                    (ReallocPlacement::InPlace, AllocInit::Uninitialized, true) => {
                        self.increment_stat(FilteredStat::GrowsInPlaceUninitializedOk, 1)
                    }
                    (ReallocPlacement::InPlace, AllocInit::Uninitialized, false) => {
                        self.increment_stat(FilteredStat::GrowsInPlaceUninitializedErr, 1)
                    }
                    (ReallocPlacement::InPlace, AllocInit::Zeroed, true) => {
                        self.increment_stat(FilteredStat::GrowsInPlaceZeroedOk, 1)
                    }
                    (ReallocPlacement::InPlace, AllocInit::Zeroed, false) => {
                        self.increment_stat(FilteredStat::GrowsInPlaceZeroedErr, 1)
                    }
                }
            }

            #[inline]
            fn shrink(
                &self,
                _ptr: NonNull<u8>,
                _layout: Layout,
                _new_size: usize,
                placement: ReallocPlacement,
                result: Result<MemoryBlock, AllocErr>,
            ) {
                match (placement, result.is_ok()) {
                    (ReallocPlacement::MayMove, true) => {
                        self.increment_stat(FilteredStat::ShrinksMayMoveOk, 1)
                    }
                    (ReallocPlacement::MayMove, false) => {
                        self.increment_stat(FilteredStat::ShrinksMayMoveErr, 1)
                    }
                    (ReallocPlacement::InPlace, true) => {
                        self.increment_stat(FilteredStat::ShrinksInPlaceOk, 1)
                    }
                    (ReallocPlacement::InPlace, false) => {
                        self.increment_stat(FilteredStat::ShrinksInPlaceErr, 1)
                    }
                }
            }

            #[inline]
            fn owns(&self, success: bool) {
                if success {
                    self.increment_stat(FilteredStat::OwnsTrue, 1)
                } else {
                    self.increment_stat(FilteredStat::OwnsFalse, 1)
                }
            }
        }
    };
}
impl_filtered_callback_ref!(FilteredCounter);
impl_filtered_callback_ref!(FilteredAtomicCounter);
