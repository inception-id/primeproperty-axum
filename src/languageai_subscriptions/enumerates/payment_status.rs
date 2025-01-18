use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum PaymentStatus {
    Success,
    Pending,
    Fail
}