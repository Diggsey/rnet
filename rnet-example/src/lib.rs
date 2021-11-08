use std::collections::HashMap;

use rnet::{net, Delegate0, Delegate1, Net};

#[derive(Net)]
pub struct Foo {
    field0: String,
    field1: Vec<bool>,
    field2: Delegate1<Foo, Foo>,
    field3: HashMap<String, bool>,
}

#[net]
pub fn hello(name: &str) -> Result<(), String> {
    println!("Hello, {}!", name);
    Err("Oh no!".into())
}

#[net]
pub fn hello_many(names: &[String]) {
    for name in names {
        println!("Hello, {}!", name);
    }
}

#[net]
pub fn is_even(value: i32) -> (bool, bool) {
    (value % 2 == 0, false)
}

#[net]
pub fn str_to_bytes(value: &str) -> Vec<u8> {
    value.as_bytes().to_vec()
}

#[net]
pub fn test(arg: Delegate1<Foo, bool>) -> Foo {
    arg.call(true)
}

#[net]
pub fn test2(arg: Delegate0<Vec<Vec<Vec<String>>>>) -> Vec<Vec<Vec<String>>> {
    arg.call()
}
