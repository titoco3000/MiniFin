use serde::{Serialize, Deserialize};
#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct SortParameter {
    pub i: u32,
    pub d: bool
}
