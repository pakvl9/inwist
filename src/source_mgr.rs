use crate::base::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    name:   String,
    src:    String,
    line_s: Vec<u32>,
}

impl File {
    pub fn new(name: impl Into<String>, src: impl Into<String>) -> Self {
        let mut line_s = vec![0];
        let content = src.into();

        let mut pos = 0u32;

        for i in content.chars() {
            pos += 1;
            if i == '\n' {
                line_s.push(pos);
            }
        } 

        Self {
            name: name.into(),
            src:  content,
            line_s: line_s,
        }
    }

    pub fn src(&self) -> &String {
        &self.src
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

pub struct SrcMgr {
    files: Vec<File>,
}

impl SrcMgr {
    pub fn new() -> Self{
        Self {
            files: Vec::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, src: impl Into<String>) -> u32 {
        let file: File = File::new(name, src);
        self.files.push(file);

        (self.files.len() - 1) as u32
    }

    pub fn get(&self, id: u32) -> Option<&File> {
        self.files.iter().nth(id as usize)
    }

    pub fn loc(&self, pos: Pos) -> Option<Loc> {
        let file = self.get(pos.file_id)?;

        let mut it= u32::MAX;
        for i in file.line_s {
            if i >= pos.offset {
                it = i-1;
            }
        }
        if it == u32::MAX && !file.line_s.is_empty() {
            it = (file.line_s.len() - 1) as u32;
        }

        let line = it + 1;
        let col  = pos.offset - file.line_s[it];

        Some(Loc{line, col})
    }
}