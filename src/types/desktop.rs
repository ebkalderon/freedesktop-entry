//! Represents the `desktop` entry type.

use super::Type;
use version;

#[derive(Clone, Debug)]
pub struct Desktop;

impl Type<version::V100> for Desktop {
    const EXTENSION: &'static str = "desktop";

    type Fields = DesktopV100;
}

impl Type<version::V110> for Desktop {
    const EXTENSION: &'static str = "desktop";

    type Fields = DesktopV110;
}

impl Type<version::V120> for Desktop {
    const EXTENSION: &'static str = "desktop";

    type Fields = DesktopV120;
}

#[derive(Clone, Debug, Default)]
pub struct DesktopV100;

#[derive(Clone, Debug, Default)]
pub struct DesktopV110;

#[derive(Clone, Debug, Default)]
pub struct DesktopV120;
