pub struct ErrorReporter {
    pub had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn report(&mut self, line: u32, location: &str, message: &str) {
        println!("[line {line}] Error {location}: {message}");
        self.had_error = true;
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }
}
