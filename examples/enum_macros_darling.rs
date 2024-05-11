#[allow(unused_imports)]
use macros::EnumFromDarling;
//proc macro crate

#[allow(unknown_lints)]
#[allow(dead_code)]
#[derive(Debug,EnumFromDarling)]
enum Direction<T> {
    Up(Up<T>),
    Down,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Up<T> {
    speed: T,
}

fn main() {
    let up:Direction<u32> =  Up { speed: 42 }.into();
    println!("{:?}", up);
}
