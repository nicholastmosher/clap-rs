
use args::{ArgSettings, Arg, ArgFlags};

#[derive(Debug, Clone, Default)]
pub struct Base<'key, 'other>
    where 'key: 'other
{
    pub name: &'key str,
    pub help: Option<&'other str>,
    pub long_help: Option<&'other str>,
    pub blacklist: Option<Vec<&'key str>>,
    pub settings: ArgFlags,
    pub r_unless: Option<Vec<&'key str>>,
    pub overrides: Option<Vec<&'key str>>,
    pub groups: Option<Vec<&'key str>>,
    pub requires: Option<Vec<(Option<&'other str>, &'key str)>>,
}

impl<'key, 'other> Base<'key, 'other> {
    pub fn new(name: &'key str) -> Self { Base { name: name, ..Default::default() } }

    pub fn set(&mut self, s: ArgSettings) { self.settings.set(s); }
    pub fn unset(&mut self, s: ArgSettings) { self.settings.unset(s); }
    pub fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
}

impl<'key, 'other, 'z> From<&'z Arg<'key, 'other>> for Base<'key, 'other> {
    fn from(a: &'z Arg<'key, 'other>) -> Self { a.b.clone() }
}

impl<'key, 'other> PartialEq for Base<'key, 'other> {
    fn eq(&self, other: &Base<'key, 'other>) -> bool {
        self.name == other.name
    }
}