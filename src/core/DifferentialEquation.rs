use crate::core::dependencies::*;

pub trait DifferentialEquation<T: Float> {
    fn evaluate(&self, t: T, state: &ArrayView1<T>, derivatives: &mut Array1<T>);
    fn dimension(&self) -> usize;
}