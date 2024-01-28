use std::io::{self, stderr, Write};
use std::io::BufRead;
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};


#[derive(Debug)]
struct Keybinding {
    key_sequence: String,
    output: String
}

impl Keybinding {
    fn ui_row(&self) -> ratatui::widgets::Row<'static> {
        Row::new(vec![self.key_sequence.clone(), self.output.clone()])
    }
}

fn main() -> io::Result<()> {
    let mut pressed_keys = String::new();
    let mut output = String::new();
    let stdin_lines: Vec<Keybinding> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| match l.split_once(" ") {
            Some((a, b)) => Some((String::from(a), String::from(b))),
            _ => None
        })
        .map(|(key_sequence, output)| Keybinding {key_sequence , output})
        .collect();

    // raw mode disables terminal handling of inputs, sending them straight
    // to the application. For example, ctrl+c will not interrupt the execution.
    enable_raw_mode()?;
    stderr().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| ui(f, &pressed_keys, &stdin_lines))?;
        should_quit = handle_events(&mut pressed_keys, &stdin_lines, &mut output)?;
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    io::stdout()
        .write_all(output.as_bytes())
        .expect("Failed to write to output.");
    Ok(())
}

fn handle_events(pressed_keys: &mut String, bindings: &Vec<Keybinding>, output: &mut String) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => return Ok(true),
                    KeyCode::Backspace => {
                        pressed_keys.pop();
                        return Ok(false)
                    },
                    KeyCode::Char(c) => {
                        pressed_keys.push(c);
                        let m = bindings
                            .iter()
                            .filter(|k| &(k.key_sequence) == pressed_keys)
                            .collect::<Vec<&Keybinding>>();
                        match m.as_slice() {
                            [Keybinding {key_sequence: _, output: out}] => {
                                output.push_str(out);
                                return Ok(true)
                            },
                            _ => return Ok(false)
                        }
                    },
                    _ => return Ok(false)
                };
            };


       }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, pressed_keys: &str, matches: &Vec<Keybinding>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
                     Constraint::Length(3),
                     Constraint::Min(10),
        ])
        .split(frame.size());

    let table_rows: Vec<Row> = matches
        .iter()
        .filter(|k| k.key_sequence.starts_with(pressed_keys))
        .map(|k| k.ui_row())
        .collect();

    let table = Table::new(table_rows, &[Constraint::Length(5), Constraint::Length(25), Constraint::Length(10)])
        .block(Block::default().title("Matches").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1)
            )
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");
    

    frame.render_widget(
        Paragraph::new(pressed_keys)
            .block(Block::default().borders(Borders::ALL)),
        layout[0],
    );
    frame.render_widget(
        table,
        layout[1]
        );
}
