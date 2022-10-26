use crate::object_empty_as_none;
use crate::NeutrinoTimeZoneResponse;
use crate::{Deserialize, Serialize};
use std::net::IpAddr;

/// Response of ip info neutrinoapi.com endpoint
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IpInfoResponse {
    pub ip: IpAddr,
    #[serde(alias = "valid", alias = "is_valid")]
    pub is_valid: bool,
    pub is_v6: bool,
    pub is_v4_mapped: bool,
    pub is_bogon: bool,
    pub country: String,
    pub country_code: String,
    pub country_code3: String,
    pub continent_code: String,
    pub currency_code: String,
    pub city: String,
    pub region: String,
    pub longitude: f64,
    pub latitude: f64,
    pub hostname: String,
    pub host_domain: String,
    #[serde(deserialize_with = "object_empty_as_none")]
    pub timezone: Option<NeutrinoTimeZoneResponse>,
}
