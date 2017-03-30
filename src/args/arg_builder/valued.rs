// Std
use std::rc::Rc;
use std::ffi::{OsStr, OsString};

// Third Party
#[cfg(feature = "serde")]
use serde;
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

// Internal
use Arg;
use args::arg_builder::default_vals::DefaultValue;
#[cfg(not(target_os = "windows"))]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use osstringext::OsStrExt3;

#[allow(missing_debug_implementations)]
#[derive(Clone, Default)]
pub struct Valued<'a, 'b>
    where 'a: 'b
{
    pub possible_vals: Option<Vec<&'b str>>,
    pub val_names: Option<Vec<&'b str>>,
    pub num_vals: Option<u64>,
    pub max_vals: Option<u64>,
    pub min_vals: Option<u64>,
    pub validator: Option<Rc<Fn(String) -> Result<(), String>>>,
    pub validator_os: Option<Rc<Fn(&OsStr) -> Result<(), OsString>>>,
    pub val_delim: Option<char>,
    pub default_val: Option<&'b OsStr>,
    pub default_vals_ifs: Option<Vec<DefaultValue<'a, 'b>>>,
    pub terminator: Option<&'b str>,
}

impl<'n, 'e> Valued<'n, 'e> {
    pub fn fill_in(&mut self) {
        if let Some(ref vec) = self.val_names {
            if vec.len() > 1 {
                self.num_vals = Some(vec.len() as u64);
            }
        }
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Valued<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        let mut v = a.v.clone();
        if let Some(ref vec) = a.v.val_names {
            if vec.len() > 1 {
                v.num_vals = Some(vec.len() as u64);
            }
        }
        v
    }
}

#[cfg(feature = "serde")]
impl<'a, 'b> serde::Serialize for Valued<'a, 'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut struc = try!(serializer.serialize_struct("Valued", 11));
        try!(struc.serialize_field("possible_vals", &self.possible_vals));
        try!(struc.serialize_field("num_vals", &self.num_vals));
        try!(struc.serialize_field("min_vals", &self.min_vals));
        try!(struc.serialize_field("max_vals", &self.max_vals));
        try!(struc.serialize_field("val_names", &self.val_names));
        try!(struc.serialize_field("validator", &self.validator.is_some()));
        try!(struc.serialize_field("validator_os", &self.validator_os.is_some()));
        try!(struc.serialize_field("val_delim", &self.val_delim));
        try!(struc.serialize_field("default_val", &self.default_val.map(|oss| oss.as_bytes())));
        try!(struc.serialize_field("default_vals_ifs", &self.default_vals_ifs));
        try!(struc.serialize_field("terminator", &self.terminator));
        struc.end()
    }
}