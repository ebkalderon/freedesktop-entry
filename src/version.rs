//! Supported versions of the Desktop Entry specification.

use std::fmt::Debug;

pub trait Version {
    const STRING: &'static str;

    type Fields: Debug + Default;
}

#[derive(Clone, Debug)]
pub struct V100;

impl Version for V100 {
    const STRING: &'static str = "1.0";

    type Fields = FieldsV100;
}

#[derive(Clone, Debug, Default)]
pub struct FieldsV100 {
    name: String,
    generic_name: Option<String>,
    no_display: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct V110;

impl Version for V110 {
    const STRING: &'static str = "1.1";

    type Fields = FieldsV110;
}

#[derive(Clone, Debug, Default)]
pub struct FieldsV110 {
    name: String,
    generic_name: Option<String>,
    no_display: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct V120;

impl Version for V120 {
    const STRING: &'static str = "1.2";

    type Fields = FieldsV120;
}

#[derive(Clone, Debug, Default)]
pub struct FieldsV120 {
    name: String,
    generic_name: Option<String>,
    no_display: Option<bool>,
}
