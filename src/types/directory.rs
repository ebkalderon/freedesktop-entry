//! Represents the `directory` entry type.

use super::Type;
use version;

#[derive(Clone, Debug)]
pub struct Directory;

impl Type<version::V100> for Directory {
    const EXTENSION: &'static str = "directory";

    type Fields = DirectoryV100;
}

impl Type<version::V110> for Directory {
    const EXTENSION: &'static str = "directory";

    type Fields = DirectoryV110;
}

impl Type<version::V120> for Directory {
    const EXTENSION: &'static str = "directory";

    type Fields = DirectoryV120;
}

#[derive(Clone, Debug, Default)]
pub struct DirectoryV100;

#[derive(Clone, Debug, Default)]
pub struct DirectoryV110;

#[derive(Clone, Debug, Default)]
pub struct DirectoryV120;
