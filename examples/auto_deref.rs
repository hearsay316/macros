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
  let  s = RespBulkString{
      inner:"hello".to_string(),
      nothing:()
  };
    println!("{:?}",s);
}