use crate::NeutrinoProviderKind;
use crate::{Deserialize, Serialize};
use std::net::IpAddr;

/// Response of ip probe neutrinoapi.com endpoint
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IpProbeResponse {
    pub country: String,
    pub country_code: String,
    pub provider_domain: String,
    pub city: String,
    pub vpn_domain: String,
    pub is_vpn: bool,
    pub as_cidr: String,
    #[serde(alias = "valid", alias = "is_valid")]
    pub is_valid: bool,
    pub provider_type: NeutrinoProviderKind,
    pub hostname: String,
    pub as_age: i64,
    pub continent_code: String,
    pub is_bogon: bool,
    pub ip: IpAddr,
    pub as_country_code: String,
    pub provider_description: String,
    pub as_country_code3: String,
    pub is_v4_mapped: bool,
    pub is_isp: bool,
    pub provider_website: String,
    pub as_description: String,
    pub is_hosting: bool,
    pub as_domains: Vec<String>,
    pub host_domain: String,
    pub is_proxy: bool,
    pub currency_code: String,
    pub region: String,
    pub asn: String,
    pub country_code3: String,
    pub is_v6: bool,
}
