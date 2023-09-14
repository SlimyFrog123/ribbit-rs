#[derive(Clone, Copy)]
pub struct Position {
    pub(crate) line: i32,
    pub(crate) start: i32,
    pub(crate) end: i32
}

impl Position {
    pub fn new(line: i32, start: i32, end: i32) -> Self {
        return Self {
            line,
            start,
            end
        };
    }

    pub fn new_single(line: i32, position: i32) -> Self {
        return Position::new(line, position, position);
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        if self.end == self.start {
            return "Line: ".to_owned() + &self.line.to_string() + ", Column: " + &self.start
                .to_string();
        }

        return "Line: ".to_owned() + &self.line.to_string() + ", Columns: " + &self.start
            .to_string() + "-" + &self.end.to_string();
    }
}
