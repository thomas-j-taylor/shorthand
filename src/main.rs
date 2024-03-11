use std::io::{self, stderr, Write};
use std::io::BufRead;
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};
use clap::Parser;

/// Menu selection like fzf, but with user defined key sequences
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Display the keys which have been pressed
    #[arg(short, long, default_value_t = false)]
    show_pressed_keys: bool,
}

struct Keybinding {
    key_sequence: String,
    output: String,
    comment: String
}

impl Keybinding {
    fn ui_row<'a>(&'a self, dimmed_len: usize) -> ratatui::widgets::Row<'a> {
        let (dimmed_part, non_dimmed_part) = self.key_sequence.split_at(dimmed_len);
        let key_sequence_line = Line::from(vec![
            Span::styled(dimmed_part, Style::default().fg(Color::Gray).add_modifier(Modifier::DIM)),
            Span::raw(non_dimmed_part)
        ]);
        Row::new(vec![key_sequence_line, self.output.clone().into(), self.comment.clone().into()])
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut pressed_keys = String::new();
    let mut output = String::new();
    let stdin_lines: Vec<Keybinding> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| match l.split_once(" ") {
            Some((a, b)) => {
                match b.split_once(" #") {
                    Some((content, comment)) => Some((String::from(a),String::from(content),String::from(comment))),
                    _ => Some((String::from(a),String::from(b),String::from("")))
                }
            }
            _ => None
        })
        .map(|(key_sequence, output, comment)| Keybinding {key_sequence , output, comment})
        .collect();

    // raw mode disables terminal handling of inputs, sending them straight
    // to the application. For example, ctrl+c will not interrupt the execution.
    enable_raw_mode()?;
    stderr().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| ui(f, &pressed_keys, &stdin_lines, &args))?;
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
                            [Keybinding {key_sequence: _, output: out, ..}] => {
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

fn ui(frame: &mut Frame, pressed_keys: &str, matches: &Vec<Keybinding>, args: &Args) {

    let layout_constraints = match args.show_pressed_keys {
        true => vec![Constraint::Length(3), Constraint::Min(10)],
        false => vec![Constraint::Min(10)]
    };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(layout_constraints)
        .split(frame.size());

    let table_rows: Vec<Row> = matches
        .iter()
        .filter(|k| k.key_sequence.starts_with(pressed_keys))
        .map(|k| k.ui_row(pressed_keys.len()))
        .enumerate()
        .map(|(n,r)| match n % 2 {
            0 => r.style(Style::default().bg(Color::Rgb(20,20,20))),
            1 => r.style(Style::default().bg(Color::Black)),
            _ => r
        })
        .collect();

    let col_widths: Vec<u16> = vec![
        matches.iter().map(|k| k.key_sequence.len()).max().unwrap() as u16,
        matches.iter().map(|k| k.output.len()).max().unwrap() as u16,
        matches.iter().map(|k| k.comment.len()).max().unwrap() as u16,
    ];

    let table = Table::new(table_rows, &[Constraint::Length(col_widths[0]), Constraint::Max(col_widths[1]), Constraint::Min(col_widths[2])])
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["", "command", "description"])
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1)
            )
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");
    

    match args.show_pressed_keys {
        true => {
            frame.render_widget(
                Paragraph::new(pressed_keys)
                .block(Block::default().borders(Borders::ALL)),
                layout[0],
                );
            frame.render_widget(
                table,
                layout[1]
                );
        },
        false => {
            frame.render_widget(
                table,
                layout[0]
                );
        }
    }
}
