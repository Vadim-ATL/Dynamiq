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

    
    let omega = 2.0 * PI;  

    let timestep = 0.01;  
    let end_time: f64 = 2.0;   
    let num_steps = (end_time / timestep) as usize;

    let oscillator = HarmonicOscillator { omega };

    let initial_state = Array1::from(vec![1.0, 0.0]);
    
    let solver = ODESolver::new(0.01, 2.0, Array1::from(vec![1.0, 0.0]), RK4::RK4_Method);


    let mut integrator = RK4::RK4_Method::new(2);

    let mut times = Vec::with_capacity(num_steps + 1);
    let mut positions = Vec::with_capacity(num_steps + 1);
    let mut exact_positions = Vec::with_capacity(num_steps + 1);

    let mut current_state = initial_state.clone();
    let mut t = 0.0;

    times.push(t);
    positions.push(current_state[0]);
    exact_positions.push(1.0);  

    for _ in 0..num_steps {
        current_state = integrator.step(&oscillator, t, &current_state.view(), timestep);
        t += timestep;

        times.push(t);
        positions.push(current_state[0]);
        exact_positions.push((omega * t).cos());
    }

    let max_error = positions.iter()
        .zip(exact_positions.iter())
        .map(|(x, x_exact)| (x - x_exact).abs())
        .fold(0.0f64, |max, error| max.max(error));

    println!("[MAE] Maximum absolute error: {:.4e}", max_error);
    
    println!("\nFirst values (t, numerical, exact, error):");
    for i in 0..5 {
        println!("t = {:.3}: {:.6}, {:.6}, {:.2e}",
                times[i],
                positions[i],
                exact_positions[i],
                (positions[i] - exact_positions[i]).abs());
    }
}