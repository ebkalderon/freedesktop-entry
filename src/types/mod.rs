//! Valid types of desktop entry files.

pub use self::desktop::Desktop;
pub use self::directory::Directory;
pub use self::link::Link;

use std::fmt::Debug;

use version::Version;

mod desktop;
mod directory;
mod link;

pub trait Type<V: Version> {
    const EXTENSION: &'static str;

    type Fields: Debug + Default;
}
