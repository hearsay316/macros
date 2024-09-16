use macros::my_vec;

fn main(){
    let a:Vec<i32> = my_vec![1,2,3,4,];
    vec![1,2,3];
    // let a:Vec<i32> = my_vec! {4;5};
    println!("{:?}",a);
}

// my_vec
#[macro_export]
macro_rules! my_vec {
    ()=>{
        Vec::new()
    };
    ($elem:expr ; $n:expr)=>{
        std::vec::from_elem($elem,$n)
    };
    ($($x:expr),+$(,)?) => {
        {
            // let mut temp_vec = Vec::new();
            // $(temp_vec.push($x);)*
            // temp_vec
            <[_]>::into_vec(Box::new([$($x),+]))
        }
    };
}