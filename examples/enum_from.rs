use macros::EnumFrom;
#[allow(unused)]
#[derive(Debug,EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right(String)
}


#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}


fn main() {
    let up:Direction = DirectionUp::new(42).into();
    println!("{:?}", up);
    let left:Direction = 10.into();
    println!("{:?}",left);
    let str:Direction = String::from("hello").into();
    println!("{:?}",str);
}
impl DirectionUp{
    fn  new(speed:u32)->Self{
        Self{speed}
    }
}