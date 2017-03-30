#[cfg(feature = "serde")]
use serde;
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

use args::{ArgSettings, Arg, ArgFlags};

#[derive(Debug, Clone, Default)]
pub struct Base<'a, 'b>
    where 'a: 'b
{
    pub name: &'a str,
    pub help: Option<&'b str>,
    pub conflicts: Option<Vec<&'a str>>,
    pub settings: ArgFlags,
    pub required_unless: Option<Vec<&'a str>>,
    pub overrides: Option<Vec<&'a str>>,
    pub groups: Option<Vec<&'a str>>,
    pub requires: Option<Vec<(Option<&'b str>, &'a str)>>,
}

impl<'n, 'e> Base<'n, 'e> {
    pub fn new(name: &'n str) -> Self { Base { name: name, ..Default::default() } }

    pub fn set(&mut self, s: ArgSettings) { self.settings.set(s); }
    pub fn unset(&mut self, s: ArgSettings) { self.settings.unset(s); }
    pub fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Base<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self { a.b.clone() }
}

impl<'n, 'e> PartialEq for Base<'n, 'e> {
    fn eq(&self, other: &Base<'n, 'e>) -> bool {
        self.name == other.name
    }
}

#[cfg(feature = "serde")]
impl<'a, 'b> serde::Serialize for Base<'a, 'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut struc = try!(serializer.serialize_struct("Base", 8));
        try!(struc.serialize_field("name", &self.name));
        try!(struc.serialize_field("help", &self.help));
        try!(struc.serialize_field("conflicts", &self.conflicts));
        try!(struc.serialize_field("settings", &self.settings));
        try!(struc.serialize_field("r_unless", &self.r_unless));
        try!(struc.serialize_field("overrides", &self.overrides));
        try!(struc.serialize_field("groups", &self.groups));
        try!(struc.serialize_field("requires", &self.requires));
        struc.end()
    }
}