use serde::{Deserialize,  Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Staff {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) work_days: f32
}

impl Staff {}