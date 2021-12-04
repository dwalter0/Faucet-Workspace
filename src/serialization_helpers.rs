use std::{collections::HashMap};
use num_traits::identities::Zero;

pub fn skip_serializing_if_empty_str(input: &str) -> bool{
    input == ""
}

pub fn skip_serializing_if_empty_vec<T>(input: &Vec<T>) -> bool{
    input.is_empty()
}

pub fn skip_serializing_if_empty_hash<T,U>(input: &HashMap<T,U>) -> bool{
    input.is_empty()
}

pub fn skip_serializing_if_empty_number<T: Zero>(input: &T) -> bool{
    input.is_zero()
}

pub fn skip_serializing_if_none<T>(input: &Option<T>) -> bool{
    input.is_none()
}

pub fn default_bool_false() -> bool{
    false
}

pub fn default_bool_true() -> bool{
    true
}

pub fn default_u32_2() -> u32 {
    2
}

pub fn default_u32_3() -> u32 {
    3
}

pub fn default_u32_5() -> u32 {
    5
}

pub fn default_u32_10() -> u32 {
    10
}

pub fn default_u32_30() -> u32 {
    30
}

pub fn default_u32_32() -> u32 {
    32
}

pub fn default_u32_250() -> u32 {
    250
}

pub fn default_u32_255() -> u32 {
    255
}

pub fn default_u32_300() -> u32 {
    300
}

pub fn default_u32_1812() -> u32 {
    1812
}

pub fn default_u32_2052() -> u32 {
    2052
}

pub fn default_u32_9099() -> u32 {
    9099
}

pub fn default_u32_9001() -> u32 {
    9001
}

pub fn default_u32_9000() -> u32 {
    9000
}

pub fn default_f32_1() -> f32 {
    1.0
}

