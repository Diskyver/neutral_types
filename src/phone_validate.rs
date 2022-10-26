use crate::PhoneInfoKind;
use crate::{Deserialize, Serialize};

/// Response of phone validate neutrinoapi.com endpoint
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PhoneValidateResponse {
    #[serde(alias = "valid", alias = "is_valid")]
    pub is_valid: bool,
    #[serde(alias = "type", alias = "kind")]
    pub kind: PhoneInfoKind,
    pub international_calling_code: String,
    pub international_number: String,
    pub local_number: String,
    pub location: String,
    pub country: String,
    pub country_code: String,
    pub country_code3: String,
    pub currency_code: String,
    pub is_mobile: bool,
    pub prefix_network: String,
}
