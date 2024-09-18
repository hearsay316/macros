use macros::{ EnumFromDarling};

// use macros::EnumFrom;
#[allow(unused)]
#[derive(Debug,EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right(String)
}


#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}


fn main() {
    let up:Direction<i32> = DirectionUp::new(42).into();
    println!("{:?}", up);
    let left:Direction<i32> = 10.into();
    println!("{:?}",left);
    let str:Direction<String> = String::from("hello").into();
    println!("{:?}",str);
}
impl<T> DirectionUp<T>{
    fn  new(speed:T)->Self{
        Self{speed}
    }
}
// impl<T> From<i32> for Direction<T>{
//     fn from(value:i32)->Self{
//         Direction::Left(value as  u32)
//     }
// }
// impl<T> From<String> for Direction<T>{
//     fn from(value:String)->Self{
//         Direction::Right(value)
//     }
// }
// impl <T> From<DirectionUp<T>> for Direction<T>{
//     fn  from(value:DirectionUp<T>)->Self{
//         Direction::Up(value)
//     }
// }