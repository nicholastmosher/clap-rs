#[doc(hidden)]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppMeta<'other> {
    pub name: String,
    pub bin_name: Option<String>,
    pub author: Option<&'other str>,
    pub version: Option<&'other str>,
    pub long_version: Option<&'other str>,
    pub about: Option<&'other str>,
    pub long_about: Option<&'other str>,
    pub after_help: Option<&'other str>,
    pub before_help: Option<&'other str>,
    pub aliases: Option<Vec<&'other str>>, 
    pub visible_aliases: Option<Vec<&'other str>>,
    pub custom_usage: Option<&'other str>,
    pub custom_help: Option<&'other str>,
    pub display_order: usize,
    pub template: Option<&'other str>,
    pub term_width: Option<usize>,
    pub max_term_width: Option<usize>,
    pub _usage: Option<String>,
}

impl<'other> AppMeta<'other> {
    pub fn new() -> Self { Default::default() }
    pub fn with_name(s: String) -> Self { AppMeta { name: s, disp_ord: 999, ..Default::default() } }
}