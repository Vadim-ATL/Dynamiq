use crate::core::dependencies::*;

///DiffEq constructor
pub trait DifferentialEquation<T: Float> {
    fn evaluate(&self, t: T, state: &ArrayView1<T>, derivatives: &mut Array1<T>);
    fn dimension(&self) -> usize;
    ///Exact solution if it exists in DiffEq
    fn exact_solution(&self, _t: T) -> Option<Array1<T>> {
        None
    }
}