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
pub struct DefaultValue<'a, 'b> {
    pub if_arg: &'a str, 
    pub if_val: Option<&'b OsStr>, 
    pub def_val: &'b OsStr
}

#[cfg(feature = "serde")]
impl<'a, 'b> serde::Serialize for DefaultValue<'a, 'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut struc = try!(serializer.serialize_struct("DefaultValue", 3));
        try!(struc.serialize_field("if_arg", &self.if_arg));
        try!(struc.serialize_field("if_val", &self.if_val.map(|oss| oss.as_bytes())));
        try!(struc.serialize_field("def_val", &self.def_val.as_bytes()));
        struc.end()
    }
}