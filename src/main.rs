use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Symbol {
    Dot,
    Dash,
    LetterSpace,
    WordSpace,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MorseCode {
    pub code: Vec<Symbol>,
}

impl MorseCode {
    pub fn new() -> Self {
        MorseCode { code: Vec::new() }
    }

    pub fn push(&mut self, symbol: Symbol) {
        self.code.push(symbol);
    }

    pub fn to_display_string(&self) -> String {
        let mut result = String::new();
        for symbol in &self.code {
            match symbol {
                Symbol::Dot => result.push('.'),
                Symbol::Dash => result.push('-'),
                Symbol::LetterSpace => result.push(' '),
                Symbol::WordSpace => result.push_str("   "),
            }
        }
        result
    }
}

lazy_static::lazy_static! {
    static ref MORSE_CODE_MAP: HashMap<char, Vec<Symbol>> = {
        let mut m = HashMap::new();
        m.insert('A', vec![Symbol::Dot, Symbol::Dash]);
        m.insert('B', vec![Symbol::Dash, Symbol::Dot, Symbol::Dot, Symbol::Dot]);
        m.insert('C', vec![Symbol::Dash, Symbol::Dot, Symbol::Dash, Symbol::Dot]);
        m.insert('D', vec![Symbol::Dash, Symbol::Dot, Symbol::Dot]);
        m.insert('E', vec![Symbol::Dot]);
        m.insert('F', vec![Symbol::Dot, Symbol::Dot, Symbol::Dash, Symbol::Dot]);
        m.insert('G', vec![Symbol::Dash, Symbol::Dash, Symbol::Dot]);
        m.insert('H', vec![Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dot]);
        m.insert('I', vec![Symbol::Dot, Symbol::Dot]);
        m.insert('J', vec![Symbol::Dot, Symbol::Dash, Symbol::Dash, Symbol::Dash]);
        m.insert('K', vec![Symbol::Dash, Symbol::Dot, Symbol::Dash]);
        m.insert('L', vec![Symbol::Dot, Symbol::Dash, Symbol::Dot, Symbol::Dot]);
        m.insert('M', vec![Symbol::Dash, Symbol::Dash]);
        m.insert('N', vec![Symbol::Dash, Symbol::Dot]);
        m.insert('O', vec![Symbol::Dash, Symbol::Dash, Symbol::Dash]);
        m.insert('P', vec![Symbol::Dot, Symbol::Dash, Symbol::Dash, Symbol::Dot]);
        m.insert('Q', vec![Symbol::Dash, Symbol::Dash, Symbol::Dot, Symbol::Dash]);
        m.insert('R', vec![Symbol::Dot, Symbol::Dash, Symbol::Dot]);
        m.insert('S', vec![Symbol::Dot, Symbol::Dot, Symbol::Dot]);
        m.insert('T', vec![Symbol::Dash]);
        m.insert('U', vec![Symbol::Dot, Symbol::Dot, Symbol::Dash]);
        m.insert('V', vec![Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dash]);
        m.insert('W', vec![Symbol::Dot, Symbol::Dash, Symbol::Dash]);
        m.insert('X', vec![Symbol::Dash, Symbol::Dot, Symbol::Dot, Symbol::Dash]);
        m.insert('Y', vec![Symbol::Dash, Symbol::Dot, Symbol::Dash, Symbol::Dash]);
        m.insert('Z', vec![Symbol::Dash, Symbol::Dash, Symbol::Dot, Symbol::Dot]);
        m.insert('0', vec![Symbol::Dash, Symbol::Dash, Symbol::Dash, Symbol::Dash, Symbol::Dash]);
        m.insert('1', vec![Symbol::Dot, Symbol::Dash, Symbol::Dash, Symbol::Dash, Symbol::Dash]);
        m.insert('2', vec![Symbol::Dot, Symbol::Dot, Symbol::Dash, Symbol::Dash, Symbol::Dash]);
        m.insert('3', vec![Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dash, Symbol::Dash]);
        m.insert('4', vec![Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dash]);
        m.insert('5', vec![Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dot]);
        m.insert('6', vec![Symbol::Dash, Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dot]);
        m.insert('7', vec![Symbol::Dash, Symbol::Dash, Symbol::Dot, Symbol::Dot, Symbol::Dot]);
        m.insert('8', vec![Symbol::Dash, Symbol::Dash, Symbol::Dash, Symbol::Dot, Symbol::Dot]);
        m.insert('9', vec![Symbol::Dash, Symbol::Dash, Symbol::Dash, Symbol::Dash, Symbol::Dot]);
        m.insert('.', vec![Symbol::Dot, Symbol::Dash, Symbol::Dot, Symbol::Dash, Symbol::Dot, Symbol::Dash]);
        m.insert(',', vec![Symbol::Dash, Symbol::Dash, Symbol::Dot, Symbol::Dot, Symbol::Dash, Symbol::Dash]);
        m.insert('?', vec![Symbol::Dot, Symbol::Dot, Symbol::Dash, Symbol::Dash, Symbol::Dot, Symbol::Dot]);
        m.insert('/', vec![Symbol::Dash, Symbol::Dot, Symbol::Dot, Symbol::Dash, Symbol::Dot]);
        m.insert('=', vec![Symbol::Dash, Symbol::Dot, Symbol::Dot, Symbol::Dot, Symbol::Dash]);
        m
    };

    static ref DECODE_MAP: HashMap<Vec<Symbol>, char> = {
        MORSE_CODE_MAP.iter().map(|(&k, v)| (v.clone(), k)).collect()
    };
}

fn encode(input: String) -> MorseCode {
    let mut encoded_msg: MorseCode = MorseCode::new();
    let mut first_word: bool = true;

    for word in input.to_uppercase().split_whitespace() {
        if !first_word {
            encoded_msg.push(Symbol::WordSpace);
        }
        first_word = false;

        let mut first_char_in_word = true;
        for ch in word.chars() {
            if !first_char_in_word {
                encoded_msg.push(Symbol::LetterSpace);
            }
            first_char_in_word = false;

            if let Some(morse_symbols) = MORSE_CODE_MAP.get(&ch) {
                for &symbol in morse_symbols {
                    encoded_msg.push(symbol);
                }
            }
        }
    }
    encoded_msg
}

fn decode(input: MorseCode) -> String {
    let mut decoded_string = String::new();
    let mut current_char_symbols: Vec<Symbol> = Vec::new();

    for symbol in input.code {
        match symbol {
            Symbol::Dot | Symbol::Dash => {
                current_char_symbols.push(symbol);
            }
            Symbol::LetterSpace => {
                if let Some(&char_val) = DECODE_MAP.get(&current_char_symbols) {
                    decoded_string.push(char_val);
                } else {
                    decoded_string.push('?');
                }
                current_char_symbols.clear();
            }
            Symbol::WordSpace => {
                if let Some(&char_val) = DECODE_MAP.get(&current_char_symbols) {
                    decoded_string.push(char_val);
                } else if !current_char_symbols.is_empty() {
                    decoded_string.push('?');
                }
                current_char_symbols.clear();
                decoded_string.push(' ');
            }
        }
    }
    if let Some(&char_val) = DECODE_MAP.get(&current_char_symbols) {
        decoded_string.push(char_val);
    } else if !current_char_symbols.is_empty() {
        decoded_string.push('?');
    }
    decoded_string
}

enum InputMode {
    Encode,
    Decode,
}

struct App {
    input: String,
    mode: InputMode,
    output: String,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            mode: InputMode::Encode,
            output: String::new(),
        }
    }

    fn update_output(&mut self) {
        match self.mode {
            InputMode::Encode => {
                let encoded = encode(self.input.clone());
                self.output = encoded.to_display_string();
            }
            InputMode::Decode => {
                let morse = self.parse_morse_input();
                self.output = decode(morse);
            }
        }
    }

    fn parse_morse_input(&self) -> MorseCode {
        let mut morse = MorseCode::new();
        let mut chars = self.input.chars().peekable();
        let mut space_count = 0;

        while let Some(ch) = chars.next() {
            match ch {
                '.' => {
                    morse.push(Symbol::Dot);
                    space_count = 0;
                }
                '-' => {
                    morse.push(Symbol::Dash);
                    space_count = 0;
                }
                ' ' => {
                    space_count += 1;
                    if space_count == 1 {
                        morse.push(Symbol::LetterSpace);
                    } else if space_count >= 3 {
                        if let Some(&Symbol::LetterSpace) = morse.code.last() {
                            morse.code.pop();
                        }
                        morse.push(Symbol::WordSpace);
                    }
                }
                _ => {}
            }
        }
        morse
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            InputMode::Encode => InputMode::Decode,
            InputMode::Decode => InputMode::Encode,
        };
        self.update_output();
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Tab => {
                        app.toggle_mode();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                        app.update_output();
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                        app.update_output();
                    }
                    KeyCode::Enter => {
                        app.input.push('\n');
                        app.update_output();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = match app.mode {
        InputMode::Encode => "Morse Code Encoder (Press Tab to switch to Decoder)",
        InputMode::Decode => "Morse Code Decoder (Press Tab to switch to Encoder)",
    };

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title_text = Paragraph::new(title)
        .block(title_block)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title_text, chunks[0]);

    let input_label = match app.mode {
        InputMode::Encode => "Input (Text)",
        InputMode::Decode => "Input (Morse: use . or - and spaces)",
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(input_label)
        .style(Style::default());

    let input = Paragraph::new(app.input.as_str())
        .block(input_block)
        .wrap(Wrap { trim: false });
    f.render_widget(input, chunks[1]);

    let arrow_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let arrow = Paragraph::new("↓")
        .block(arrow_block)
        .style(Style::default().fg(Color::Yellow))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(arrow, chunks[2]);

    let output_label = match app.mode {
        InputMode::Encode => "Output (Morse Code)",
        InputMode::Decode => "Output (Text)",
    };

    let output_block = Block::default()
        .borders(Borders::ALL)
        .title(output_label)
        .style(Style::default());

    let output = Paragraph::new(app.output.as_str())
        .block(output_block)
        .wrap(Wrap { trim: false });
    f.render_widget(output, chunks[3]);

    let help_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let help_text = Line::from(vec![
        Span::raw("Press "),
        Span::styled(
            "Tab",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" to switch modes | "),
        Span::styled(
            "Q",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" to quit"),
    ]);

    let help = Paragraph::new(help_text).block(help_block);
    f.render_widget(help, chunks[4]);
}
