use crate::source_map::SourceMap;

#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub position: usize,
    pub source_id: usize,
}

pub struct ErrorReporter;

impl ErrorReporter {
    pub fn print(source_map: &SourceMap, err: &CompileError) {
        let source = &source_map.files[err.source_id];
        let (line, col) = source.get_line_col(err.position);
        eprintln!(
            "error: {}\n --> {}:{}:{}",
            err.message, source.file_name, line, col
        );
    }
}