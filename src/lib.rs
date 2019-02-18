use std::marker::PhantomData;
use std::mem::size_of;

pub trait BincodeMaxSize {
    fn bincode_max_size() -> usize;
}

macro_rules! primitive_impls {
    ($($ty:ident),*) => {
        $(
        impl BincodeMaxSize for $ty {
            fn bincode_max_size() -> usize {
                size_of::<Self>()
            }
        }
        )*
    }
}

primitive_impls!(u8, i8, u16, i16, u32, i32, f32, f64, u64, i64);

macro_rules! array_impls {
    ($($len:tt),*) => {
        $(
        impl<T> BincodeMaxSize for [T; $len]
        where
            T: BincodeMaxSize
        {
            fn bincode_max_size() -> usize {
                <T as BincodeMaxSize>::bincode_max_size() * $len
            }
        }
        )*
    }
}

array_impls!( 0,  1,  2,  3,  4,  5,  6,  7,
             08, 09, 10, 11, 12, 13, 14, 15,
             16, 17, 18, 19, 20, 21, 22, 23,
             24, 25, 26, 27, 28, 29, 30, 31,
             32);

macro_rules! tuple_impl {
    ($($tyvar:ident),*) => {
        impl<$($tyvar: BincodeMaxSize),*> BincodeMaxSize for ( $( $tyvar ),* ,)
        {
            fn bincode_max_size() -> usize {
                0 $( + <$tyvar as BincodeMaxSize>::bincode_max_size() )*
            }
        }
    }
}

tuple_impl!(A);
tuple_impl!(A, B);
tuple_impl!(A, B, C);
tuple_impl!(A, B, C, D);
tuple_impl!(A, B, C, D, E);
tuple_impl!(A, B, C, D, E, F);
tuple_impl!(A, B, C, D, E, F, G);
tuple_impl!(A, B, C, D, E, F, G, H);
tuple_impl!(A, B, C, D, E, F, G, H, I);
tuple_impl!(A, B, C, D, E, F, G, H, I, J);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L);


impl<T: BincodeMaxSize> BincodeMaxSize for Option<T> {
    fn bincode_max_size() -> usize {
        1 + T::bincode_max_size()
    }
}

impl<T: BincodeMaxSize> BincodeMaxSize for PhantomData<T> {
    fn bincode_max_size() -> usize {
        0
    }
}
