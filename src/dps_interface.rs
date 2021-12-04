use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DP {
    #[serde(default = "crate::serialization_helpers::default_u32_30")]
    advertise_interval: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_250")]
    arp_neighbor_timeout: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    description: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    dot1x: HashMap<String, DPDot1X>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    dp_id: Option<u32>,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    drop_broadcast_source_address: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    drop_spoofed_faucet_mac: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    group_table: bool,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    hardware: String,
    #[serde(default = "crate::serialization_helpers::default_u32_9001")]
    high_priority: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_9099")]
    highest_priority: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_10")]
    ignore_learn_ins: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    interfaces: HashMap<u32, DPInterface>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    interface_ranges: HashMap<String, DPInterface>,
    #[serde(default = "crate::serialization_helpers::default_u32_10")]
    learn_ban_timeout: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_10")]
    learn_jitter: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    lldp_beacon: HashMap<String, DPLLDP>,
    #[serde(default = "crate::serialization_helpers::default_u32_9000")]
    low_priority: u32,
    #[serde(default)]
    lowest_priority: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_10")]
    max_host_fib_retry_count: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_5")]
    max_hosts_per_resolve_cycle: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_32")]
    max_resolve_backoff_time: u32,
    #[serde(default)]
    metrics_rate_limit_sec: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    name: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    ofchannel_log: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    packetin_pps: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    slowpath_pps: Option<u32>,
    #[serde(default)]
    priority_offset: u32,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    proactive_learn_v4: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    proactive_learn_v6: bool,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    #[serde(default)]
    stack: Option<DPStack>,
    #[serde(default = "crate::serialization_helpers::default_u32_300")]
    timeout: u32,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    use_idle_timeout: bool,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    table_sizes: HashMap<String, String>, //noted in defects, unsure of attribute type
    #[serde(default = "crate::serialization_helpers::default_f32_1")]
    port_table_scale_factor: f32,
    #[serde(default)]
    #[serde(skip_serializing_if="crate::serialization_helpers::skip_serializing_if_none")]
    global_vlan: Option<u32>,
}

impl DP {
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
struct DPStack {
    #[serde(default)]
    priority: u32,
    #[serde(default = "crate::serialization_helpers::default_u32_3")]
    down_time_multiple: u32,
    #[serde(default = "crate::serialization_helpers::default_f32_1")]
    min_stack_health: f32,
    #[serde(default = "crate::serialization_helpers::default_f32_1")]
    min_lacp_health: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPLLDP {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    system_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    send_interval: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    max_per_interval: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPDot1X {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    nfv_intf: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    nfv_sw_port: Option<u32>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    radius_ip: String,
    #[serde(default = "crate::serialization_helpers::default_u32_1812")]
    radius_port: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    radius_secret: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    noauth_acl: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    auth_acl: String,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    dot1x_assigned: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPInterface {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    acl_in: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    acls_in: Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    description: String,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    dot1x: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    dot1x_acl: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    dot1x_mab: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    enabled: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    hairpin: bool,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    lldp_beacon: HashMap<String, DPInterfaceLLDPBeacon>,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    loop_protect: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    loop_protect_external: bool,
    #[serde(default = "crate::serialization_helpers::default_u32_255")]
    max_hosts: u32,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    mirror: Vec<String>,
    name: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    native_vlan: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    number: Option<u32>,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    opstatus_reconf: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    output_only: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    permanent_learn: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    stack: Option<DPInterfaceStack>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    tagged_vlans: Vec<String>,
    #[serde(default = "crate::serialization_helpers::default_bool_true")]
    unicast_flood: bool,
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    restricted_bcast_arpnd: bool,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    coprocessor: HashMap<String, String>, //defect noted uncertain about this attribute type
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPInterfaceStack {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    dp: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    port: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPInterfaceLLDPBeacon {
    #[serde(default = "crate::serialization_helpers::default_bool_false")]
    enable: bool,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    org_tlvs: Vec<DPInterfaceLLDPBeaconTLV>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    port_descr: String,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    system_name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DPInterfaceLLDPBeaconTLV {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    info: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    oui: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    subtype: Option<u32>,
}
