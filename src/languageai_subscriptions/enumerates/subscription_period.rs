use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum SubscriptionPeriod {
    OneYear,
    ThreeMonths,
    OneMonth
}