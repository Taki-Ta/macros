use anyhow::Result;
use std::boxed;

fn main() -> Result<()> {
    let v = my_vec! {1,2,3};
    let c = my_vec! {2;3};
    let e = my_vec! {1,2,3,};
    println!("{:?}", v);
    println!("{:?}", c);
    println!("{:?}", e);

    let x = <[_]>::into_vec(boxed::Box::new([1, 2, 3, 4]));
    println!("{:?}", x);

    Ok(())
}

#[macro_export]
macro_rules! my_vec {
    ()=>{Vec::new()};
    ($x:expr;$y:expr)=>{
        std::vec::from_elem($x,$y)
    };
    ($($x:expr),+ $(,)?) => {
        {
            // let mut temp_vec=Vec::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec
            <[_]>::into_vec(boxed::Box::new([$($x),*]))
        }
    };
}
