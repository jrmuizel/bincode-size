pub trait BincodeMaxSize {
    fn bincode_max_size() -> usize;
}

impl BincodeMaxSize for i32 {
    fn bincode_max_size() -> usize {
        4
    }
}

impl BincodeMaxSize for u32 {
    fn bincode_max_size() -> usize {
        4
    }
}

impl BincodeMaxSize for bool {
    fn bincode_max_size() -> usize {
        1
    }
}

impl BincodeMaxSize for f32 {
    fn bincode_max_size() -> usize {
        4
    }
}

impl BincodeMaxSize for f64 {
    fn bincode_max_size() -> usize {
        8
    }
}

impl BincodeMaxSize for i64 {
    fn bincode_max_size() -> usize {
        8
    }
}

impl BincodeMaxSize for u64 {
    fn bincode_max_size() -> usize {
        8
    }
}

impl<T: BincodeMaxSize> BincodeMaxSize for Option<T> {
    fn bincode_max_size() -> usize {
        1 + T::bincode_max_size()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
