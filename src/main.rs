mod core;
mod python_bindings;
use std::f64::consts::PI;

use core::dependencies::*;
use core::DifferentialEquation::DifferentialEquation;
use python_bindings::RK4::{self, RK4_Method};
use core::ODE::ODESolver;

struct HarmonicOscillator {
    omega: f64,
}
impl HarmonicOscillator{
    pub fn get_omega(&self) -> f64 {
        self.omega
    }
}

impl DifferentialEquation<f64> for HarmonicOscillator {
    fn dimension(&self) -> usize {
        2
    }
    fn evaluate(&self, _t: f64, state: &ArrayView1<f64>, derivatives: &mut Array1<f64>) {
        derivatives[0] = state[1];                  
        derivatives[1] = -self.omega * self.omega * state[0]; 
    }

}


fn main() {

    let omega: f64 = 2.0 * PI;  

    let oscillator = HarmonicOscillator { omega };    
    let integrator = Box::new(RK4::RK4_Method::new(2));
    let mut solver: ODESolver<f64> = ODESolver::new(0.01, 2.0, Array1::from(vec![1.0, 0.0]), integrator);
    
    solver.integrate(&oscillator, Array1::from(vec![omega]));

    solver.get_numerical_values();

}