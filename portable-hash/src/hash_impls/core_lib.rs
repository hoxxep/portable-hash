use core::{
    // explicitly omitted: alloc::Layout,
    // explicitly omitted: any::TypeId,
    cmp::{Ordering, Reverse},
    convert::Infallible,
    ffi::CStr,
    // explicitly omitted: fmt::Error,
    marker::{PhantomData, PhantomPinned},
    // explicitly omitted: mem::{Discriminant},
    mem::{ManuallyDrop},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Saturating, Wrapping,
    },
    ops::{
        Bound, ControlFlow, Deref, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
    panic::Location,
    pin::Pin,
    // TODO: ptr::NonNull, (can we safely hash this?)
    // TODO: sync::atomic, (issues with Ordering stability, gating by available atomics, and what ordering to choose)
    task::Poll,
    time::Duration,
};
use crate::{PortableHash, PortableHasher};

impl<T: PortableHash> PortableHash for Option<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        match self {
            Some(value) => {
                state.write_u8(1); // Indicate presence
                value.portable_hash(state);
            }
            None => state.write_u8(0), // Indicate absence
        }
    }
}

impl<T: PortableHash, E: PortableHash> PortableHash for Result<T, E> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        match self {
            Ok(value) => {
                state.write_u8(1);  // Indicate success
                value.portable_hash(state);
            }
            Err(err) => {
                state.write_u8(0);  // Indicate error
                err.portable_hash(state);
            },
        }
    }
}

impl PortableHash for Ordering {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        match self {
            Ordering::Less => state.write_u8(1),
            Ordering::Equal => state.write_u8(2),
            Ordering::Greater => state.write_u8(3),
        }
    }
}

impl<T: PortableHash> PortableHash for Reverse<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        self.0.portable_hash(state);
    }
}

impl PortableHash for Infallible {
    fn portable_hash<H: PortableHasher>(&self, _state: &mut H) {
        // do nothing, as Infallible cannot be instantiated.
    }
}

impl PortableHash for CStr {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_bytes(self.to_bytes_with_nul());
    }
}

impl<T: PortableHash> PortableHash for PhantomData<T> {
    fn portable_hash<H: PortableHasher>(&self, _state: &mut H) {
        // do nothing, as PhantomData does not hold any data.
    }
}

impl PortableHash for PhantomPinned {
    fn portable_hash<H: PortableHasher>(&self, _state: &mut H) {
        // do nothing, as PhantomPinned does not hold any data.
    }
}

impl<T: PortableHash> PortableHash for ManuallyDrop<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        self.deref().portable_hash(state);
    }
}

macro_rules! impl_non_zero {
    ($int:ident, $method:ident) => {
        impl PortableHash for $int {
            #[inline]
            fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
                state.$method(self.get());
            }
        }
    };
}

impl_non_zero!(NonZeroI8, write_i8);
impl_non_zero!(NonZeroI16, write_i16);
impl_non_zero!(NonZeroI32, write_i32);
impl_non_zero!(NonZeroI64, write_i64);
impl_non_zero!(NonZeroI128, write_i128);
impl_non_zero!(NonZeroIsize, write_isize);
impl_non_zero!(NonZeroU8, write_u8);
impl_non_zero!(NonZeroU16, write_u16);
impl_non_zero!(NonZeroU32, write_u32);
impl_non_zero!(NonZeroU64, write_u64);
impl_non_zero!(NonZeroU128, write_u128);
impl_non_zero!(NonZeroUsize, write_usize);

impl<T: PortableHash> PortableHash for Saturating<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        self.0.portable_hash(state);
    }
}

impl<T: PortableHash> PortableHash for Wrapping<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        self.0.portable_hash(state);
    }
}

impl<T: PortableHash> PortableHash for Bound<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        match self {
            Bound::Included(value) => {
                state.write_u8(1); // Included
                value.portable_hash(state);
            }
            Bound::Excluded(value) => {
                state.write_u8(2); // Excluded
                value.portable_hash(state);
            }
            Bound::Unbounded => {
                state.write_u8(0); // Unbounded
            }
        }
    }
}

impl<B: PortableHash, C: PortableHash> PortableHash for ControlFlow<B, C> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        match self {
            ControlFlow::Continue(value) => {
                state.write_u8(1); // Continue
                value.portable_hash(state);
            }
            ControlFlow::Break(value) => {
                state.write_u8(0); // Break
                value.portable_hash(state);
            }
        }
    }
}

impl<T: PortableHash> PortableHash for Range<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u8(0);
        self.start.portable_hash(state);
        self.end.portable_hash(state);
    }
}

impl<T: PortableHash> PortableHash for RangeFrom<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u8(1);
        self.start.portable_hash(state);
    }
}

impl PortableHash for RangeFull {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        // TODO(stabilisation): decide whether these Range discriminants should be removed.
        state.write_u8(2);
        // RangeFull has no data to hash
    }
}

impl<T: PortableHash> PortableHash for RangeInclusive<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u8(3);
        self.start().portable_hash(state);
        self.end().portable_hash(state);
    }
}

impl<T: PortableHash> PortableHash for RangeTo<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u8(4);
        self.end.portable_hash(state);
    }
}

impl<T: PortableHash> PortableHash for RangeToInclusive<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u8(5);
        self.end.portable_hash(state);
    }
}

impl PortableHash for Location<'_> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_str(self.file());
        state.write_u32(self.line());
        state.write_u32(self.column());
    }
}

impl<T: Deref<Target = impl PortableHash>> PortableHash for Pin<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        self.deref().portable_hash(state);
    }
}

// macro_rules! impl_atomic {
//     ($int:ident, $method:ident) => {
//         impl PortableHash for atomic::$int {
//             #[inline]
//             fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
//                 // TODO(stabilisation): Remove this implementation as ordering is application-dependent?
//                 state.$method(self.load(atomic::Ordering::SeqCst));
//             }
//         }
//     };
// }
//
// TODO: feature-gate based on availability of atomic types.
// impl_atomic!(AtomicI8, write_i8);
// impl_atomic!(AtomicI16, write_i16);
// impl_atomic!(AtomicI32, write_i32);
// impl_atomic!(AtomicI64, write_i64);
// impl_atomic!(AtomicIsize, write_isize);
// impl_atomic!(AtomicU8, write_u8);
// impl_atomic!(AtomicU16, write_u16);
// impl_atomic!(AtomicU32, write_u32);
// impl_atomic!(AtomicU64, write_u64);
// impl_atomic!(AtomicUsize, write_usize);

// impl PortableHash for atomic::Ordering {
//     // TODO(stabilisation): Consider removing this method if atomic orderings aren't stable.
//     #[inline]
//     fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
//         match self {
//             atomic::Ordering::Relaxed => state.write_u8(0),
//             atomic::Ordering::SeqCst => state.write_u8(1),
//             atomic::Ordering::Acquire => state.write_u8(2),
//             atomic::Ordering::Release => state.write_u8(3),
//             atomic::Ordering::AcqRel => state.write_u8(4),
//             _ => panic!("Currently unsupported atomic ordering. Please raise a github issue."),
//         }
//     }
// }

impl<T: PortableHash> PortableHash for Poll<T> {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        match self {
            Poll::Pending => state.write_u8(0), // Pending
            Poll::Ready(inner) => {
                state.write_u8(1); // Ready
                inner.portable_hash(state);
            },
        }
    }
}

impl PortableHash for Duration {
    #[inline]
    fn portable_hash<H: PortableHasher>(&self, state: &mut H) {
        state.write_u64(self.as_secs());
        state.write_u32(self.subsec_nanos());
    }
}
