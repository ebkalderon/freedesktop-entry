//! Represents the `link` entry type.

use url::Url;

use super::Type;
use version;

#[derive(Clone, Debug)]
pub struct Link;

impl Type<version::V100> for Link {
    const EXTENSION: &'static str = "desktop";

    type Fields = LinkV100;
}

impl Type<version::V110> for Link {
    const EXTENSION: &'static str = "desktop";

    type Fields = LinkV110;
}

impl Type<version::V120> for Link {
    const EXTENSION: &'static str = "desktop";

    type Fields = LinkV120;
}

#[derive(Clone, Debug, Default)]
pub struct LinkV100 {
    pub url: Option<Url>,
}

#[derive(Clone, Debug, Default)]
pub struct LinkV110 {
    /// FIXME: Cannot be optional.
    pub url: Option<Url>,
}

#[derive(Clone, Debug, Default)]
pub struct LinkV120 {
    /// FIXME: Cannot be optional.
    pub url: Option<Url>,
}
