use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct DokuNotificationServiceOrAcquirerOrChannel {
    id: String,
}

#[derive(Deserialize, Serialize)]
struct DokuNotificationOrder {
    amount: u32,
    invoice_number: String,
}

#[derive(Deserialize, Serialize)]
struct DokuNotificationVaInfo {
    virtual_account_number: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DokuNotificationTransaction {
    status: String,
    date: String,
    pub original_request_id: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DokuNotification {
    service: DokuNotificationServiceOrAcquirerOrChannel,
    acquirer: DokuNotificationServiceOrAcquirerOrChannel,
    channel: DokuNotificationServiceOrAcquirerOrChannel,
    order: DokuNotificationOrder,
    virtual_account_info: DokuNotificationVaInfo,
    pub transaction: DokuNotificationTransaction,
}
