mod app;
mod navigation;
mod shell;
mod shortcuts;
mod ui;

use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use app::{App, AppMode};
use shell::{
    executor::{execute_command, resolve_cd_path},
    parser::{parse_command, ParsedCommand},
};
use ui::{render_navigator, render_sidebar, render_terminal};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                AppMode::Normal => handle_normal_mode(app, key.code, key.modifiers),
                AppMode::NavigationList => handle_navigation_mode(app, key.code),
                AppMode::ShortcutSelection => handle_goto_mode(app, key.code),
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn draw_ui(f: &mut ratatui::Frame, app: &mut App) {
    let size = f.size();

    // Main layout: sidebar on left, terminal on right
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(25), // Sidebar (fixed width)
            Constraint::Min(40),    // Terminal (flexible)
        ])
        .split(size);

    // Always render sidebar first (left side), passing selection info if in goto mode
    let selected_index = if app.mode == AppMode::ShortcutSelection {
        Some(app.selected_shortcut_index)
    } else {
        None
    };
    render_sidebar(f, main_chunks[0], &app.shortcuts, selected_index);

    // Render terminal/navigator based on mode (right side)
    match app.mode {
        AppMode::Normal => {
            render_terminal(f, main_chunks[1], app);
        }
        AppMode::NavigationList => {
            // In navigation mode, show navigator in the terminal area
            render_navigator(f, main_chunks[1], &mut app.navigation_state);
        }
        AppMode::ShortcutSelection => {
            // In goto mode, still show terminal but highlight sidebar
            render_terminal(f, main_chunks[1], app);
        }
    }
}

fn handle_normal_mode(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    match (code, modifiers) {
        // Ctrl+1 through Ctrl+9 - navigate to shortcut
        (KeyCode::Char(c), KeyModifiers::CONTROL) if ('1'..='9').contains(&c) => {
            let index = c.to_digit(10).unwrap() as usize;
            if let Some(shortcut) = app.shortcuts.get_shortcut(index) {
                let path = shortcut.path.clone();
                if path.is_dir() {
                    app.add_output(&format!("cd {}", path.display()));
                    app.current_dir = path.clone();
                    app.shortcuts.touch_shortcut(&path);
                } else {
                    app.add_output(&format!("Error: {} no longer exists", path.display()));
                }
            }
        }

        // Ctrl+C - cancel/clear
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            if app.input.is_empty() {
                app.should_quit = true;
            } else {
                app.add_output(&format!("{}{}^C", app.prompt(), app.input));
                app.clear_input();
            }
        }

        // Ctrl+D - exit
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            if app.input.is_empty() {
                app.should_quit = true;
            }
        }

        // Ctrl+L - clear screen
        (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
            app.output.clear();
        }

        // Ctrl+A - move to start
        (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
            app.cursor_home();
        }

        // Ctrl+E - move to end
        (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
            app.cursor_end();
        }

        // Ctrl+U - clear line
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            app.clear_input();
        }

        // Enter - execute command
        (KeyCode::Enter, _) => {
            execute_input(app);
        }

        // Backspace - delete character
        (KeyCode::Backspace, _) => {
            app.delete_char();
        }

        // Left arrow - move cursor left
        (KeyCode::Left, _) => {
            app.cursor_left();
        }

        // Right arrow - move cursor right
        (KeyCode::Right, _) => {
            app.cursor_right();
        }

        // Up arrow - history previous
        (KeyCode::Up, _) => {
            app.history_prev();
        }

        // Down arrow - history next
        (KeyCode::Down, _) => {
            app.history_next();
        }

        // Home - move to start
        (KeyCode::Home, _) => {
            app.cursor_home();
        }

        // End - move to end
        (KeyCode::End, _) => {
            app.cursor_end();
        }

        // Tab - could be used for autocomplete later
        (KeyCode::Tab, _) => {
            // TODO: Implement tab completion
        }

        // Escape - clear input
        (KeyCode::Esc, _) => {
            app.clear_input();
        }

        // Regular character input
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
            app.insert_char(c);
        }

        _ => {}
    }
}

fn handle_navigation_mode(app: &mut App, code: KeyCode) {
    match code {
        // Up - move selection up
        KeyCode::Up => {
            app.navigation_state.move_up();
        }

        // Down - move selection down
        KeyCode::Down => {
            app.navigation_state.move_down();
        }

        // Right - enter selected directory
        KeyCode::Right => {
            app.navigation_state.enter_selected();
        }

        // Left - go up one level
        KeyCode::Left => {
            app.navigation_state.go_up();
        }

        // Enter - confirm selection
        KeyCode::Enter => {
            if let Some(path) = app.navigation_state.get_selected_path() {
                app.add_output(&format!("cd {}", path.display()));
            }
            app.confirm_navigation();
        }

        // Escape - cancel navigation
        KeyCode::Esc => {
            app.exit_navigation_mode();
        }

        _ => {}
    }
}

fn handle_goto_mode(app: &mut App, code: KeyCode) {
    match code {
        // Up - move selection up
        KeyCode::Up => {
            app.goto_move_up();
        }

        // Down - move selection down
        KeyCode::Down => {
            app.goto_move_down();
        }

        // Enter - confirm selection and navigate
        KeyCode::Enter => {
            app.confirm_goto();
        }

        // Escape - cancel goto mode
        KeyCode::Esc => {
            app.exit_goto_mode();
        }

        _ => {}
    }
}

fn execute_input(app: &mut App) {
    let input = app.input.clone();
    app.add_command_to_output(&input);
    app.add_to_history(&input);
    app.clear_input();

    match parse_command(&input) {
        ParsedCommand::Empty => {
            // Do nothing for empty commands
        }

        ParsedCommand::Cd(path) => {
            let target = path.as_deref().unwrap_or("~");
            match resolve_cd_path(target, &app.current_dir) {
                Ok(new_path) => {
                    app.current_dir = new_path;
                }
                Err(e) => {
                    app.add_output(&format!("cd: {}", e));
                }
            }
        }

        ParsedCommand::CdList => {
            app.enter_navigation_mode();
        }

        ParsedCommand::Clear => {
            app.output.clear();
        }

        ParsedCommand::Exit => {
            app.should_quit = true;
        }

        ParsedCommand::JermSave => {
            app.shortcuts.add_shortcut(app.current_dir.clone());
            app.add_output(&format!("Shortcut saved: {}", app.current_dir.display()));
        }

        ParsedCommand::JermGoto => {
            app.enter_goto_mode();
        }

        ParsedCommand::Shell(cmd) => match execute_command(&cmd, &app.current_dir) {
            Ok(result) => {
                for line in result.all_lines() {
                    app.add_output(&line);
                }
            }
            Err(e) => {
                app.add_output(&format!("Error: {}", e));
            }
        },
    }
}
