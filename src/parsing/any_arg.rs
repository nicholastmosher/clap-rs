// Std
use std::rc::Rc;
use std::fmt as std_fmt;
use std::ffi::{OsStr, OsString};

// Third Party
use vec_map::{self, VecMap};

// Internal
use args::settings::ArgSettings;

#[doc(hidden)]
pub trait AnyArg<'key, 'other>: std_fmt::Display {
    fn name(&self) -> &'key str;
    fn overrides(&self) -> Option<&[&'other str]>;
    fn aliases(&self) -> Option<Vec<&'other str>>;
    fn requires(&self) -> Option<&[(Option<&'other str>, &'key str)]>;
    fn blacklist(&self) -> Option<&[&'other str]>;
    fn required_unless(&self) -> Option<&[&'other str]>;
    fn is_set(&self, ArgSettings) -> bool;
    fn set(&mut self, ArgSettings);
    fn has_switch(&self) -> bool;
    fn max_vals(&self) -> Option<u64>;
    fn min_vals(&self) -> Option<u64>;
    fn num_vals(&self) -> Option<u64>;
    fn possible_vals(&self) -> Option<&[&'other str]>;
    fn validator(&self) -> Option<&Rc<Fn(String) -> Result<(), String>>>;
    fn validator_os(&self) -> Option<&Rc<Fn(&OsStr) -> Result<(), OsString>>>;
    fn short(&self) -> Option<char>;
    fn long(&self) -> Option<&'other str>;
    fn val_delim(&self) -> Option<char>;
    fn takes_value(&self) -> bool;
    fn val_names(&self) -> Option<&VecMap<&'other str>>;
    fn help(&self) -> Option<&'other str>;
    fn long_help(&self) -> Option<&'other str>;
    fn default_val(&self) -> Option<&'other OsStr>;
    fn default_vals_ifs(&self) -> Option<vec_map::Values<(&'key str, Option<&'other OsStr>, &'other OsStr)>>;
    fn longest_filter(&self) -> bool;
    fn val_terminator(&self) -> Option<&'other str>;
}

pub trait DispOrder {
    fn disp_ord(&self) -> usize;
}
