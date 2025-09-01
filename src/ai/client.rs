use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Prompt {
    pub(crate) role: String,
    pub(crate) content: String,
}