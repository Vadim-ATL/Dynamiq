use crate::core::dependencies::*;
use crate::core::Integrator::Integrator;
use crate::core::DifferentialEquation::DifferentialEquation;
pub struct ODESolver<T: Float> {
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

impl<T: Float> ODESolver<T> {
    pub fn new(
        timestep: f64, 
        end_time: f64, 
        state_size: usize, 
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
        let mut t = T::zero();  // T is a generic type, should be initialized correctly
        let mut state = self.initial_state.clone();
        
        self.times.push(t);
        self.positions.push(state.clone());
    
        for _ in 0..self.num_steps {
            //state = self.integrator.step(equation, t, &state.view(), self.timestep);
            t = t + T::from(self.timestep).unwrap();  // Using T, not f64
            
            self.times.push(t);
            self.positions.push(state.clone());
        }
    }

    pub fn get_times(&self) -> &[T] {
        &self.times
    }

    pub fn get_positions(&self) -> &[Array1<T>] {
        &self.positions
    }

    pub fn get_exact_positions(&self) -> Option<&[Array1<T>]> {
        self.exact_positions.as_deref()
    }

    pub fn mae_calculate(&self, positions: &[Array1<T>], exact_positions: &[Array1<T>]) -> f64 {
        positions.iter()
            .zip(exact_positions.iter())
            .map(|(x, x_exact)| {
                (x - x_exact).mapv(|v| v.abs()).sum()
            })
            .fold(T::zero(), |max, error| max.max(error))
            .to_f64()  
            .unwrap_or(0.0) 
    }

}