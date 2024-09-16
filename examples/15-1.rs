use std::cell::Cell;

fn  main(){
    let data = Cell::new(100);

    let p = data;
    let a = &p;
   // data.set(10);
    println!("{p:?}");
    println!("{a:?}");

}