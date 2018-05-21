//! Valid types of desktop entry files.

use std::fmt::Debug;

use version::{self, Version};

pub trait Type<V: Version> {
    type Fields: Debug + Default;
}

#[derive(Clone, Debug)]
pub struct Desktop;

impl Type<version::V100> for Desktop {
    type Fields = DesktopV100;
}

#[derive(Clone, Debug, Default)]
pub struct DesktopV100;

impl Type<version::V110> for Desktop {
    type Fields = DesktopV110;
}

#[derive(Clone, Debug, Default)]
pub struct DesktopV110;

impl Type<version::V120> for Desktop {
    type Fields = DesktopV120;
}

#[derive(Clone, Debug, Default)]
pub struct DesktopV120;

#[derive(Clone, Debug)]
pub struct Directry;

#[derive(Clone, Debug)]
pub struct Entry;
