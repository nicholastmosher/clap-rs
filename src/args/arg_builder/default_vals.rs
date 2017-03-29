use std::ffi::OsStr;

#[cfg(feature = "serde")]
use serde;
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

#[cfg(not(target_os = "windows"))]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use osstringext::OsStrExt3;

#[derive(Clone, Debug)]
pub struct DefaultValueIfs<'a, 'b> {
    data: Vec<DefaultValue<'a, 'b>>
}

#[cfg(feature = "serde")]
impl<'a, 'b> serde::Serialize for DefaultValueIfs<'a, 'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut struc = try!(serializer.serialize_struct("DefaultValueIfs", 1));
        try!(struc.serialize_field("data", &self.data));
        struc.end()
    }
}

#[derive(Clone, Debug)]
struct DefaultValue<'a, 'b> {
    if_name: &'a str, 
    if_val: Option<&'b OsStr>, 
    def_val: &'b OsStr
}

#[cfg(feature = "serde")]
impl<'a, 'b> serde::Serialize for DefaultValue<'a, 'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut struc = try!(serializer.serialize_struct("DefaultValue", 3));
        try!(struc.serialize_field("if_name", &self.if_name));
        try!(struc.serialize_field("if_val", &self.if_val.map(|oss| oss.as_bytes())));
        try!(struc.serialize_field("def_val", &self.def_val.as_bytes()));
        struc.end()
    }
}