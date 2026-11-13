#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub file_id: u32,
    pub offset:  u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Pos,
    pub end:   Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Loc {
    pub line: u32,
    pub col:  u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileId(u32);