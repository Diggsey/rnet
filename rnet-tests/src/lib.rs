use rnet::net;

rnet::root!();

#[net]
fn do_nothing() {}

#[net]
fn return_42i32() -> i32 {
    42
}
