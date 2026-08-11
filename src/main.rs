use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;

const CONFIG_DIR_ERROR: &str = "--config-dir requires a non-empty path";

fn print_help() {
    println!("xline {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    xline                     Launch TUI theme editor");
    println!("    <json> | xline            Render status line from JSON input");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help                Print this help message");
    println!("    -V, --version             Print version");
    println!("    --config-dir <path>       Use a different xline config directory");
    println!("    --install-themes          Install/reinstall default themes");
    println!();
    println!("ENVIRONMENT:");
    println!("    XLINE_CONFIG_DIR          Override the xline config directory");
}

fn config_dir_arg(args: &[String]) -> Result<Option<PathBuf>, &'static str> {
    let mut config_dir = None;
    let mut index = 0;

    while index < args.len() {
        if args[index] == "--config-dir" {
            let Some(value) = args.get(index + 1) else {
                return Err(CONFIG_DIR_ERROR);
            };
            if value.is_empty() || value.starts_with('-') {
                return Err(CONFIG_DIR_ERROR);
            }
            config_dir = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        if let Some(value) = args[index].strip_prefix("--config-dir=") {
            if value.is_empty() {
                return Err(CONFIG_DIR_ERROR);
            }
            config_dir = Some(PathBuf::from(value));
        }

        index += 1;
    }

    Ok(config_dir)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("xline {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let config_dir = match config_dir_arg(&args) {
        Ok(path) => path,
        Err(message) => {
            eprintln!("xline: {message}");
            std::process::exit(2);
        }
    };
    if let Some(path) = config_dir {
        xline::config::manager::set_config_dir_override(path)
            .expect("config directory override was already set");
    }

    // Handle --install-themes
    if args.iter().any(|a| a == "--install-themes") {
        let dir = xline::config::manager::themes_dir();
        if let Err(e) = std::fs::create_dir_all(&dir) {
            eprintln!("xline: cannot create themes dir: {}", e);
            std::process::exit(1);
        }
        match xline::config::manager::write_default_themes(&dir, true) {
            Ok(n) => {
                eprintln!(
                    "xline: installed {} default theme(s) to {}",
                    n,
                    dir.display()
                );
            }
            Err(e) => {
                eprintln!("xline: error installing themes: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Bootstrap: ensure themes dir exists with starter themes
    if let Err(e) = xline::config::manager::bootstrap() {
        eprintln!("xline: bootstrap error: {}", e);
        std::process::exit(1);
    }

    let stdin_is_terminal = io::stdin().is_terminal();

    if stdin_is_terminal {
        // No stdin data → launch TUI editor
        if let Err(e) = xline::tui::run() {
            eprintln!("xline: TUI error: {}", e);
            std::process::exit(1);
        }
    } else {
        // Stdin has data → statusline rendering mode
        run_statusline();
    }
}

fn run_statusline() {
    // Read JSON from stdin
    let mut input_str = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input_str) {
        eprintln!("xline: stdin read error: {}", e);
        std::process::exit(1);
    }

    let input: xline::core::input::InputData = match serde_json::from_str(&input_str) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("xline: JSON parse error: {}", e);
            std::process::exit(1);
        }
    };

    // Load active theme
    let (_name, _path, theme) = match xline::config::manager::load_active_theme() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("xline: theme load error: {}", e);
            std::process::exit(1);
        }
    };

    // Collect component data and render
    let components = xline::core::statusline::collect_all_components(&theme, &input);
    let generator = xline::core::statusline::StatusLineGenerator::new(&theme);
    let statusline = generator.generate(components);

    print!("{}\x1b[0m", statusline);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn config_dir_arg_accepts_separate_value() {
        assert_eq!(
            config_dir_arg(&args(&["--config-dir", "/tmp/xline"])),
            Ok(Some(PathBuf::from("/tmp/xline")))
        );
    }

    #[test]
    fn config_dir_arg_accepts_equals_value() {
        assert_eq!(
            config_dir_arg(&args(&["--config-dir=/tmp/xline"])),
            Ok(Some(PathBuf::from("/tmp/xline")))
        );
    }

    #[test]
    fn config_dir_arg_uses_last_value() {
        assert_eq!(
            config_dir_arg(&args(&[
                "--config-dir=/tmp/first",
                "--config-dir",
                "/tmp/second",
            ])),
            Ok(Some(PathBuf::from("/tmp/second")))
        );
    }

    #[test]
    fn config_dir_arg_rejects_missing_or_empty_value() {
        assert_eq!(
            config_dir_arg(&args(&["--config-dir"])),
            Err(CONFIG_DIR_ERROR)
        );
        assert_eq!(
            config_dir_arg(&args(&["--config-dir="])),
            Err(CONFIG_DIR_ERROR)
        );
        assert_eq!(
            config_dir_arg(&args(&["--config-dir", "--install-themes"])),
            Err(CONFIG_DIR_ERROR)
        );
    }

    #[test]
    fn config_dir_arg_ignores_unrelated_arguments() {
        assert_eq!(config_dir_arg(&args(&["--install-themes"])), Ok(None));
    }
}
