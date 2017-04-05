
use Arg;

#[derive(Debug)]
pub struct Switched<'other> {
    pub short: Option<char>,
    pub long: Option<&'other str>,
    pub aliases: Option<Vec<(&'other str, bool)>>, // (name, visible)
    pub disp_ord: usize,
    pub unified_ord: usize,
}

impl<'other> Default for Switched<'other> {
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

impl<'key, 'other, 'z> From<&'z Arg<'key, 'other>> for Switched<'other> {
    fn from(a: &'z Arg<'key, 'other>) -> Self { a.s.clone() }
}

impl<'other> Clone for Switched<'other> {
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
