use crate::NeutrinoSensor;
use crate::{Deserialize, Serialize};
use std::net::IpAddr;

/// Response of ip blocklist neutrinoapi.com endpoint
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IpBlocklistResponse {
    pub ip: IpAddr,
    pub is_listed: bool,
    pub last_seen: usize,
    pub list_count: usize,
    pub blocklists: Vec<String>,
    pub sensors: Vec<NeutrinoSensor>,
    pub is_proxy: bool,
    pub is_tor: bool,
    pub is_vpn: bool,
    pub is_malware: bool,
    pub is_spyware: bool,
    pub is_dshield: bool,
    pub is_hijacked: bool,
    pub is_spider: bool,
    pub is_bot: bool,
    pub is_spam_bot: bool,
    pub is_exploit_bot: bool,
}
