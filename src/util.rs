use num_traits::{cast, Num, NumCast, Unsigned};

pub fn format_radix<T>(mut x: T, radix: u32) -> String
where
    T: Unsigned + NumCast + Num + Copy,
{
    let mut result = vec![];

    loop {
        let m = x % cast::<u32, T>(radix).unwrap();

        x = x / cast::<u32, T>(radix).unwrap();

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(cast::<T, u32>(m).unwrap(), radix).unwrap());

        if cast::<T, u32>(x).unwrap() == 0 {
            break;
        }
    }

    result.into_iter().rev().collect()
}
