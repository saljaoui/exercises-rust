#[derive(Debug, Clone)]
pub struct StepIterator<T> {
pub beg: T, 
pub end: T, 
pub step: T
}

use std::ops::Add;
use std::cmp::*;

impl<T> StepIterator<T> {
	pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            beg: beg,
            end: end,
            step: step

        }
	}
}

impl<T: std::fmt::Debug + std::ops::AddAssign + Clone + PartialOrd> std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.beg.clone();
        if self.beg <= self.end {
            self.beg += self.step.clone();
            return Some(current);
        }
        None
    }
}