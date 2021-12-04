use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ACL {
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    allow: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    force_port_vlan: bool,
    //Default is complex?? see doco. added to defect
    cookie: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    meter: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    mirror: String,
    //The output field can be a list or a dictionary. Implemented here as a dictionary but this needs to be revisted. Added to backlog
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    output: HashMap<String, ACLOutput>,
}

impl ACL {
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
struct ACLOutput {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    set_fields: Vec<HashMap<String, String>>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    port: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    ports: Vec<String>,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    pop_vlans: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    vlan_vid: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    swap_vid: Option<u32>,
    //The vlan_vids field is a bunch of bullshit list of integer or a list of objects see doco. added to backlog
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    vlan_vids: Vec<u32>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    failover: HashMap<String, ACLOutputFailover>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    tunnel: HashMap<String, ACLOutputTunnel>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ACLOutputFailover {
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    group_id: Option<u32>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    ports: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ACLOutputTunnel {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    tunnel_type: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    tunnel_id: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    dp: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    port: String,
    //Needs some analysis on actual type - might not just be string description sounds complex, backlog entry created.
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    exit_instructions: Vec<String>,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    maintain_encapsulation: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    bi_directional: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    reverse: bool,
}
