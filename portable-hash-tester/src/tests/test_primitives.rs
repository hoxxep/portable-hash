use portable_hash::BuildPortableHasher;
use crate::{rng, FixtureDB};

macro_rules! suffix {
    ($name:ident, $suffix:expr) => {
        format!("{}_{}", stringify!($name), $suffix)
    };
}

macro_rules! test_integer_type {
    ($f:ident, $int:ident) => {
        {
            $f.test_fixture(suffix!($int, "min"), $int::MIN);
            $f.test_fixture(suffix!($int, "max"), $int::MAX);
            $f.test_fixture(suffix!($int, "0"), 0 as $int);
            $f.test_fixture(suffix!($int, "1"), 1 as $int);
            $f.test_fixture(suffix!($int, "n1"), (0i64 - 1) as $int);
            $f.test_fixture(suffix!($int, "2"), 2 as $int);
            $f.test_fixture(suffix!($int, "54"), 54 as $int);
            $f.test_fixture(suffix!($int, "100"), 100 as $int);
            $f.test_fixture(suffix!($int, "123"), 123 as $int);

            $f.test_fixture(suffix!($int, "123_ref"), &(123 as $int));
            $f.test_fixture(suffix!($int, "123_mut_ref"), &mut (123 as $int));

            let mut seed = 0x98572309512;
            for i in 0..25 {
                let num = rng(&mut seed);
                let test_name = format!("rng_{}_{}", i, num);
                $f.test_fixture(suffix!($int, test_name), (num as u64) as $int);
            }

            let mut array = [0 as $int; 20];
            array[1] = 25;
            for i in 2..array.len() {
                array[i] = (rng(&mut seed) as u64) as $int;
            }

            $f.test_fixture(suffix!($int, "array"), array);
            $f.test_fixture(suffix!($int, "&array"), &array);
            $f.test_fixture(suffix!($int, "&slice"), array.as_slice());
        }
    };
}

pub fn test_primitives(fixtures: &mut FixtureDB<impl BuildPortableHasher>) {
    test_integer_type!(fixtures, u8);
    test_integer_type!(fixtures, u16);
    test_integer_type!(fixtures, u32);
    test_integer_type!(fixtures, u64);
    test_integer_type!(fixtures, u128);
    test_integer_type!(fixtures, usize);
    test_integer_type!(fixtures, i8);
    test_integer_type!(fixtures, i16);
    test_integer_type!(fixtures, i32);
    test_integer_type!(fixtures, i64);
    test_integer_type!(fixtures, i128);
    test_integer_type!(fixtures, isize);

    fixtures.test_fixture("bool_true", true);
    fixtures.test_fixture("bool_false", true);

    fixtures.test_fixture("char_a", 'a');
    fixtures.test_fixture("char_b", 'b');
    fixtures.test_fixture("char_0", '0');
    fixtures.test_fixture("char_smiley", '\u{263A}');

    fixtures.test_fixture("str_empty", "");
    fixtures.test_fixture("str_hello", "Hello, World!");
    fixtures.test_fixture("str_rust", "Rust is awesome!");
    fixtures.test_fixture("str_unicode", "こんにちは世界"); // "Hello, World!" in Japanese
    fixtures.test_fixture("str_special_chars", "!@#$%^&*()_+-={}[]|;:'\",.<>?/~`");
    fixtures.test_fixture("str_long", "This is a long string that is used to test the hashing of longer strings. It should be sufficiently long to ensure that the hasher can handle it without any issues. Let's see how it performs with this input, which is quite verbose and detailed, to ensure we cover all edge cases and scenarios that might arise during hashing.");
    fixtures.test_fixture("str_multiline", "This is a string\nthat spans multiple\nlines.\nIt should be handled correctly by the hasher.\n");
    fixtures.test_fixture("str_with_null", "Hello\0World"); // String with a null character
    fixtures.test_fixture("str_with_emoji", "Hello, World! 😊🌍");
    fixtures.test_fixture("str_with_control_chars", "Hello,\nWorld!\tThis is a test.\r\n");

    fixtures.test_fixture("tuple_empty", ());
    fixtures.test_fixture("tuple_1_int", (42u64,));
    fixtures.test_fixture("tuple_1_str", ("hello",));
    fixtures.test_fixture("tuple_2_int_int", (2u64, 1u64));
    fixtures.test_fixture("tuple_2_int_str", (2u64, "1"));
    fixtures.test_fixture("tuple_2_str_int", ("1", 2u64));
    fixtures.test_fixture("tuple_2_str_str", ("1", "2"));
    fixtures.test_fixture("tuple_3", (1u64, 2u64, 3u64));
    fixtures.test_fixture("tuple_4", (1u64, 2u64, 3u64, 4u64));
    fixtures.test_fixture("tuple_5", (1u64, 2u64, 3u64, 4u64, 5u64));
    fixtures.test_fixture("tuple_6", (1u64, 2u64, 3u64, 4u64, 5u64, 6u64));
    fixtures.test_fixture("tuple_7", (1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64));
    fixtures.test_fixture("tuple_8", (1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64));
    fixtures.test_fixture("tuple_9", (1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64));
    fixtures.test_fixture("tuple_10", (1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64));
    fixtures.test_fixture("tuple_11", (1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64, 11u64));
    fixtures.test_fixture("tuple_12", (1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64, 11u64, 12u64));
}
