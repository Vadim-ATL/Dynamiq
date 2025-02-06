use crate::core::dependencies::*;
use crate::core::DifferentialEquation::DifferentialEquation; 
use crate::core::Integrator::Integrator;

#[derive(Debug)]
pub struct RK4_Method<T: Float + ScalarOperand> {
    ///Temp array for storing calculation
    temp: Array1<T>,
    ///The four slopes for RK4
    k1: Array1<T>,
    k2: Array1<T>,
    k3: Array1<T>,
    k4: Array1<T>,
}
///RK4 Constructor with zeroed arrays shaped as state_size
impl<T: Float + ScalarOperand> RK4_Method<T> {
    pub fn new(state_size: usize) -> Self {
        RK4_Method {
            k1: Array1::zeros(state_size),
            k2: Array1::zeros(state_size),
            k3: Array1::zeros(state_size),
            k4: Array1::zeros(state_size),
            temp: Array1::zeros(state_size),
        }
    }
}
///Implemented Integrator trait for RK4 method
/// Passed inside equation of reference system, implemented DifEq. trait
/// t - current time
/// state - (array) represent current state of the system
/// the time step dt 
impl<T: Float + ScalarOperand> Integrator<T> for RK4_Method<T> {
    fn step(&mut self, equation: &dyn DifferentialEquation<T>, t: T, state: &ArrayView1<T>, dt: T) -> Array1<T> {  
        
        ///Constants initialization
        let half = T::from(0.5).expect("Conversion failed for half");
        let sixth = T::from(1.0 / 6.0).expect("Conversion failed for sixth");
        let third = T::from(1.0 / 3.0).expect("Conversion failed for third");


        // Convert constants to the type `T` and handle the result without unwrap

        ///Slope at the beginning of the interval
        /// evaluate method computes f(t,y), the slope at (t,y)
        equation.evaluate(t, state, &mut self.k1);
        /// updates values of k (array of slope values) with dt time step element-wise
        self.k1.mapv_inplace(|x| x * dt);
        ///copies all values to the temporary array from state array
        self.temp.assign(state);

        /// multipies each element of k1 by 0.5 
        let scaled_k1 = &self.k1 * half;
        ///assign values of vectors state and scaled_k1 vector to temporary vector
        /// to store the computations
        self.temp.assign(&(state + &scaled_k1));

        ///Slope at the midpoint, using k1, again use f(t,y) with new array of temp and 
        /// t = t + dt * 0.5
        equation.evaluate(t + dt * half, &self.temp.view(), &mut self.k2);
        /// updates values of k (array of slope values) with dt time step element-wise
        self.k2.mapv_inplace(|x| x * dt);
        ///after using temp in equation of k2, assigning it to state vector again.
        self.temp.assign(state);
        ///scaling k2 to 0.5
        let scaled_k2 = &self.k2 * half;
        ///assign values of vectors state and scaled_k2 vector to temporary vector
        /// to store the computations for evaluation
        self.temp.assign(&(state + &scaled_k2));

        ///Slope at the midpoint, using k2
        equation.evaluate(t + dt * half, &self.temp.view(), &mut self.k3);
        self.k3.mapv_inplace(|x| x * dt);

        self.temp.assign(state);
        self.temp.assign(&(state + &self.k3));

        ///Slope at the midpoint, using k3
        equation.evaluate(t + dt, &self.temp.view(), &mut self.k4);
        self.k4.mapv_inplace(|x| x * dt);
        /// 1/6 and 1/3 converting to float for further calculation
        /// of the result
        let sixth = T::from(1.0 / 6.0).unwrap();
        let third = T::from(1.0 / 3.0).unwrap();
        ///taking state vector to result and making it mutable, owned Array1
        let mut result = state.to_owned();
        ///finally weighed_sum is 1/6 * k1 + 1/3 * k2 + 1/3 * k3 + 1/6 * k4
        let weighted_sum = &(&(&(&self.k1 * sixth) + &(&self.k2 * third)) + &(&self.k3 * third)) + &(&self.k4 * sixth);
        ///final result is sum of state and weighted sum
        result.assign(&(state + &weighted_sum));

        result
    }
}