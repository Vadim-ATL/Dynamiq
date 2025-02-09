mod core;
mod python_bindings;
use std::f64::consts::PI;

use core::dependencies::*;
use core::DifferentialEquation::DifferentialEquation;
use python_bindings::RK4::{self, RK4_Method};
use core::ODE::ODESolver;

pub struct HarmonicOscillator<T: Float> {
    omega: T,
}

impl<T: Float> HarmonicOscillator<T> {
    pub fn new(omega: T) -> Self {
        Self { omega }
    }
}

impl<T: Float> DifferentialEquation<T> for HarmonicOscillator<T> {
    fn evaluate(&self, _t: T, state: &ArrayView1<T>, derivatives: &mut Array1<T>) {
        derivatives[0] = state[1]; // dx/dt = v
        derivatives[1] = -self.omega * self.omega * state[0]; // dv/dt = -ω²x
    }

    fn dimension(&self) -> usize {
        2
    }

    /// Exact solution: x(t) = cos(ωt), v(t) = -sin(ωt)
    fn exact_solution(&self, t: T) -> Option<Array1<T>> {
        let time = t.to_f64()?; 
        let omega = self.omega.to_f64()?; 
        let exact_x = T::from((omega * time).cos())?;
        let exact_v = T::from(-(omega * time).sin())?;
        
        Some(Array1::from(vec![exact_x, exact_v]))
    }
}

fn main() {

    let omega: f64 = 2.0 * PI;  

    let oscillator = HarmonicOscillator { omega };    
    let integrator = Box::new(RK4::RK4_Method::new(2));
    let mut solver: ODESolver<f64> = ODESolver::new(0.01, 2.0, Array1::from(vec![1.0, 0.0]), integrator);
    
    solver.integrate(&oscillator);

    solver.get_numerical_values();

}