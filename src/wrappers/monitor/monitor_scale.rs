use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MonitorScale(pub(super) f64);

impl MonitorScale {
    pub const MIN: MonitorScale = MonitorScale(0.5);
    pub const MAX: MonitorScale = MonitorScale(5.0);
}

#[derive(Debug, Error)]
pub enum TryParseMonitorScaleError {
    #[error("Monitor scale must be a multiple of 0.5")]
    MustBeMultipleOfOneHalf,
    #[error("Monitor scale out of range. Must be between 0.5 and 5.0")]
    OutOfRange,
}

impl TryFrom<f64> for MonitorScale {
    type Error = TryParseMonitorScaleError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value % 0.5 != 0.0 {
            return Err(TryParseMonitorScaleError::MustBeMultipleOfOneHalf);
        }

        if !(0.5..=5.0).contains(&value) {
            return Err(TryParseMonitorScaleError::OutOfRange);
        }

        Ok(Self(value))
    }
}

impl From<MonitorScale> for Value {
    fn from(value: MonitorScale) -> Self {
        value.0.into()
    }
}
