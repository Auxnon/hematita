use core::hash::{Hash, Hasher};
use std::str::FromStr;
#[derive(Debug)]
pub struct WrappedFloat {
    pub val: f64,
}
// pub enum WrappedFloat {
//     Real(f64),
//     NaN,
// }
impl WrappedFloat {
    pub fn new(val: f64) -> Self {
        Self { val }
    }
}
impl Eq for WrappedFloat {}
impl PartialEq for WrappedFloat {
    fn eq(&self, other: &Self) -> bool {
        float_cmp::approx_eq!(f64, self.val, other.val)
        // self.val == other.val
    }
}

impl Hash for WrappedFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.to_bits().hash(state);
    }
}
impl Clone for WrappedFloat {
    fn clone(&self) -> Self {
        Self { val: self.val }
    }
}
impl FromStr for WrappedFloat {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            val: s.parse::<f64>()?,
        })
    }
}
