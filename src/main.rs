use std::collections::HashMap;

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
            } else {
                eprintln!("Warning: Character '{}' not supported, skipping.", ch);
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
                    decoded_string.push_str("[UNKNOWN_CHAR]");
                    eprintln!("Warning: Unknown Morse sequence for characters, skipping.");
                }
                current_char_symbols.clear();
            }
            Symbol::WordSpace => {
                if let Some(&char_val) = DECODE_MAP.get(&current_char_symbols) {
                    decoded_string.push(char_val);
                } else if !current_char_symbols.is_empty() {
                    decoded_string.push_str("[UNKNOWN_CHAR]");
                    eprintln!("Warning: Unknown Morse sequence for character, skipping");
                }
                current_char_symbols.clear();
                decoded_string.push(' ');
            }
        }
    }
    if let Some(&char_val) = DECODE_MAP.get(&current_char_symbols) {
        decoded_string.push(char_val);
    } else if !current_char_symbols.is_empty() {
        decoded_string.push_str("[UNKNOWN_CHAR]");
        eprintln!("Warning: Unknown Morse sequence at the end of message");
    }
    decoded_string
}

fn show_morse_map() -> () {
    println!("---- Morse Code Map ----");
    for (char_val, morse_symbols) in MORSE_CODE_MAP.iter() {
        let morse_string: String = morse_symbols
            .iter()
            .map(|&s| match s {
                Symbol::Dot => '.',
                Symbol::Dash => '-',
                _ => '?',
            })
            .collect();
        println!("'{}': {}", char_val, morse_string);
    }
    println!("---- ----- ---- --- ----");
}

fn main() -> () {
    show_morse_map();

    let message = "Hello World";
    println!("Original message: {}", message);

    let encoded = encode(message.to_string());
    println!("Encoded Morse: {:?}", encoded);

    let decoded = decode(encoded);
    println!("Decoded message: {}", decoded);

    let message2 = "SOS. THIS IS A TEST";
    println!("\nOriginal message: {}", message2);
    let encoded2 = encode(message2.to_string());
    println!("Encoded Morse: {:?}", encoded2);
    let decoded2 = decode(encoded2);
    println!("Decoded message: {}", decoded2);

    let message3 = "RUST PROGRAMMING";
    println!("\nOriginal message: {}", message3);
    let encoded3 = encode(message3.to_string());
    println!("Encoded Morse: {:?}", encoded3);
    let decoded3 = decode(encoded3);
    println!("Decoded message: {}", decoded3);
}
