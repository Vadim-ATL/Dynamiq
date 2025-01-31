use crate::core::dependencies::*;

pub trait StepSizeController<T: Float> {
    fn adjust_step(&mut self, 
                  error_estimate: T, 
                  current_dt: T) -> (T, bool);
}