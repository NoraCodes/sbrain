enum ParserState {
    Code,
    Comment,
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
        '^' => Some(12),
        '!' => Some(13),
        '&' => Some(14),
        '@' => Some(15),
        _ => None,
    }
}

/// Transliterate a source code into the corresponding instructions.
pub fn source_to_tape(source: &str) -> Vec<u8> {
    // Strip out comments. Anything between # goes.
    // Code gets turned into u8s

    let mut code: Vec<u8> = Vec::new();

    let mut state: ParserState = ParserState::Code;

    for character in source.chars() {
        match state {
            ParserState::Code => {
                if character == '#' {
                    state = ParserState::Comment;
                } else {
                    match char_to_instruction(character) {
                        None => {}
                        Some(n) => code.push(n),
                    };
                }
            }
            ParserState::Comment => {
                if character == '#' {
                    state = ParserState::Code;
                }
            }
        };
    }
    return code;
}
