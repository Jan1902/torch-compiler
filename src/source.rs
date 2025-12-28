pub struct Source {
    pub content: String,
    pub line_starts: Vec<usize>,
    pub file_name: String,
}

impl Source {
    pub fn new(content: String, file_name: String) -> Self {
        let mut line_starts = vec![0];
        for (i, c) in content.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        Source { content, line_starts, file_name }
    }

    pub fn get_line_col(&self, position: usize) -> (usize, usize) {
        let line = match self.line_starts.binary_search(&position) {
            Ok(line) => line,
            Err(line) => line - 1,
        };
        let col = position - self.line_starts[line];
        (line + 1, col + 1)
    }
}