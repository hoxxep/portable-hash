extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet, LinkedList, VecDeque};
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;

#[rustversion::since(1.64)]
use alloc::ffi::CString;

use portable_hash::BuildPortableHasher;
use crate::{rng, FixtureDB};

pub fn test_alloc(fixtures: &mut FixtureDB<impl BuildPortableHasher>) {
    // String — delegates to str, should match &str fixture hashes
    fixtures.test_fixture("string_empty", String::new());
    fixtures.test_fixture("string_hello", String::from("Hello, World!"));
    fixtures.test_fixture("string_unicode", String::from("こんにちは世界"));

    // CString — delegates to CStr
    test_cstring(fixtures);

    // Vec<T> — delegates to &[T]
    fixtures.test_fixture("vec_u32_empty", Vec::<u32>::new());
    fixtures.test_fixture("vec_u32_3", vec![1u32, 2, 3]);
    fixtures.test_fixture("vec_str_3", vec!["hello", "world", "test"]);
    fixtures.test_fixture("vec_nested", vec![vec![1u32, 2], vec![3, 4]]);

    let mut seed: u64 = 0x7a3b9e1d4c5f2a08;
    let vec_u32_10: Vec<u32> = (0..10).map(|_| rng(&mut seed) as u32).collect();
    fixtures.test_fixture("vec_u32_10", vec_u32_10.clone());

    // VecDeque<T> — hashes as length prefix + elements, same as Vec/slice
    fixtures.test_fixture("vecdeque_u32_empty", VecDeque::<u32>::new());
    let mut vd = VecDeque::new();
    vd.push_back(1u32);
    vd.push_back(2);
    vd.push_back(3);
    fixtures.test_fixture("vecdeque_u32_3", vd);

    let mut vd_10 = VecDeque::new();
    for &v in &vec_u32_10 {
        vd_10.push_back(v);
    }
    fixtures.test_fixture("vecdeque_u32_10", vd_10);

    // LinkedList<T> — hashes as length prefix + elements
    fixtures.test_fixture("linkedlist_u32_empty", LinkedList::<u32>::new());
    let mut ll = LinkedList::new();
    ll.push_back(1u32);
    ll.push_back(2);
    ll.push_back(3);
    fixtures.test_fixture("linkedlist_u32_3", ll);

    // BTreeMap<K, V> — length prefix + (key, value) pairs in sorted order
    fixtures.test_fixture("btreemap_empty", BTreeMap::<u32, u32>::new());

    let mut map1 = BTreeMap::new();
    map1.insert(1u32, 10u32);
    fixtures.test_fixture("btreemap_u32_u32_1", map1);

    let mut map3 = BTreeMap::new();
    map3.insert(1u32, 10u32);
    map3.insert(2, 20);
    map3.insert(3, 30);
    fixtures.test_fixture("btreemap_u32_u32_3", map3);

    let mut map_str = BTreeMap::new();
    map_str.insert("a", 1u32);
    map_str.insert("b", 2);
    map_str.insert("c", 3);
    fixtures.test_fixture("btreemap_str_u32_3", map_str);

    // BTreeSet<K> — length prefix + elements in sorted order
    fixtures.test_fixture("btreeset_empty", BTreeSet::<u32>::new());

    let mut set3 = BTreeSet::new();
    set3.insert(1u32);
    set3.insert(2);
    set3.insert(3);
    fixtures.test_fixture("btreeset_u32_3", set3);

    let mut set_str = BTreeSet::new();
    set_str.insert("a");
    set_str.insert("b");
    set_str.insert("c");
    fixtures.test_fixture("btreeset_str_3", set_str);

    // Box<T> — transparent, hashes as inner value
    fixtures.test_fixture("box_u32", Box::new(123u32));
    fixtures.test_fixture("box_str", Box::<str>::from("Hello, World!"));
    fixtures.test_fixture("box_vec", Box::new(vec![1u32, 2, 3]));

    // Rc<T> — transparent, hashes as inner value
    fixtures.test_fixture("rc_u32", Rc::new(123u32));
    fixtures.test_fixture("rc_str", Rc::<str>::from("Hello, World!"));

    // Arc<T> — transparent, hashes as inner value
    fixtures.test_fixture("arc_u32", Arc::new(123u32));
    fixtures.test_fixture("arc_str", Arc::<str>::from("Hello, World!"));

    // Cow<T> — hashes as the borrowed value
    fixtures.test_fixture("cow_borrowed_str", Cow::Borrowed("Hello, World!"));
    fixtures.test_fixture("cow_owned_str", Cow::<str>::Owned(String::from("Hello, World!")));
    fixtures.test_fixture("cow_borrowed_slice", Cow::Borrowed(&[1u32, 2, 3][..]));
    fixtures.test_fixture("cow_owned_vec", Cow::<[u32]>::Owned(vec![1u32, 2, 3]));
}

#[rustversion::since(1.64)]
fn test_cstring(fixtures: &mut FixtureDB<impl BuildPortableHasher>) {
    fixtures.test_fixture("cstring_empty", CString::new("").unwrap());
    fixtures.test_fixture("cstring_hello", CString::new("Hello").unwrap());
    fixtures.test_fixture("cstring_world", CString::new("World").unwrap());
}

#[rustversion::before(1.64)]
fn test_cstring(_fixtures: &mut FixtureDB<impl BuildPortableHasher>) {}
