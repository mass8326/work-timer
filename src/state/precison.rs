use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerPrecision {
    #[default]
    Decisecond,
    Second,
}
