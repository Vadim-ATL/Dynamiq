use crate::core::dependencies::*;
use crate::core::DifferentialEquation::DifferentialEquation;

pub trait Integrator<T: Float> {
    fn step(&mut self, 
           equation: &dyn DifferentialEquation<T>,
           t: T, 
           state: &ArrayView1<T>, 
           dt: f64) -> Array1<T>;
}