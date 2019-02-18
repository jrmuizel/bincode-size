use bincode_maxsize_derive::BincodeMaxSize;
use bincode_maxsize::BincodeMaxSize;
#[derive(BincodeMaxSize)]
pub enum Foo {
    M(i32),
    O(u32, u32, u32),
    P(f64, f64)
}

#[test]
fn works() {
    println!("{}", Option::<u32>::bincode_max_size());

    println!("{}", Foo::bincode_max_size());
}