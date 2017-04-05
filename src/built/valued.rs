use std::rc::Rc;
use std::ffi::{OsStr, OsString};

use vec_map::VecMap;

use Arg;

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Valued<'key, 'other>
    where 'key: 'other
{
    pub possible_vals: Option<Vec<&'other str>>,
    pub val_names: Option<VecMap<&'other str>>,
    pub num_vals: Option<u64>,
    pub max_vals: Option<u64>,
    pub min_vals: Option<u64>,
    pub validator: Option<Rc<Fn(String) -> Result<(), String>>>,
    pub validator_os: Option<Rc<Fn(&OsStr) -> Result<(), OsString>>>,
    pub val_delim: Option<char>,
    pub default_val: Option<&'other OsStr>,
    pub default_vals_ifs: Option<VecMap<(&'key str, Option<&'other OsStr>, &'other OsStr)>>,
    pub terminator: Option<&'other str>,
}

impl<'key, 'other> Default for Valued<'key, 'other> {
    fn default() -> Self {
        Valued {
            possible_vals: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            val_names: None,
            validator: None,
            validator_os: None,
            val_delim: None,
            default_val: None,
            default_vals_ifs: None,
            terminator: None,
        }
    }
}

impl<'key, 'other> Valued<'key, 'other> {
    pub fn fill_in(&mut self) {
        if let Some(ref vec) = self.val_names {
            if vec.len() > 1 {
                self.num_vals = Some(vec.len() as u64);
            }
        }
    }
}

impl<'key, 'other, 'z> From<&'z Arg<'key, 'other>> for Valued<'key, 'other> {
    fn from(a: &'z Arg<'key, 'other>) -> Self {
        let mut v = a.v.clone();
        if let Some(ref vec) = a.v.val_names {
            if vec.len() > 1 {
                v.num_vals = Some(vec.len() as u64);
            }
        }
        v
    }
}
