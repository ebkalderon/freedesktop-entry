#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use self::types::Type;
use self::version::{V120, Version};

pub mod categories;
pub mod types;
ub mod version;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopEntry<T: Type<V>, V: Version = V120> {
    standard: V::Fields,
    type_specific: T::Fields,
}

impl<V: Version, T: Type<V>> DesktopEntry<T, V> {
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
        use super::types::Desktop;
        use super::version::V120;

        let entry: DesktopEntry<Desktop> = DesktopEntry::new();
    }
}
