use std::collections::HashMap;

use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct DevShell {
    pub(super) packages: Vec<String>,
    pub(super) env_vars: HashMap<String, String>,
}
