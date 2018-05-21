//! Supported versions of the Desktop Entry specification.

use std::fmt::Debug;
use std::path::PathBuf;

/// TODO: Find another place to put this.
#[derive(Clone, Debug)]
pub enum ShowIn {
    Only(Vec<String>),
    Not(Vec<String>),
}

/// TODO: Find another place to put this.
#[derive(Clone, Debug)]
pub enum Icon {
    Path(PathBuf),
    Name(String),
}

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
    pub name: String,
    pub generic_name: Option<String>,
    pub no_display: Option<bool>,
    pub comment: Option<String>,
    pub hidden: Option<bool>,
    pub icon: Option<Icon>,
    pub show_in: Option<ShowIn>,
}

#[derive(Clone, Debug)]
pub struct V110;

impl Version for V110 {
    const STRING: &'static str = "1.1";

    type Fields = FieldsV110;
}

#[derive(Clone, Debug, Default)]
pub struct FieldsV110 {
    pub name: String,
    pub generic_name: Option<String>,
    pub no_display: Option<bool>,
    pub comment: Option<String>,
    pub hidden: Option<bool>,
    pub icon: Option<Icon>,
    pub show_in: Option<ShowIn>,
    pub dbus_activatable: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct V120;

impl Version for V120 {
    const STRING: &'static str = "1.2";

    type Fields = FieldsV120;
}

#[derive(Clone, Debug, Default)]
pub struct FieldsV120 {
    pub name: String,
    pub generic_name: Option<String>,
    pub no_display: Option<bool>,
    pub comment: Option<String>,
    pub hidden: Option<bool>,
    pub icon: Option<Icon>,
    pub show_in: Option<ShowIn>,
    pub dbus_activatable: Option<bool>,
}
