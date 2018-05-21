//! Represents the `desktop` entry type.

use std::collections::HashMap;
use std::path::PathBuf;

use mime::Mime;

use super::Type;
use categories::Categories;
use version::{self, Icon};

/// TODO: Find another place to put this.
#[derive(Clone, Debug)]
pub struct Action {
    name: String,
    icon: Option<Icon>,
    exec: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Application;

impl Type<version::V100> for Application {
    const EXTENSION: &'static str = "desktop";

    type Fields = ApplicationV100;
}

impl Type<version::V110> for Application {
    const EXTENSION: &'static str = "desktop";

    type Fields = ApplicationV110;
}

impl Type<version::V120> for Application {
    const EXTENSION: &'static str = "desktop";

    type Fields = ApplicationV120;
}

#[derive(Clone, Debug, Default)]
pub struct ApplicationV100 {
    pub try_exec: Option<PathBuf>,
    pub exec: Option<String>,
    pub path: Option<PathBuf>,
    pub terminal: Option<bool>,
    pub mime_type: Option<Vec<Mime>>,
    pub categories: Option<Categories>,
    pub keywords: Option<Vec<String>>,
    pub startup_notify: Option<bool>,
    pub startup_wm_class: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ApplicationV110 {
    pub try_exec: Option<PathBuf>,
    pub exec: Option<String>,
    pub path: Option<PathBuf>,
    pub terminal: Option<bool>,
    pub actions: Option<HashMap<String, Action>>,
    pub mime_type: Option<Vec<Mime>>,
    pub categories: Option<Categories>,
    pub keywords: Option<Vec<String>>,
    pub startup_notify: Option<bool>,
    pub startup_wm_class: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ApplicationV120 {
    pub try_exec: Option<PathBuf>,
    pub exec: Option<String>,
    pub path: Option<PathBuf>,
    pub terminal: Option<bool>,
    pub actions: Option<HashMap<String, Action>>,
    pub mime_type: Option<Vec<Mime>>,
    pub categories: Option<Categories>,
    pub keywords: Option<Vec<String>>,
    pub startup_notify: Option<bool>,
    pub startup_wm_class: Option<String>,
}
