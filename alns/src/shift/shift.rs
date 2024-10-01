use serde::{Deserialize, Serialize};
use rand::Rng;
use rand::seq::SliceRandom;
pub trait Random {
    fn random<R: Rng>(rng: &mut R) -> Option<Self> where Self: Sized;
}

#[derive(Deserialize, Serialize)]
pub struct Shift {
    pub (crate) id: String,
    pub (crate) name: String,
    pub (crate) duration: i8,
    pub (crate) shift_type: String,
}