pub fn valid_continuation(character: char) -> bool {
    return character.is_alphabetic() || character.is_numeric() || character == '_'
}

pub fn valid_start(character: char) -> bool {
    return character.is_alphabetic() || character == '_'
}
