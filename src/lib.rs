#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

pub mod categories;
pub mod ty;
pub mod version;

use self::version::Version;
use self::ty::Type;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopEntry<V: Version, T: Type<V>> {
    standard: V::Fields,
    type_specific: T::Fields,
}

impl<V: Version, T: Type<V>> DesktopEntry<V, T> {
    pub fn new() -> Self {
        DesktopEntry {
            standard: V::Fields::default(),
            type_specific: T::Fields::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiation() {
        use super::ty::Desktop;
        use super::version::V120;

        let entry: DesktopEntry<V120, Desktop> = DesktopEntry::new();
    }
}
