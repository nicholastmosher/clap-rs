* 'a -> 'key
* 'n -> 'key
* 'b -> 'other
* 'e -> 'other
* Removed deprecated App::with_defaults
* Removed App::help_short
* Removed App::help_message
* Removed App::version_short
* Removed App::version_message
* Restructure code
* Args takes IntoIterator<Item=Arg>
* Moved completion gen to completions mod
* Arg::set -> Arg::setting
* Arg::unset -> Arg::unset_setting
* Add Arg::settings
* Add Arg::unset_settings
* Remove Arg::next_line_help
* Remove Arg::require_value_delimiter
* Remove Arg::use_value_delimiter
* Remove Arg::hidden
* Remove Arg::empty_values
* Remove Arg::multiple
* Remove Arg::hide_default_values
* Remove Arg::hide_possible_values
* Remove Arg::takes_value
* Remove Arg::allow_hyphen_values
* Remove Arg::require_equals
* Remove Arg::required
* Remove Arg::last
* Add ArgSettings::MultipleValues
* Add ArgSettings::MultipleOccurrences
* Arg::validator_os returns String