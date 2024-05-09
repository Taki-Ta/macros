use macros::EnumFrom;
//proc macro crate

#[allow(dead_code)]
#[derive(Debug,EnumFrom)]
enum Direction{
    Up(Up),
    Down
}

#[allow(dead_code)]
#[derive(Debug)]
struct Up{
    speed:u32
}

// impl From<Up> for Direction{
//     fn from(value: Up) -> Self{
//         Direction::Up(value)
//     }
// }

fn main(){
    let up=Direction::Up(Up{speed:42});
    println!("{:?}",up);
}



