use macros::{AutoDebug};
#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString{
    inner:String,
    #[debug(skip)]
    nothing:(),
}

//    #[debug(skip)]
fn main() {
    let  s = RespBulkString{
        inner:"hello".to_string(),
        nothing:(),
    };
    println!("{:?}",s);
}