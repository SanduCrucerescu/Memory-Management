use std::slice::SliceIndex;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub start: i32,
    pub end: i32,
    pub operation: Option<Operation>,
}

impl Block {
    pub fn new(size: i32) -> Block {
        Block {
            start: 0,
            end: size,
            operation: None,
        }
    }

    pub fn new_full(new_start: i32, new_end: i32, op: Operation) -> Block {
        Block {
            start: new_start,
            end: new_end,
            operation: Some(op),
        }
    }

    pub fn new_empty(new_start: i32, new_end: i32) -> Block {
        Block {
            start: new_start,
            end: new_end,
            operation: None,
        }
    }

    pub fn get_size(&self) -> i32 {
        //self.hole.get_size()
        (self.end - self.start) + 1
    }

    pub fn is_avalible(&self) -> bool {
        self.operation.is_none()
    }

    pub fn set_range(&mut self, start: i32, end: i32) {
        self.start = start;
        self.end = end;
    }

    pub fn can_be_placed(&self, new_operation: Operation) -> bool {
        self.is_avalible()
            && new_operation.operation == 'A'
            && new_operation.argument <= Some(self.get_size())
    }

    pub fn display_block(&self) -> String {
        let start = self.start;
        let end = self.end;
        let mut al: String = Default::default();
        let mut op: String = Default::default();

        if !self.is_avalible() {
            al = format!("{}", self.operation.unwrap().bl_id.unwrap()).to_string();
            op = format!("{};{};{}", al, start, end);
        } else {
            op = format!("{};{}", start, end);
        }
        op
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub id: usize,
    pub bl_id: Option<i32>,
    pub operation: char,
    pub argument: Option<i32>,
}
