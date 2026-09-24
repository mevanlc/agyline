use agyline::config::{
    catalog::{Catalog, ResourceRef},
    store::Store,
};
use std::io::Write;
use std::process::{Command, Stdio};
fn run(root: &std::path::Path, args: &[&str], input: Option<&str>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_agyline"));
    command
        .arg("--config-dir")
        .arg(root)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().unwrap();
    if let Some(input) = input {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
    }
    child.wait_with_output().unwrap()
}
const INPUT: &str = r#"{"model":{"id":"gemini-3.7-flash","display_name":"Gemini 3.7 Flash","effort":"high"},"workspace":{"current_dir":"/tmp","project_dir":"/tmp"},"vcs":{"branch":"main","dirty":false},"agent_state":"idle"}"#;
#[test]
fn rendering_and_validation_do_not_bootstrap_and_ignore_old_files() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("absent");
    let out = run(&root, &[], Some(INPUT));
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!out.stdout.is_empty());
    assert!(!root.exists());
    let out = run(&root, &["--validate-config"], None);
    assert!(out.status.success());
    assert!(!root.exists());
    std::fs::create_dir_all(root.join("themes")).unwrap();
    std::fs::write(
        root.join("themes/old.toml"),
        "active = true\nnot even toml!",
    )
    .unwrap();
    assert!(run(&root, &[], Some(INPUT)).status.success());
    assert!(!root.join("config.toml").exists());
    assert_eq!(
        std::fs::read_to_string(root.join("themes/old.toml")).unwrap(),
        "active = true\nnot even toml!"
    );
}
#[test]
fn invalid_catalog_and_obsolete_or_unknown_flags_fail_without_overwrite() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    for contents in [
        "bad toml",
        "schema_version=999\n[active_theme]\nsource='builtin'\nname='Default'",
    ] {
        std::fs::write(&path, contents).unwrap();
        let out = run(dir.path(), &[], Some(INPUT));
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
        assert!(!out.stderr.is_empty());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), contents);
    }
    for arg in ["--install-themes", "--bogus"] {
        let out = run(dir.path(), &[arg], None);
        assert_eq!(out.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&out.stderr).contains("Unknown argument"));
    }
}
#[test]
fn active_saved_references_control_next_invocation() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = Store::open(dir.path()).unwrap();
    let mut c = Catalog::default();
    let first = run(dir.path(), &[], Some(INPUT));
    c.active_theme = ResourceRef::builtin("Powerline Dark");
    store.save(&c).unwrap();
    let second = run(dir.path(), &[], Some(INPUT));
    assert!(second.status.success());
    assert_ne!(first.stdout, second.stdout);
    assert!(String::from_utf8_lossy(&second.stdout).contains('\u{e0b0}'));
}

#[test]
fn documented_example_catalog_validates() {
    let text = include_str!("../examples/config.toml");
    let c: Catalog = toml::from_str(text).unwrap();
    c.validate().unwrap();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("config.toml"), text).unwrap();
    assert!(
        run(dir.path(), &["--validate-config"], None)
            .status
            .success()
    );
}
