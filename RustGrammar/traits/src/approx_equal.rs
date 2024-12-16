

pub trait ApproxEqual {
    fn approval_equal(&self, other: &Self) -> bool;
}
impl ApproxEqual for f32 {
    fn approval_equal(&self, other: &Self) -> bool {
        (self - other).abs() <= ::std::f32::EPSILON
    }
}