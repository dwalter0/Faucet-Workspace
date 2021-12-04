use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Meter {
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    meter_id: Option<u32>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_hash")]
    #[serde(default)]
    entry: HashMap<String, MeterEntry>,
}

impl Meter {
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
struct MeterEntry {
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    flags: Vec<String>,
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_vec")]
    #[serde(default)]
    bands: Vec<HashMap<String, MeterBand>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MeterBand {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_empty_str")]
    #[serde(default)]
    meter_band_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    rate: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    burst_size: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::serialization_helpers::skip_serializing_if_none")]
    prec_level: Option<u32>,
}
