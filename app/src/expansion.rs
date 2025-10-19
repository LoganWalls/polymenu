use anyhow::{Context, Result, anyhow};
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub fn expand_path(path: &PathBuf) -> Result<String> {
    shellexpand::full(
        path.as_os_str()
            .to_str()
            .with_context(|| format!("could not convert path {path:?} into string"))?,
    )
    .map(|s| s.into_owned())
    .map_err(|e| {
        anyhow!(
            "Failed while expanding variable {}:\n{}",
            e.var_name,
            e.cause
        )
    })
    .context("could not expand path")
}

pub fn shell_expand<SI>(item: &SI, args: Option<&HashMap<String, String>>) -> Result<String>
where
    SI: AsRef<str> + ?Sized,
{
    shellexpand::full_with_context(item, home_dir, |s| env_expansion_context(s, args))
        .map(|s| s.into_owned())
        .map_err(|e| {
            anyhow!(
                "Failed while expanding variable {}:\n{}",
                e.var_name,
                e.cause
            )
        })
}

fn home_dir() -> Option<String> {
    dirs::home_dir().and_then(|s| s.into_os_string().into_string().ok())
}

fn env_expansion_context(
    s: &str,
    args: Option<&HashMap<String, String>>,
) -> Result<Option<Cow<'static, str>>> {
    if let Some(arg_map) = args
        && arg_map.contains_key(s)
    {
        return Ok(Some(arg_map.get(s).unwrap().to_owned().into()));
    }

    match env::var(s) {
        Ok(value) => Ok(Some(value.into())),
        Err(env::VarError::NotPresent) => Err(anyhow!(
            "{s} is not an environment variable, and was not passed as a command argument"
        )),
        Err(env::VarError::NotUnicode(u)) => Err(anyhow!("Non-unicode values: {u:?} found in {s}")),
    }
    .context("Variable expansion failed")
}
