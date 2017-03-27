#[cfg(feature = "serde")]
use serde;
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

use Arg;

#[derive(Debug)]
pub struct Switched<'b> {
    pub short: Option<char>,
    pub long: Option<&'b str>,
    pub aliases: Option<Vec<(&'b str, bool)>>, // (name, visible)
    pub disp_ord: usize,
    pub unified_ord: usize,
}

impl<'e> Default for Switched<'e> {
    fn default() -> Self {
        Switched {
            short: None,
            long: None,
            aliases: None,
            disp_ord: 999,
            unified_ord: 999,
        }
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Switched<'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self { a.s.clone() }
}

impl<'e> Clone for Switched<'e> {
    fn clone(&self) -> Self {
        Switched {
            short: self.short,
            long: self.long,
            aliases: self.aliases.clone(),
            disp_ord: self.disp_ord,
            unified_ord: self.unified_ord,
        }
    }
}

#[cfg(feature = "serde")]
impl<'b> serde::Serialize for Switched<'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut struc = try!(serializer.serialize_struct("Switched", 5));
        try!(struc.serialize_field("short", &self.short));
        try!(struc.serialize_field("long", &self.long));
        try!(struc.serialize_field("aliases", &self.aliases));
        try!(struc.serialize_field("disp_ord", &self.disp_ord));
        try!(struc.serialize_field("unified_ord", &self.unified_ord));
        struc.end()
    }
}