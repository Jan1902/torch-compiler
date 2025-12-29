use crate::source::Source;

pub struct SourceMap {
    pub files: Vec<Source>,
}

impl SourceMap {
    pub fn new() -> Self { Self { files: Vec::new() } }
    
    pub fn add(&mut self, source: Source) -> usize {
        self.files.push(source);
        self.files.len() - 1
    }

    pub fn add_from_file(&mut self, file_name: &String) -> std::io::Result<usize> {
        let content = std::fs::read_to_string(file_name)?;
        let source = Source::new(content, file_name.clone());
        Ok(self.add(source))
    }
}