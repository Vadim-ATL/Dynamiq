use crate::core::dependencies::*;

pub trait DenseOutput<T: Float> {
    fn evaluate(&self, t: T) -> Array1<T>;
    fn valid_range(&self) -> (T, T);
}
