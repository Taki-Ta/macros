#[allow(unused_imports)]
use macros::EnumFrom;
//proc macro crate

#[allow(unknown_lints)]
#[allow(dead_code)]
#[derive(Debug,EnumFrom)]
enum Direction<T> {
    Up(Up<T>),
    Down,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Up<T> {
    speed: T,
}

// impl<T> From<Up<T>> for Direction<T>{
//     fn from(value: Up<T>) -> Self{
//         Direction::Up(value)
//     }
// }

fn main() {
    let up:Direction<u32> =  Up { speed: 42 }.into();
    println!("{:?}", up);
}
