use crate::core::dependencies::*;
use crate::core::Integrator::Integrator;
use crate::core::DifferentialEquation::DifferentialEquation;
use std::fmt::Display;

pub struct ODESolver<T: Float > {
    timestep: f64,
    end_time: f64,
    num_steps: usize,
    initial_state: Array1<T>,
    integrator: Box<dyn Integrator<T>>,
    times: Vec<T>,
    positions: Vec<Array1<T>>, 
    exact_positions: Option<Vec<Array1<T>>>,
}

//step_controller: Option<Box<dyn StepSizeController<T>>>,
//config: Option<SolverConfig<T>>,
//dense_output: Option<Box<dyn DenseOutput<T>>>,
impl <T: Float + std::fmt::Display> ODESolver<T>{
    pub fn new(
        timestep: f64, 
        end_time: f64, 
        initial_state: Array1<T>,
        integrator:Box<dyn Integrator<T>>, 
    ) -> Self {
        let num_steps = (end_time / timestep) as usize;

        ODESolver {
            timestep,
            end_time,
            num_steps,
            initial_state,
            integrator, 
            times: Vec::with_capacity(num_steps + 1),
            positions: Vec::with_capacity(num_steps + 1),
            exact_positions: None,
        }
    }

    pub fn integrate(&mut self, equation: &dyn DifferentialEquation<T>) {
        let mut t = T::zero(); 
        let mut state = self.initial_state.clone();
        
        self.times.push(t);
        self.positions.push(state.clone());

        let mut exact_positions = equation.exact_solution(t).map(|s| vec![s]);

        for _ in 0..self.num_steps {
            // Numerical step
            if let Some(step_size) = T::from(self.timestep) {
                state = self.integrator.step(equation, t, &state.view(), step_size);
                t = t + step_size;
            } else {
                break;
            }

            self.times.push(t);
            self.positions.push(state.clone());

            // Exact solution
            if let Some(ref mut positions) = exact_positions {
                if let Some(exact) = equation.exact_solution(t) {
                    positions.push(exact);
                }
            }
        }

        self.exact_positions = exact_positions;
    }
    
    pub fn get_numerical_values(&self) {
        println!("\nFirst values (t, numerical, exact, error):");
    
        for i in 0..5 {
            let t = self.times[i];
            let numerical = format!("{:.6}", self.positions[i][0]); 
            
            if let Some(ref exact_positions) = self.exact_positions {
                let exact = format!("{:.6}", exact_positions[i][0]); 
                let error = (self.positions[i][0] - exact_positions[i][0]).abs();
                
                println!("t = {:.3}: {}, {}, {:.6e}", t, numerical, exact, error.to_f64().unwrap());
            } else {
                println!("t = {:.3}: {}, (no exact), (no error)", t, numerical);
            }
        }
    
        // Print MAE at the end
        if let Some(ref exact_positions) = self.exact_positions {
            let mae = self.mae_calculate(&self.positions, Some(exact_positions));
            println!("[MAE] Maximum absolute error: {:.4e}", mae);
        }
    }
    
    pub fn mae_calculate(&self, positions: &[Array1<T>], exact_positions: Option<&[Array1<T>]>) -> f64 {
        if let Some(exact_positions) = exact_positions {
            positions.iter()
                .zip(exact_positions.iter())
                .map(|(x, x_exact)| {
                    (x[0] - x_exact[0]).abs()
                })
                .fold(T::zero(), |max, error| max.max(error))
                .to_f64()
                .unwrap_or(0.0)
        } else {
            0.0 
        }
    }
}