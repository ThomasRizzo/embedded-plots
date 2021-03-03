use core::ops::{Range, Add, Sub, Mul, Div};

pub trait Scalable<T>
    where
        T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>,
{
    fn scale_between_ranges(&self, input_range: &Range<T>, output_range: &Range<T>) -> T;
}

impl<T> Scalable<T> for T
    where
        T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>,
{
    fn scale_between_ranges(&self, input_range: &Range<T>, output_range: &Range<T>) -> T
    {
        (*self - input_range.start) * (output_range.end - output_range.start)
            / (input_range.end - input_range.start) + output_range.start
    }
}


#[cfg(test)]
mod tests {
    use core::ops::Range;
    use test_case::test_case;
    use crate::range_conv::Scalable;

    #[test_case(0..10, 0..10, 5 => 5; "equal ranges")]
    #[test_case(0..10, 0..20, 5 => 10; "double")]
    #[test_case(0..20, 0..10, 10 => 5; "half")]
    #[test_case(- 20..20, 0..10, 0 => 5; "negative input range")]
    #[test_case(0..10, - 20..20, 5 => 0; "negative output range")]
    #[test_case(0..10, 10..0, 2 => 8; "reversing")]
    #[test_case(- 20..20, 0..20, - 10 => 5; "reversing negative range")]
    fn convert(in_range: Range<i32>, out_range: Range<i32>, val: i32) -> i32 {
        val.scale_between_ranges(&in_range, &out_range)
    }
}
