use core::{
    // explicitly omitted: alloc::Layout,
    // explicitly omitted: any::TypeId,
    cmp::{Ordering, Reverse},
    // not testable: convert::Infallible,
    // gated to rustc 1.64: ffi::CStr,
    // explicitly omitted: fmt::Error,
    marker::{PhantomData, PhantomPinned},
    // explicitly omitted: mem::{Discriminant},
    mem::{ManuallyDrop},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, /* gated to 1.74 Saturating, */
        Wrapping,
    },
    ops::{
        Bound, ControlFlow, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    },
    // explicitly omitted: panic::Location,
    pin::Pin,
    task::Poll,
    time::Duration,
};

#[rustversion::since(1.64)]
use core::ffi::CStr;

#[rustversion::since(1.74)]
use core::num::Saturating;

use portable_hash::BuildPortableHasher;
use crate::{rng, FixtureDB};

macro_rules! nonzero {
    ($name:ident, $suffix:expr) => {
        format!("nonzero_{}_{}", stringify!($name), $suffix)
    };
}

macro_rules! saturating {
    ($name:ident, $suffix:expr) => {
        format!("saturating_{}_{}", stringify!($name), $suffix)
    };
}

macro_rules! wrapping {
    ($name:ident, $suffix:expr) => {
        format!("wrapping_{}_{}", stringify!($name), $suffix)
    };
}

macro_rules! test_nonzero_types {
    ($f:ident, $int:ident, $nonzero:ident) => {
        {
            $f.test_fixture(nonzero!($int, "max"), $nonzero::new($int::MAX).unwrap());
            $f.test_fixture(nonzero!($int, "1"), $nonzero::new(1).unwrap());
            $f.test_fixture(nonzero!($int, "n1"), $nonzero::new((0 - 1i64) as $int).unwrap());
            $f.test_fixture(nonzero!($int, "2"), $nonzero::new(2).unwrap());
            $f.test_fixture(nonzero!($int, "54"), $nonzero::new(54).unwrap());
            $f.test_fixture(nonzero!($int, "100"), $nonzero::new(100).unwrap());
            $f.test_fixture(nonzero!($int, "123"), $nonzero::new(123).unwrap());
        }
    };
}

macro_rules! test_saturating_type {
    ($f:ident, $int:ident) => {
        {
            $f.test_fixture(saturating!($int, "min"), Saturating::<$int>($int::MIN));
            $f.test_fixture(saturating!($int, "max"), Saturating::<$int>($int::MAX));
            $f.test_fixture(saturating!($int, "0"), Saturating::<$int>(0));
            $f.test_fixture(saturating!($int, "1"), Saturating::<$int>(1));
            $f.test_fixture(saturating!($int, "123"), Saturating::<$int>(123));
        }
    };
}

macro_rules! test_wrapping_types {
    ($f:ident, $int:ident) => {
        {
            $f.test_fixture(wrapping!($int, "min"), Wrapping::<$int>($int::MIN));
            $f.test_fixture(wrapping!($int, "max"), Wrapping::<$int>($int::MAX));
            $f.test_fixture(wrapping!($int, "0"), Wrapping::<$int>(0));
            $f.test_fixture(wrapping!($int, "1"), Wrapping::<$int>(1));
            $f.test_fixture(wrapping!($int, "123"), Wrapping::<$int>(123));
        }
    };
}

pub fn test_core(fixtures: &mut FixtureDB<impl BuildPortableHasher>) {
    let mut seed: u64;

    fixtures.test_fixture("option_none_u32", None::<u32>);
    fixtures.test_fixture("option_none_str", None::<&str>);
    fixtures.test_fixture("option_some_u32", Some(12u32));
    fixtures.test_fixture("option_some_str", Some("1234567890"));

    fixtures.test_fixture("result_ok_u32", Ok::<u32, u32>(12));
    fixtures.test_fixture("result_err_u32", Err::<u32, u32>(12));
    fixtures.test_fixture("result_ok_str", Ok::<&str, &str>("1234567890"));
    fixtures.test_fixture("result_err_str", Err::<&str, &str>("1234567890"));

    fixtures.test_fixture("ordering_less", Ordering::Less);
    fixtures.test_fixture("ordering_equal", Ordering::Equal);
    fixtures.test_fixture("ordering_greater", Ordering::Greater);

    fixtures.test_fixture("reverse_u32", Reverse(1234567890u32));
    fixtures.test_fixture("reverse_str", Reverse("1234567890"));

    // can't be instantiated: fixtures.test_fixture("infallible", std::convert::Infallible);

    // #[rustversion] gated to 1.64
    test_cstr(fixtures);

    fixtures.test_fixture("phantom_data", PhantomData::<u32>);
    fixtures.test_fixture("phantom_pinned", PhantomPinned);

    let manually_drop = ManuallyDrop::new(9812347891u64);
    fixtures.test_fixture("manually_drop_u64", manually_drop);
    ManuallyDrop::into_inner(manually_drop);

    test_nonzero_types!(fixtures, u8, NonZeroU8);
    test_nonzero_types!(fixtures, u16, NonZeroU16);
    test_nonzero_types!(fixtures, u32, NonZeroU32);
    test_nonzero_types!(fixtures, u64, NonZeroU64);
    test_nonzero_types!(fixtures, u128, NonZeroU128);
    test_nonzero_types!(fixtures, usize, NonZeroUsize);
    test_nonzero_types!(fixtures, i8, NonZeroI8);
    test_nonzero_types!(fixtures, i16, NonZeroI16);
    test_nonzero_types!(fixtures, i32, NonZeroI32);
    test_nonzero_types!(fixtures, i64, NonZeroI64);
    test_nonzero_types!(fixtures, i128, NonZeroI128);
    test_nonzero_types!(fixtures, isize, NonZeroIsize);

    test_saturating(fixtures);

    test_wrapping_types!(fixtures, u8);
    test_wrapping_types!(fixtures, u16);
    test_wrapping_types!(fixtures, u32);
    test_wrapping_types!(fixtures, u64);
    test_wrapping_types!(fixtures, u128);
    test_wrapping_types!(fixtures, usize);
    test_wrapping_types!(fixtures, i8);
    test_wrapping_types!(fixtures, i16);
    test_wrapping_types!(fixtures, i32);
    test_wrapping_types!(fixtures, i64);
    test_wrapping_types!(fixtures, i128);
    test_wrapping_types!(fixtures, isize);

    fixtures.test_fixture("bound_unbounded", Bound::<u32>::Unbounded);
    fixtures.test_fixture("bound_included", Bound::<u32>::Included(123));
    fixtures.test_fixture("bound_excluded", Bound::<u32>::Excluded(123));
    fixtures.test_fixture("control_flow_continue", ControlFlow::<u32, u32>::Continue(123));
    fixtures.test_fixture("control_flow_break", ControlFlow::<u32, u32>::Break(123));
    fixtures.test_fixture("range_u32", Range::<u32> { start: 1, end: 10 });
    fixtures.test_fixture("range_from_u32", RangeFrom::<u32> { start: 1 });
    fixtures.test_fixture("range_full", RangeFull);
    fixtures.test_fixture("range_inclusive_u32", RangeInclusive::<u32>::new(1, 10));
    fixtures.test_fixture("range_to_u32", RangeTo::<u32> { end: 10 });
    fixtures.test_fixture("range_to_inclusive_u32", RangeToInclusive::<u32> { end: 10 });

    fixtures.test_fixture("pin_u32", Pin::new(&mut 123u32));
    fixtures.test_fixture("pin_str", Pin::new(&mut "1234567890"));

    fixtures.test_fixture("poll_u32_pending", Poll::<u32>::Pending);
    fixtures.test_fixture("poll_u32_ready_1", Poll::<u32>::Ready(1));
    fixtures.test_fixture("poll_u32_ready_123", Poll::<u32>::Ready(123));
    fixtures.test_fixture("poll_str_pending", Poll::<&str>::Pending);
    fixtures.test_fixture("poll_str_ready", Poll::<&str>::Ready("ready"));

    fixtures.test_fixture("duration_zero", Duration::ZERO);
    fixtures.test_fixture("duration_1ns", Duration::new(0, 1));
    fixtures.test_fixture("duration_1s", Duration::new(1, 0));
    fixtures.test_fixture("duration_12.345678901s", Duration::new(12, 345678901));
    seed = 1893753812972305982;
    fixtures.test_fixture("duration_rand", Duration::new(rng(&mut seed), rng(&mut seed) as u32));
}

#[rustversion::since(1.64)]
fn test_cstr(fixtures: &mut FixtureDB<impl BuildPortableHasher>) {
    fixtures.test_fixture("cstr_empty", CStr::from_bytes_with_nul(b"\0").unwrap());
    fixtures.test_fixture("cstr_hello", CStr::from_bytes_with_nul(b"Hello\0").unwrap());
    fixtures.test_fixture("cstr_world", CStr::from_bytes_with_nul(b"World\0").unwrap());
    fixtures.test_fixture("cstr_smiley", CStr::from_bytes_with_nul("\u{263A}\0".as_bytes()).unwrap());
}

#[rustversion::before(1.64)]
fn test_cstr(_fixtures: &mut FixtureDB<impl BuildPortableHasher>) {}

#[rustversion::since(1.74)]
fn test_saturating(fixtures: &mut FixtureDB<impl BuildPortableHasher>) {
    test_saturating_type!(fixtures, u8);
    test_saturating_type!(fixtures, u16);
    test_saturating_type!(fixtures, u32);
    test_saturating_type!(fixtures, u64);
    test_saturating_type!(fixtures, u128);
    test_saturating_type!(fixtures, usize);
    test_saturating_type!(fixtures, i8);
    test_saturating_type!(fixtures, i16);
    test_saturating_type!(fixtures, i32);
    test_saturating_type!(fixtures, i64);
    test_saturating_type!(fixtures, i128);
    test_saturating_type!(fixtures, isize);
}

#[rustversion::before(1.74)]
fn test_saturating(_fixtures: &mut FixtureDB<impl BuildPortableHasher>) {}
