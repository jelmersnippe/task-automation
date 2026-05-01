use crate::{
    fs::get_absolute_path,
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::{DictArgs, OptionalValue},
        dictionary::DictionaryDeclaration,
    },
};

pub struct ResolvedCmd {
    pub cwd: String,
    pub cmd: Option<String>,
}

impl ResolvedCmd {
    /// Produces `cd '<cwd>'[ && <cmd>] && exec $SHELL`. Used by shell.open so
    /// the terminal stays alive after an optional one-off command finishes.
    pub fn to_open_cmd(&self) -> String {
        let mut s = format!("cd '{}'", self.cwd);
        if let Some(cmd) = &self.cmd {
            s.push_str(&format!(" && {}", cmd));
        }
        s.push_str(" && exec $SHELL");
        s
    }

    /// Produces `cd '<cwd>' && <cmd>`. Used by shell.run where the terminal
    /// closes once the command exits.
    pub fn to_run_cmd(&self) -> Option<String> {
        self.cmd
            .as_ref()
            .map(|cmd| format!("cd '{}' && {}", self.cwd, cmd))
    }
}

pub fn resolve_cmd(
    dict: &DictionaryDeclaration,
    fn_name: &str,
    fallback_cwd: &str,
) -> Result<ResolvedCmd, ExecutionError> {
    let dict_args = DictArgs::new(fn_name, dict);

    let cwd = dict_args
        .string("cwd")
        .optional()?
        .unwrap_or_else(|| fallback_cwd.to_string());

    let absolute_cwd = get_absolute_path(&cwd)
        .map_err(|e| ExecutionError::new(CallInfo::new(fn_name), &e.to_string()))?;

    let cmd = dict_args.string("cmd").optional()?;

    Ok(ResolvedCmd {
        cwd: absolute_cwd,
        cmd,
    })
}
