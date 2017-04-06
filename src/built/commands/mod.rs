use std::mem;

#[doc(hidden)]
#[derive(Clone, Default)]
pub struct Command<'key, 'other>
    where 'key: 'other
{
    meta: AppMeta<'other>,
    settings: AppFlags,
    g_settings: AppFlags,
    flags: Vec<FlagBuilder<'key, 'other>>,
    options: Vec<OptBuilder<'key, 'other>>,
    positionals: VecMap<PosBuilder<'key, 'other>>,
    subcommands: Vec<Command<'key, 'other>>,
    groups: Vec<ArgGroup<'key>>,
    args: Vec<Arg<'key, 'other>>,
}

impl<'key. 'other> From<App<'key, 'other>> for Command<'key, 'other> {
    fn from(mut app: App<'key, 'other>) -> Self {
        Command {
            meta: mem::replace(&mut app.meta, AppMeta::default()),
            settings: app.settings,
            g_settings: app.g_settings,
            args: mem::replace(&mut app.args, vec![]),
            groups: mem::replace(&mut app.groups, vec![]),
            .. Command::default()
        }
    }
}