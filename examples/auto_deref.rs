use std::fmt;
use macros::AutoDeref;
#[allow(unused)]
#[derive(Debug,AutoDeref)]
#[deref(field="inner",mutable = true)]
pub struct RespBulkString{
    inner:String,
    nothing:(),
}

//    #[debug(skip)]
fn main() {
  let mut s = RespBulkString{
      inner:"hello".to_string(),
      nothing:()
  };
    println!("{:?}",s);
    s.inner.push_str(", Rust!");
    println!("{:?}",s);
}