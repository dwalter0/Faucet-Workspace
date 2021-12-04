use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VLAN {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    pub acl_in: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    acls_in: Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    description: String,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    dot1x_assigned: bool,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    faucet_vips: Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    faucet_mac: String,
    #[serde(default = "crate::serialization_helpers::default_u32_255")]
    max_hosts: u32,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    minimum_ip_size_check: bool,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    name: String,
    #[serde(default = "crate::serialization_helpers::default_u32_2052")]
    proactive_arp_limit: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_2052")]
    proactive_nd_limit: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    routes: Vec<VLANRoute>,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    targeted_gw_resolution: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    unicast_flood: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    vid: Option<u32>,
}

impl VLAN {
    pub fn get_config_string(&self) -> String {
        let mut ret: String = String::from("");
        // ret += "VID: ";
        // ret += &self.vid.unwrap().to_string();
        // ret += "\n";
        ret += &format!("{:?}",&self);
        ret = ret.replace(",",",\n");
        ret = ret.replace("{","{\n");
        ret = ret.replace("}","\n}");
        return ret;
        
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct VLANRoute {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    route: HashMap<String, String>,
}
