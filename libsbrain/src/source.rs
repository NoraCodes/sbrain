use machine::*;

enum ParserState {
    Code,
    Data,
    Comment,
    ExpectingAt,
}

/// Given a character, turn it into a SBrainVM instruction
fn char_to_instruction(character: char) -> Option<u8> {
    match character {
        '<' => Some(0),
        '>' => Some(1),
        '-' => Some(2),
        '+' => Some(3),
        '[' => Some(4),
        ']' => Some(5),
        '.' => Some(6),
        ',' => Some(7),
        '{' => Some(8),
        '}' => Some(9),
        '(' => Some(10),
        ')' => Some(11),
        'z' => Some(12),
        '!' => Some(13),
        's' => Some(14),
        'S' => Some(15),
        '|' => Some(16),
        '&' => Some(17),
        '*' => Some(18),
        '^' => Some(19),
        '$' => Some(20),
        'a' => Some(21),
        'd' => Some(22),
        'q' => Some(23),
        'm' => Some(24),
        'p' => Some(25),
        '@' => Some(31),
        _ => None,
    }
}

/// Given source code, create data and instruction tapes.
pub fn source_to_tapes(source: &str) -> (Vec<u8>, Vec<u32>) {
    // Strip out comments. Anything between # goes.
    // Data gets turned into u32s and put into data
    // Code gets turned into u8s and put into code

    let mut code: Vec<u8> = Vec::new();
    let mut data: Vec<u32> = Vec::new();

    // Code is the default state.
    let mut state: ParserState = ParserState::Code;

    for character in source.chars() {
        match state {
            ParserState::Code => {
                if character == '#' {
                    state = ParserState::Comment;
                } else {
                    if character == '@' {
                        state = ParserState::ExpectingAt;
                    }
                    match char_to_instruction(character) {
                        None => {}
                        Some(n) => code.push(n),
                    };
                }
            }
            ParserState::Data => {
                data.push(character as u32);
            }
            ParserState::Comment => {
                if character == '#' {
                    state = ParserState::Code;
                }
            }
            ParserState::ExpectingAt => {
                if character == '@' {
                    state = ParserState::Data;
                } else {
                    match char_to_instruction(character) {
                        None => {}
                        Some(n) => code.push(n),
                    };
                }
            }
        };
    }
    return (code, data);
}
