use crate::{
    block::Block,
    block::{self, Operation},
    file_api::FileApi,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Result {
    Ok,
    AllocError(i32, usize, i32),
    DeallocError(i32, usize, i32),
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::Ok => write!(f, "OK"),
            Result::AllocError(_, block_id, size) => {
                write!(f, "A;{};{}", block_id, size)
            }
            Result::DeallocError(_, block_id, reson) => {
                write!(f, "D;{};{}", block_id, reson)
            }
        }
    }
}
impl Result {
    pub fn get_id(&self) -> i32 {
        match self {
            Result::Ok => 0,
            Result::AllocError(id, _, _) => *id as i32,
            Result::DeallocError(id, _, _) => *id as i32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryManagement {
    pub max_bytes: i32,
    pub operations: Vec<Operation>,
    pub blocks_vec: Vec<Block>,
    pub file_api: FileApi,
    pub errors: Vec<Result>,
}

impl MemoryManagement {
    pub fn first_fit(&mut self) {
        self.blocks_vec.push(Block::new(self.max_bytes - 1));
        for op in self.operations.clone() {
            match op.operation {
                'A' => {
                    let mut j: Option<usize> = None;
                    for (i, block) in self.blocks_vec.iter().enumerate() {
                        if block.is_avalible() {
                            if self.blocks_vec[i].get_size() >= op.argument.unwrap() {
                                j = Some(i);
                                break;
                            }
                        }
                    }

                    match j {
                        Some(i) => self.alloc(i, op),
                        None => {
                            let max = self.get_max();
                            self.error(op.bl_id.unwrap(), op.id, max, 'A');
                        }
                    }
                }
                'D' => self.dealloc(op.bl_id, op.id),
                'O' => self.output("First fit"),
                'C' => self.compact(),
                _ => todo!(),
            }
        }
    }

    pub fn best_fit(&mut self) {
        self.blocks_vec.push(Block::new(self.max_bytes - 1));
        for op in self.operations.clone() {
            match op.operation {
                'A' => {
                    let mut j: Option<usize> = None;
                    for (i, block) in self.blocks_vec.iter().enumerate() {
                        if block.is_avalible() {
                            if block.get_size() >= op.argument.unwrap() {
                                match j {
                                    Some(idx) => {
                                        if block.get_size() < self.blocks_vec[idx].get_size() {
                                            j = Some(i);
                                        }
                                    }
                                    None => j = Some(i),
                                }
                            }
                        }
                    }

                    match j {
                        Some(i) => self.alloc(i, op),
                        None => {
                            let max = self.get_max();
                            self.error(op.bl_id.unwrap(), op.id, max, 'A');
                        }
                    }
                }
                'D' => self.dealloc(op.bl_id, op.id),
                'O' => self.output("Best fit"),
                'C' => self.compact(),
                _ => todo!(),
            }
        }
    }
    pub fn worst_fit(&mut self) {
        self.blocks_vec.push(Block::new(self.max_bytes - 1));
        for op in self.operations.clone() {
            match op.operation {
                'A' => {
                    let mut j: Option<usize> = None;
                    for (i, block) in self.blocks_vec.iter().enumerate() {
                        if block.is_avalible() {
                            if block.get_size() >= op.argument.unwrap() {
                                match j {
                                    Some(idx) => {
                                        if block.get_size() > self.blocks_vec[idx].get_size() {
                                            j = Some(i);
                                        }
                                    }
                                    None => j = Some(i),
                                }
                            }
                        }
                    }
                    match j {
                        Some(i) => self.alloc(i, op),
                        None => {
                            let max = self.get_max();
                            self.error(op.bl_id.unwrap(), op.id, max, 'A');
                        }
                    }
                }
                'D' => self.dealloc(op.bl_id, op.id),
                'O' => self.output("Worst fit"),
                'C' => self.compact(),
                _ => todo!(),
            }
        }
    }

    fn alloc(&mut self, i: usize, op: Operation) {
        let start = self.blocks_vec[i].start;
        let end = self.blocks_vec[i].end;
        self.blocks_vec[i].operation = Some(op);
        self.blocks_vec[i].set_range(start, start + op.argument.unwrap() - 1);
        if self.blocks_vec[i].end < end {
            let b = Block::new_empty(self.blocks_vec[i].end + 1, end);
            self.blocks_vec.insert(i + 1, b);
        }
        self.blocks_vec.sort_by(|a, b| a.start.cmp(&b.start));
    }

    fn error(&mut self, bl_id: i32, id: usize, par: i32, typ: char) {
        match typ {
            'A' => {
                self.errors.push(Result::AllocError(bl_id, id, par));
            }
            'D' => {
                self.errors.push(Result::DeallocError(bl_id, id, par));
            }
            _ => todo!(),
        }

        self.errors.dedup_by(|a, b| a == b);
    }

    fn output(&mut self, typ: &str) {
        self.file_api.out += 1;
        let (all, free) = self.print_block();
        self.file_api.write_file(
            true,
            typ,
            self.fragmentation(),
            all,
            free,
            self.errors.clone(),
        );
    }

    pub fn fragmentation(&self) -> f64 {
        let max = self.get_max();
        let sum: i32 = self
            .blocks_vec
            .iter()
            .filter(|block| block.is_avalible())
            .map(|block| block.get_size())
            .sum();

        (1.0 - (max as f64 / sum as f64)) as f64
    }

    fn exists(&self, id: i32) -> bool {
        self.blocks_vec
            .iter()
            .filter(|block| !block.is_avalible())
            .any(|block| block.operation.unwrap().bl_id.unwrap() == id)
    }

    fn compact(&mut self) {
        let mut last_start = 0;
        let mut temp: Vec<Block> = Vec::new();
        self.blocks_vec.sort_by(|a, b| a.start.cmp(&b.start));
        for block in self.blocks_vec.clone() {
            if !block.is_avalible() {
                let size = block.get_size();
                let b =
                    Block::new_full(last_start, last_start + size - 1, block.operation.unwrap());
                last_start = b.end + 1;
                temp.push(b);
            }
        }

        temp.push(Block::new_empty(last_start, self.max_bytes - 1));

        self.blocks_vec = temp;
    }

    fn join_blocks(&mut self) {
        'outer: for i in 1..self.blocks_vec.len() {
            if self.blocks_vec[i].is_avalible() && self.blocks_vec[i - 1].is_avalible() {
                let x = self.blocks_vec[i].clone();
                let start = self.blocks_vec[i - 1].start;
                if self.blocks_vec[i].is_avalible() && x.is_avalible() {
                    self.blocks_vec[i - 1].set_range(start, x.end);
                    self.blocks_vec.remove(i);
                    break 'outer;
                }
            }
        }
    }

    fn dealloc(&mut self, bl_id: Option<i32>, id: usize) {
        self.dealloc_err(bl_id, id);
        for i in 0..self.blocks_vec.len() {
            if self.blocks_vec[i].operation.is_some() {
                if self.blocks_vec[i].operation.unwrap().bl_id == bl_id {
                    self.blocks_vec[i].operation = None;
                }
            }
        }
        self.blocks_vec.sort_by(|a, b| a.start.cmp(&b.start));

        self.join_blocks();
    }

    fn dealloc_err(&mut self, bl_id: Option<i32>, id: usize) {
        let t = self
            .blocks_vec
            .iter()
            .filter(|block| !block.is_avalible())
            .any(|block| block.operation.unwrap().bl_id.unwrap() == bl_id.unwrap());
        if self.tried_alloc(bl_id) == 1 {
            self.error(bl_id.unwrap(), id, self.tried_alloc(bl_id), 'D');
        } else if !t {
            self.error(bl_id.unwrap(), id, self.tried_alloc(bl_id), 'D');
        }
    }

    fn tried_alloc(&self, bl_id: Option<i32>) -> i32 {
        let t = self.errors.iter().find(|e| match e {
            Result::AllocError(id, _, _) => id == &bl_id.unwrap(),
            _ => false,
        });
        match t {
            Some(Result::AllocError(_, _, _)) => 1,
            _ => 0,
        }
    }

    fn get_max(&self) -> i32 {
        let max = self
            .blocks_vec
            .iter()
            .filter(|block| block.is_avalible())
            .map(|block| block.get_size())
            .max();
        max.unwrap()
    }

    pub fn print_block(&self) -> (Vec<String>, Vec<String>) {
        let mut blocks = self
            .blocks_vec
            .iter()
            .filter(|block| !block.is_avalible())
            .collect::<Vec<_>>();
        blocks.sort_by(|a, b| a.operation.unwrap().bl_id.cmp(&b.operation.unwrap().bl_id));
        let b = blocks
            .iter()
            .map(|block| block.display_block())
            .collect::<Vec<_>>();
        let free = self
            .blocks_vec
            .iter()
            .filter(|block| block.is_avalible())
            .map(|block| block.display_block())
            .collect::<Vec<_>>();

        (b, free)
    }
}
