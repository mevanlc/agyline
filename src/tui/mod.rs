pub mod app;
pub mod editor_state;
pub mod layout;
pub mod widgets;

use crate::config::{manager, store::Store};
use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Paragraph, Wrap},
};
use std::io;

struct TerminalGuard;
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
    }
}
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let root = manager::config_dir();
    let mut loaded = Store::open(&root).map(App::new);
    enable_raw_mode()?;
    let _guard = TerminalGuard;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    let mut error_scroll = 0u16;
    loop {
        terminal.draw(|f|match &mut loaded {
            Ok(app)=>app.ui(f),
            Err(e)=>{
                let parts=ratatui::layout::Layout::vertical([ratatui::layout::Constraint::Min(0),ratatui::layout::Constraint::Length(1)]).split(f.area());
                f.render_widget(Paragraph::new(format!("{}\n\n{e}\n\nFix the catalog externally and reload. The original file has not been modified.",root.join("config.toml").display())).block(Block::bordered().title(" Catalog could not be loaded ")).wrap(Wrap {trim:false}).scroll((error_scroll,0)),parts[0]);
                f.render_widget(Paragraph::new("r reload  ↑↓ scroll  q quit"),parts[1]);
            }
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match &mut loaded {
                Ok(app) => {
                    app.handle_key(key.code, key.modifiers);
                    if app.should_quit {
                        break;
                    }
                }
                Err(_) => match key.code {
                    KeyCode::Char('r') => {
                        loaded = Store::open(&root).map(App::new);
                        error_scroll = 0;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Up => error_scroll = error_scroll.saturating_sub(1),
                    KeyCode::Down => error_scroll = error_scroll.saturating_add(1),
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
