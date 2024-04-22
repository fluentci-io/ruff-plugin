use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "install", &format!("ruff@{}", version)])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn check(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "ruff", "check", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn clean(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "ruff", "clean", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn format(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "ruff", "format", &args])?
        .stdout()?;
    Ok(stdout)
}
