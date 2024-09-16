
fn main() {
    let mut data = 100_i32;
    let mut p  = &mut data; // 这个是复制,不是引用
    let mut binding = 10;
    p = &mut binding;
    // data = data+1;
    println! ("{}", p);// p is dead
    println!("{data}");
    // data = 101;
    // p = &data;         // p is live again
    // println! ("{}", p);// p is dead again
}