use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Router {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    vlans: Vec<String>,
    bgp: RouterBGP,
}

impl Router {
    pub fn get_config_string(&self) -> String
    {
        let mut ret: String = String::from("");
        ret += &format!("{:?}",&self);
        ret = ret.replace(",",",\n");
        ret = ret.replace("{","{\n");
        ret = ret.replace("}","\n}");
        return ret;
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct RouterBGP {
    #[serde(rename = "as")]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    router_bgp_as: Option<u32>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    connect_mode: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    neighbor_addresses: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    neighbor_as: Option<u32>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    routerid: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    server_addresses: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    port: Option<u32>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    vlan: String,
}
