use crate::core::dependencies::*;

pub trait EventDetector<T: Float> {
    fn detect(&self, t: T, state: &ArrayView1<T>) -> bool;
    fn get_event_time(&self) -> Option<T>;
}