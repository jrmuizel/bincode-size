use bincode_maxsize_derive::BincodeMaxSize;
use bincode_maxsize::BincodeMaxSize;

#[derive(BincodeMaxSize)]
pub struct Bar {
    a: [u32; 5],
    b: (u8, u8)
}

#[derive(BincodeMaxSize)]
pub enum Foo {
    M(i32),
    O(u32, u32, u32),
    P(Option<f64>, f64),
    Q(Bar)
}

#[test]
fn it_works() {
    assert_eq!(Foo::bincode_max_size(), 26);
}
