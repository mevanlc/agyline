use std::io::{self, IsTerminal, Read};

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
    println!("    --install-themes          Install/reinstall default themes");
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
