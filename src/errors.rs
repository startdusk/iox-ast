pub enum LoxError {}

impl LoxError {
    pub fn error(line: usize, message: impl Into<String>) {
        Self::report(line, "", message)
    }

    pub fn report(line: usize, position: impl Into<String>, message: impl Into<String>) {
        println!(
            "[line {}] Error: {}\n\t{}",
            line,
            message.into(),
            position.into(),
        );
    }
}
