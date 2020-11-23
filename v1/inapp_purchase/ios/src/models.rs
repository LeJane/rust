// https://developer.apple.com/library/content/releasenotes/General/ValidateAppStoreReceipt/Chapters/ValidateRemotely.html
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum Environment {
    Production,
    Sandbox,
}

// The IAPRequest type has the request parameter
#[derive(Deserialize, Serialize, Debug)]
pub struct IAPRequest {
    #[serde(rename(serialize = "receipt-data", deserialize = "receipt-data"))]
    pub receipt_data: String,
    // Only used for receipts that contain auto-renewable subscriptions.
    pub password: String,
    // Only used for iOS7 style app receipts that contain auto-renewable or non-renewing subscriptions.
    // If value is true, response includes only the latest renewal transaction for any subscriptions.
    #[serde(rename(
        serialize = "exclude-old-transactions",
        deserialize = "exclude-old-transactions"
    ))]
    pub exclude_old_transactions: bool,
}

// The ReceiptCreationDate type indicates the date when the app receipt was created.
#[derive(Deserialize, Serialize, Debug)]
pub struct ReceiptCreationDate {
    pub receipt_creation_date: String,
    pub receipt_creation_date_ms: String,
    pub receipt_creation_date_pst: String,
}

// The RequestDate type indicates the date and time that the request was sent
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestDate {
    pub request_date: String,
    pub request_date_ms: String,
    pub request_date_pst: String,
}

// The PurchaseDate type indicates the date and time that the item was purchased
#[derive(Deserialize, Serialize, Debug)]
pub struct PurchaseDate {
    pub purchase_date: String,
    pub purchase_date_ms: String,
    pub purchase_date_pst: String,
}

// The OriginalPurchaseDate type indicates the beginning of the subscription period
#[derive(Deserialize, Serialize, Debug)]
pub struct OriginalPurchaseDate {
    pub original_purchase_date: String,
    pub original_purchase_date_ms: String,
    pub original_purchase_date_pst: String,
}

// The ExpiresDate type indicates the expiration date for the subscription
#[derive(Deserialize, Serialize, Debug)]
pub struct ExpiresDate {
    pub expires_date: String,
    pub expires_date_ms: String,
    pub expires_date_pst: String,
    pub expires_date_formatted: String,
    pub expires_date_formatted_pst: String,
}

// The CancellationDate type indicates the time and date of the cancellation by Apple customer support
#[derive(Deserialize, Serialize, Debug)]
pub struct CancellationDate {
    pub cancellation_date: String,
    pub cancellation_date_ms: String,
    pub cancellation_date_pst: String,
}

// The GracePeriodDate type indicates the grace period date for the subscription
#[derive(Deserialize, Serialize, Debug)]
pub struct GracePeriodDate {
    pub grace_period_expires_date: String,
    pub grace_period_expires_date_ms: String,
    pub grace_period_expires_date_pst: String,
}
// The InApp type has the receipt attributes
#[derive(Deserialize, Serialize, Debug)]
pub struct InApp {
    pub quantity: String,
    pub product_id: String,
    pub transaction_id: String,
    pub original_transaction_id: String,
    pub web_order_line_item_id: String,
    pub promotional_offer_id: String,
    pub subscription_group_identifier: String,

    pub is_trial_period: String,
    pub is_in_intro_offer_period: String,
    pub is_upgraded: String,

    pub expires_date: String,
    pub expires_date_ms: String,
    pub expires_date_pst: String,
    pub expires_date_formatted: String,
    pub expires_date_formatted_pst: String,

    pub purchase_date: String,
    pub purchase_date_ms: String,
    pub purchase_date_pst: String,

    pub original_purchase_date: String,
    pub original_purchase_date_ms: String,
    pub original_purchase_date_pst: String,

    pub cancellation_date: String,
    pub cancellation_date_ms: String,
    pub cancellation_date_pst: String,

    pub cancellation_reason: String,
}

// The Receipt type has whole data of receipt
#[derive(Deserialize, Serialize, Debug)]
pub struct Receipt {
    pub receipt_type: String,
    pub adam_id: i64,
    pub app_item_id: String,
    pub bundle_id: String,
    pub application_version: String,
    pub download_id: i64,
    pub version_external_identifier: String,
    pub original_application_version: String,
    pub in_app: Vec<InApp>,

    pub receipt_creation_date: String,
    pub receipt_creation_date_ms: String,
    pub receipt_creation_date_pst: String,

    pub request_date: String,
    pub request_date_ms: String,
    pub request_date_pst: String,

    pub original_purchase_date: String,
    pub original_purchase_date_ms: String,
    pub original_purchase_date_pst: String,
}

// A pending renewal may refer to a renewal that is scheduled in the future or a renewal that failed in the past for some reason.
#[derive(Deserialize, Serialize, Debug)]
pub struct PendingRenewalInfo {
    pub expiration_intent: String,
    pub auto_renew_product_id: String,
    pub is_in_billing_retry_period: String,
    pub auto_renew_status: String,
    pub price_consent_status: String,
    pub product_id: String,
    pub original_transaction_id: String,

    pub grace_period_expires_date: String,
    pub grace_period_expires_date_ms: String,
    pub grace_period_expires_date_pst: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IAPResponse {
    pub status: i32,
    pub environment: String,
    pub receipt: Receipt,
    pub latest_receipt_info: Vec<InApp>,
    pub latest_receipt: String,
    pub pending_renewal_info: Vec<PendingRenewalInfo>,
    #[serde(rename(serialize = "is-retryable", deserialize = "is-retryable"))]
    pub is_retryable: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatusResponse {
    pub status: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IAPResponseForIOS6 {
    pub auto_renew_product_id: String,
    pub auto_renew_status: i32,
    pub cancellation_reason: String,
    pub expiration_intent: String,
    pub is_in_billing_retry_period: String,
    pub receipt: ReceiptForIOS6,
    pub latest_expired_receipt_info: ReceiptForIOS6,
    pub latest_receipt: String,
    pub latest_receipt_info: ReceiptForIOS6,
    pub status: i8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReceiptForIOS6 {
    pub app_item_id: String,
    pub bid: String,
    pub bvrs: String,

    pub cancellation_date: String,
    pub cancellation_date_ms: String,
    pub cancellation_date_pst: String,

    pub expires_date: String,
    pub expires_date_ms: String,
    pub expires_date_pst: String,
    pub expires_date_formatted: String,
    pub expires_date_formatted_pst: String,

    pub is_trial_period: String,
    pub is_in_intro_offer_period: String,
    pub item_id: String,
    pub product_id: String,

    pub purchase_date: String,
    pub purchase_date_ms: String,
    pub purchase_date_pst: String,

    pub original_transaction_id: String,

    pub original_purchase_date: String,
    pub original_purchase_date_ms: String,
    pub original_purchase_date_pst: String,

    pub quantity: String,
    pub transaction_id: String,
    pub unique_identifier: String,
    pub unique_vendor_identifier: String,
    pub version_external_identifier: String,
    pub web_order_line_item_id: String,
}
