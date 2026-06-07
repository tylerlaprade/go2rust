
pub trait GoInteger: Copy + Clone + PartialOrd + 'static {
    fn go_from_i128(value: i128) -> Self;
    fn go_to_i128(self) -> i128;
}

macro_rules! impl_go_integer {
    ($($t:ty),* $(,)?) => {
        $(
            impl GoInteger for $t {
                fn go_from_i128(value: i128) -> Self {
                    value as $t
                }

                fn go_to_i128(self) -> i128 {
                    self as i128
                }
            }
        )*
    };
}

impl_go_integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

fn go_integer_from_i128<T: GoInteger>(value: i128) -> T {
    T::go_from_i128(value)
}

fn go_integer_cast<T: GoInteger, U: GoInteger>(value: U) -> T {
    T::go_from_i128(value.go_to_i128())
}

fn go_integer_add_one<T: GoInteger>(value: T) -> T {
    T::go_from_i128(value.go_to_i128() + 1)
}

fn go_integer_sub_one<T: GoInteger>(value: T) -> T {
    T::go_from_i128(value.go_to_i128() - 1)
}
