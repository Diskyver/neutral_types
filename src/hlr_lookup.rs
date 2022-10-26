use crate::PhoneInfoKind;
use crate::{Deserialize, Serialize};

/// Describes Hlr Status from Hrl lookup API
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "kebab-case"))]
pub enum HlrStatus {
    Ok,
    Absent,
    Unknown,
    Invalid,
    FixedLine,
    Voip,
    Failed,
}

/// Response of hlr lookup neutrinoapi.com endpoint
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HlrLookupResponse {
    #[serde(rename(deserialize = "number_valid"))]
    pub is_valid: bool,
    #[serde(rename(deserialize = "hlr_valid"))]
    pub is_hlr_valid: bool,
    pub hlr_status: HlrStatus,
    pub is_mobile: bool,
    pub is_ported: bool,
    pub is_roaming: bool,
    pub imsi: String,
    pub mcc: String,
    pub mnc: String,
    pub msin: String,
    pub msc: String,
    pub current_network: String,
    pub origin_network: String,
    pub ported_network: String,
    #[serde(rename(deserialize = "number_type"))]
    pub kind: PhoneInfoKind,
    pub location: String,
    pub country: String,
    pub country_code: String,
    pub country_code3: String,
    pub currency_code: String,
    pub roaming_country_code: String,
    pub international_calling_code: String,
    pub international_number: String,
    pub local_number: String,
}
