use serde::{Deserialize, Serialize};
use sgp4::Elements;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ElementStore {
    pub elements: HashMap<u64, Elements>,
}
