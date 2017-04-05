#[doc(hidden)]
#[derive(Default, Clone, Debug)]
pub struct AppMeta<'other> {
    pub name: String,
    pub bin_name: Option<String>,
    pub author: Option<&'other str>,
    pub version: Option<&'other str>,
    pub long_version: Option<&'other str>,
    pub about: Option<&'other str>,
    pub long_about: Option<&'other str>,
    pub more_help: Option<&'other str>,
    pub pre_help: Option<&'other str>,
    pub aliases: Option<Vec<(&'other str, bool)>>, // (name, visible)
    pub usage_str: Option<&'other str>,
    pub usage: Option<String>,
    pub help_str: Option<&'other str>,
    pub disp_ord: usize,
    pub term_w: Option<usize>,
    pub max_w: Option<usize>,
    pub template: Option<&'other str>,
}

impl<'other> AppMeta<'other> {
    pub fn new() -> Self { Default::default() }
    pub fn with_name(s: String) -> Self { AppMeta { name: s, disp_ord: 999, ..Default::default() } }
}