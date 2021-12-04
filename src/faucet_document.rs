use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::acls;
use crate::dps_interface;
use crate::meter;
use crate::router;
use crate::serialization_helpers;
use crate::vlan_route;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FaucetDocument {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    pub acls: HashMap<String, acls::ACL>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    pub dps: HashMap<String, dps_interface::DP>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    pub meters: HashMap<String, meter::Meter>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    pub routers: HashMap<String, router::Router>,
    #[serde(default = "crate::serialization_helpers::default_u32_2")]
    pub version: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    pub vlans: HashMap<String, vlan_route::VLAN>,
}