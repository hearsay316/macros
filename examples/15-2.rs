use std::cell::RefCell;
fn main() {
    let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec! [1, 2, 3]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;
    // let p1 = shared1.borrow();
    // let p2 = &p1[0];
    shared2.borrow_mut().push(4);
    let mut  a = shared2.borrow_mut().push(5);
    // a.push(5);
    println! ("{:?}", shared2);
    println!("{:?}",a);
}
