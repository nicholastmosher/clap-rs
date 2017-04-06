#[derive(Clone. Debug, Default)]
pub struct ConditionalReqirement<'key, 'other> {
    if_arg: &'key str,
    if_val: &'other str,
    required: &'key str,
}