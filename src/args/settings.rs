// Std
use std::ascii::AsciiExt;
use std::str::FromStr;

// Third Party
#[cfg(feature = "serde")]
use serde;
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;

bitflags! {
    flags Flags: u16 {
        const REQUIRED         = 1 << 0,
        const MULTIPLE         = 1 << 1,
        const EMPTY_VALS       = 1 << 2,
        const GLOBAL           = 1 << 3,
        const HIDDEN           = 1 << 4,
        const TAKES_VAL        = 1 << 5,
        const USE_DELIM        = 1 << 6,
        const NEXT_LINE_HELP   = 1 << 7,
        const R_UNLESS_ALL     = 1 << 8,
        const REQ_DELIM        = 1 << 9,
        const DELIM_NOT_SET    = 1 << 10,
        const HIDE_POS_VALS    = 1 << 11,
        const ALLOW_TAC_VALS   = 1 << 12,
        const REQUIRE_EQUALS   = 1 << 13,
        const LAST             = 1 << 14,
        const HIDE_DEFAULT_VAL = 1 << 15,
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ArgFlags(Flags);

impl ArgFlags {
    pub fn new() -> Self { ArgFlags::default() }

    impl_settings!{ArgSettings,
        Required => REQUIRED,
        Multiple => MULTIPLE,
        EmptyValues => EMPTY_VALS,
        Global => GLOBAL,
        Hidden => HIDDEN,
        TakesValue => TAKES_VAL,
        UseValueDelimiter => USE_DELIM,
        NextLineHelp => NEXT_LINE_HELP,
        RequiredUnlessAll => R_UNLESS_ALL,
        RequireDelimiter => REQ_DELIM,
        ValueDelimiterNotSet => DELIM_NOT_SET,
        HidePossibleValues => HIDE_POS_VALS,
        AllowLeadingHyphen => ALLOW_TAC_VALS,
        RequireEquals => REQUIRE_EQUALS,
        Last => LAST,
        HideDefaultValue => HIDE_DEFAULT_VAL
    }
}

impl Default for ArgFlags {
    fn default() -> Self { ArgFlags(EMPTY_VALS | DELIM_NOT_SET) }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ArgFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        use ArgSettings::*;

        // TODO: There's probably a better way to do this...
        macro_rules! maybe_push {
            ($s:ident, $v:ident) => {{
                if self.is_set($s) { $v.push($s); }
            }};
        }
        let mut v = vec![];

        maybe_push!(AllowLeadingHyphen, v);
        maybe_push!(Required, v);
        maybe_push!(Multiple, v);
        maybe_push!(Global, v);
        maybe_push!(EmptyValues, v);
        maybe_push!(Hidden, v);
        maybe_push!(TakesValue, v);
        maybe_push!(UseValueDelimiter, v);
        maybe_push!(NextLineHelp, v);
        maybe_push!(RequiredUnlessAll, v);
        maybe_push!(RequireDelimiter, v);
        maybe_push!(ValueDelimiterNotSet, v);
        maybe_push!(HidePossibleValues, v);
        maybe_push!(HideDefaultValue, v);
        maybe_push!(RequireEquals, v);
        maybe_push!(Last, v);

        let mut seq = try!(serializer.serialize_seq(Some(v.len())));
        for e in v {
            try!(seq.serialize_element(&e));
        }
        seq.end()
    }
}


/// Various settings that apply to arguments and may be set, unset, and checked via getter/setter
/// methods [`Arg::set`], [`Arg::unset`], and [`Arg::is_set`]
/// [`Arg::set`]: ./struct.Arg.html#method.set
/// [`Arg::unset`]: ./struct.Arg.html#method.unset
/// [`Arg::is_set`]: ./struct.Arg.html#method.is_set
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgSettings {
    /// Allows vals that start with a '-'
    AllowLeadingHyphen,
    /// The argument allows empty values such as `--option ""`
    EmptyValues,
    /// The argument should be propagated down through all child [`SubCommands`]
    /// [`SubCommand`]: ./struct.SubCommand.html
    Global,
    /// Hides the default value from the help string
    HideDefaultValue,
    /// Hides the possible values from the help string
    HidePossibleValues,
    /// The argument should **not** be shown in help text
    Hidden,
    /// Specifies that the arg is the last positional argument and may be accessed early via `--`
    /// syntax
    Last,
    /// The argument may be used multiple times such as `--flag --flag`
    Multiple,
    /// Prints the help text on the line after the argument
    NextLineHelp,
    /// Requires the use of a value delimiter for all multiple values
    RequireDelimiter,
    /// Require options use `--option=val` syntax
    RequireEquals,
    /// The argument must be used
    Required,
    /// The argument accepts a value, such as `--option <value>`
    TakesValue,
    /// Determines if the argument allows values to be grouped via a delimter
    UseValueDelimiter,
    #[doc(hidden)]
    RequiredUnlessAll,
    #[doc(hidden)]
    ValueDelimiterNotSet,
}

impl FromStr for ArgSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "required" => Ok(ArgSettings::Required),
            "multiple" => Ok(ArgSettings::Multiple),
            "global" => Ok(ArgSettings::Global),
            "emptyvalues" => Ok(ArgSettings::EmptyValues),
            "hidden" => Ok(ArgSettings::Hidden),
            "takesvalue" => Ok(ArgSettings::TakesValue),
            "usevaluedelimiter" => Ok(ArgSettings::UseValueDelimiter),
            "nextlinehelp" => Ok(ArgSettings::NextLineHelp),
            "requiredunlessall" => Ok(ArgSettings::RequiredUnlessAll),
            "requiredelimiter" => Ok(ArgSettings::RequireDelimiter),
            "valuedelimiternotset" => Ok(ArgSettings::ValueDelimiterNotSet),
            "hidepossiblevalues" => Ok(ArgSettings::HidePossibleValues),
            "allowleadinghyphen" => Ok(ArgSettings::AllowLeadingHyphen),
            "requireequals" => Ok(ArgSettings::RequireEquals),
            "last" => Ok(ArgSettings::Last),
            "hidedefaultvalue" => Ok(ArgSettings::HideDefaultValue),
            _ => Err("unknown ArgSetting, cannot convert from str".to_owned()),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ArgSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        use ArgSettings::*;
        match *self {
            AllowLeadingHyphen => {
                serializer.serialize_unit_variant("ArgSettings", 1, "AllowLeadingHyphen")
            }
            EmptyValues => serializer.serialize_unit_variant("ArgSettings", 2, "EmptyValues"),
            Global => serializer.serialize_unit_variant("ArgSettings", 3, "Global"),
            HideDefaultValue => {
                serializer.serialize_unit_variant("ArgSettings", 4, "HideDefaultValue")
            }
            HidePossibleValues => {
                serializer.serialize_unit_variant("ArgSettings", 5, "HidePossibleValues")
            }
            Hidden => serializer.serialize_unit_variant("ArgSettings", 6, "Hidden"),
            Last => serializer.serialize_unit_variant("ArgSettings", 7, "Last"),
            Multiple => serializer.serialize_unit_variant("ArgSettings", 8, "Multiple"),
            NextLineHelp => serializer.serialize_unit_variant("ArgSettings", 9, "NextLineHelp"),
            RequireDelimiter => {
                serializer.serialize_unit_variant("ArgSettings", 10, "RequireDelimiter")
            }
            RequireEquals => serializer.serialize_unit_variant("ArgSettings", 12, "RequireEquals"),
            Required => serializer.serialize_unit_variant("ArgSettings", 13, "Required"),
            TakesValue => serializer.serialize_unit_variant("ArgSettings", 14, "TakesValue"),
            UseValueDelimiter => {
                serializer.serialize_unit_variant("ArgSettings", 15, "UseValueDelimiter")
            }
            RequiredUnlessAll => {
                serializer.serialize_unit_variant("ArgSettings", 16, "RequiredUnlessAll")
            }
            ValueDelimiterNotSet => {
                serializer.serialize_unit_variant("ArgSettings", 17, "ValueDelimiterNotSet")
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::ArgSettings;

    #[test]
    fn arg_settings_fromstr() {
        assert_eq!("allowleadinghyphen".parse::<ArgSettings>().unwrap(),
                   ArgSettings::AllowLeadingHyphen);
        assert_eq!("emptyvalues".parse::<ArgSettings>().unwrap(),
                   ArgSettings::EmptyValues);
        assert_eq!("global".parse::<ArgSettings>().unwrap(),
                   ArgSettings::Global);
        assert_eq!("hidepossiblevalues".parse::<ArgSettings>().unwrap(),
                   ArgSettings::HidePossibleValues);
        assert_eq!("hidden".parse::<ArgSettings>().unwrap(),
                   ArgSettings::Hidden);
        assert_eq!("multiple".parse::<ArgSettings>().unwrap(),
                   ArgSettings::Multiple);
        assert_eq!("nextlinehelp".parse::<ArgSettings>().unwrap(),
                   ArgSettings::NextLineHelp);
        assert_eq!("requiredunlessall".parse::<ArgSettings>().unwrap(),
                   ArgSettings::RequiredUnlessAll);
        assert_eq!("requiredelimiter".parse::<ArgSettings>().unwrap(),
                   ArgSettings::RequireDelimiter);
        assert_eq!("required".parse::<ArgSettings>().unwrap(),
                   ArgSettings::Required);
        assert_eq!("takesvalue".parse::<ArgSettings>().unwrap(),
                   ArgSettings::TakesValue);
        assert_eq!("usevaluedelimiter".parse::<ArgSettings>().unwrap(),
                   ArgSettings::UseValueDelimiter);
        assert_eq!("valuedelimiternotset".parse::<ArgSettings>().unwrap(),
                   ArgSettings::ValueDelimiterNotSet);
        assert_eq!("requireequals".parse::<ArgSettings>().unwrap(),
                   ArgSettings::RequireEquals);
        assert_eq!("last".parse::<ArgSettings>().unwrap(), ArgSettings::Last);
        assert_eq!("hidedefaultvalue".parse::<ArgSettings>().unwrap(),
                   ArgSettings::HideDefaultValue);
        assert!("hahahaha".parse::<ArgSettings>().is_err());
    }
}
