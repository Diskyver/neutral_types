//! # neutral_types
//! This crate provide all necessary types to create a client of neutrinoapi.com
//! Currently this crate is used as a dependency of [neutral](https://docs.rs/neutral/latest/neutral) and you shoud not use this crate directly.
//!
//! If you want to interact with neutrinoapi.com from rust, use the [neutral](https://docs.rs/neutral/latest/neutral) crate.

use serde::{Deserialize, Deserializer, Serialize};

pub mod hlr_lookup;
pub mod ip_blocklist;
pub mod ip_info;
pub mod ip_probe;
pub mod phone_validate;

/// Describes a kind of phone number.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "kebab-case"))]
pub enum PhoneInfoKind {
    Mobile,
    FixedLine,
    PremiumRate,
    TollFree,
    Voip,
    Unknown,
}

/// Describes a timezone from neutrinoapi.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NeutrinoTimeZoneResponse {
    pub id: String,
    pub name: String,
    pub abbr: String,
    pub date: String,
    pub time: String,
    pub offset: String,
}

/// Describes a network provider.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all(deserialize = "snake_case", serialize = "snake_case"))]
pub enum NeutrinoProviderKind {
    Isp,
    Hosting,
    Vpn,
    Proxy,
    University,
    Government,
    Commercial,
    Unknown,
}

/// Desribe a neutrinoapi sensor.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NeutrinoSensor {
    pub id: usize,
    pub blocklist: String,
    pub description: String,
}

pub(crate) fn object_empty_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    for<'a> T: Deserialize<'a>,
{
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct Empty {}

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Aux<T> {
        T(T),
        Empty(Empty),
        Null,
    }

    match Deserialize::deserialize(deserializer)? {
        Aux::T(t) => Ok(Some(t)),
        Aux::Empty(_) | Aux::Null => Ok(None),
    }
}
