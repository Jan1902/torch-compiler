use crate::source::Source;

#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub position: usize,
}

pub struct ErrorReporter;

impl ErrorReporter {
    pub fn print(source: &Source, err: &CompileError) {
        let (line, col) = source.get_line_col(err.position);
        eprintln!(
            "error: {}\n --> {}:{}:{}",
            err.message, source.file_name, line, col
        );
    }
}