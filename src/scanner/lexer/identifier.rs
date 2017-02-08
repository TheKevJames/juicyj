pub fn valid_continuation(c: char) -> bool {
    return c.is_alphabetic() || c.is_numeric() || c == '_';
}

pub fn valid_start(c: char) -> bool {
    return c.is_alphabetic() || c == '_';
}
