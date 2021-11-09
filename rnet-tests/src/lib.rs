use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
};

use rnet::net;

rnet::root!();

#[net]
fn do_nothing() {}

#[net]
fn return_42i32() -> i32 {
    42
}

#[net]
fn return_42i64() -> i64 {
    42
}

#[net]
fn return_false_bool() -> bool {
    false
}

#[net]
fn return_true_bool() -> bool {
    true
}

#[net]
fn return_42f32() -> f32 {
    42.0
}

#[net]
fn return_42f64() -> f64 {
    42.0
}

#[net]
fn return_hello_string() -> String {
    "hello".into()
}

#[net]
fn return_fib_vec() -> Vec<i32> {
    vec![0, 1, 1, 2, 3, 5, 8, 13, 21]
}

#[net]
fn return_pow2_hash_set() -> HashSet<i32> {
    vec![1, 2, 4, 8, 16, 32, 64, 128, 256].into_iter().collect()
}

#[net]
fn return_pow2_btree_set() -> BTreeSet<i32> {
    vec![1, 2, 4, 8, 16, 32, 64, 128, 256].into_iter().collect()
}

#[net]
fn return_evens_hash_map() -> HashMap<i32, bool> {
    vec![(0, true), (1, false), (2, true), (3, false)]
        .into_iter()
        .collect()
}

#[net]
fn return_evens_btree_map() -> BTreeMap<i32, bool> {
    vec![(0, true), (1, false), (2, true), (3, false)]
        .into_iter()
        .collect()
}

#[net]
fn return_ones_tuple() -> (i32, u64, bool, f32, f64, String) {
    (1, 1, true, 1.0, 1.0, "one".into())
}

#[net]
fn return_ok_unit() -> Result<(), String> {
    Ok(())
}

#[net]
fn return_err_unit() -> Result<(), String> {
    Err("Err".into())
}

#[net]
fn return_ok_42u32() -> Result<u32, String> {
    Ok(42)
}

#[net]
fn return_err_u32() -> Result<u32, String> {
    Err("Err".into())
}

#[net]
fn return_ok_hello_string() -> Result<String, String> {
    Ok("hello".into())
}

#[net]
fn return_err_string() -> Result<String, String> {
    Err("Err".into())
}

#[net]
fn return_nested_vec() -> Vec<Vec<String>> {
    vec![vec!["foo".into()]]
}

fn check_passed<T: PartialEq + Debug>(a: T, b: T) -> Result<(), String> {
    if a == b {
        Ok(())
    } else {
        Err(format!("{:?} != {:?}", a, b))
    }
}

#[net]
fn pass_42i32(arg: i32) -> Result<(), String> {
    check_passed(arg, 42)
}

#[net]
fn pass_42i64(arg: i64) -> Result<(), String> {
    check_passed(arg, 42)
}

#[net]
fn pass_false_bool(arg: bool) -> Result<(), String> {
    check_passed(arg, false)
}

#[net]
fn pass_true_bool(arg: bool) -> Result<(), String> {
    check_passed(arg, true)
}

#[net]
fn pass_42f32(arg: f32) -> Result<(), String> {
    check_passed(arg, 42.0)
}

#[net]
fn pass_42f64(arg: f64) -> Result<(), String> {
    check_passed(arg, 42.0)
}

#[net]
fn pass_hello_string(arg: String) -> Result<(), String> {
    check_passed(arg, "hello".into())
}

#[net]
fn pass_fib_vec(arg: Vec<i32>) -> Result<(), String> {
    check_passed(arg, vec![0, 1, 1, 2, 3, 5, 8, 13, 21])
}

#[net]
fn pass_pow2_hash_set(arg: HashSet<i32>) -> Result<(), String> {
    check_passed(
        arg,
        vec![1, 2, 4, 8, 16, 32, 64, 128, 256].into_iter().collect(),
    )
}

#[net]
fn pass_pow2_btree_set(arg: BTreeSet<i32>) -> Result<(), String> {
    check_passed(
        arg,
        vec![1, 2, 4, 8, 16, 32, 64, 128, 256].into_iter().collect(),
    )
}

#[net]
fn pass_evens_hash_map(arg: HashMap<i32, bool>) -> Result<(), String> {
    check_passed(
        arg,
        vec![(0, true), (1, false), (2, true), (3, false)]
            .into_iter()
            .collect(),
    )
}

#[net]
fn pass_evens_btree_map(arg: BTreeMap<i32, bool>) -> Result<(), String> {
    check_passed(
        arg,
        vec![(0, true), (1, false), (2, true), (3, false)]
            .into_iter()
            .collect(),
    )
}

#[net]
fn pass_ones_tuple(arg: (i32, u64, bool, f32, f64, String)) -> Result<(), String> {
    check_passed(arg, (1, 1, true, 1.0, 1.0, "one".into()))
}

#[net]
fn pass_nested_vec(arg: Vec<Vec<String>>) -> Result<(), String> {
    check_passed(arg, vec![vec!["foo".into()]])
}
