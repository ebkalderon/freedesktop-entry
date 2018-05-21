#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate mime;
#[macro_use]
extern crate serde;
extern crate url;
#[macro_use]
extern crate url_serde;

use self::types::{Application, Directory, Link, Type};
use self::version::{V120, Version};

pub mod categories;
pub mod prelude;
pub mod types;
pub mod version;

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

impl DesktopEntry<Application> {
    pub fn name(&self) -> &str {
        self.standard.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiation() {
        let entry: DesktopEntry<Application> = DesktopEntry::new();
        println!("{:?}", entry);
        let entry: DesktopEntry<Directory> = DesktopEntry::new();
        println!("{:?}", entry);
        let entry: DesktopEntry<Link> = DesktopEntry::new();
        println!("{:?}", entry);
    }
}
