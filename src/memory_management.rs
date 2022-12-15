use crate::{block::Block, block::Operation, file_api::FileApi};

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
                    for i in 0..self.blocks_vec.len() {
                        self.alloc(i, op)
                    }
                }
                'D' => self.dealloc(op.bl_id, op.id),
                'O' => self.output("First fit"),
                'C' => println!("test"),
                _ => todo!(),
            }
        }
    }

    pub fn best_fit(&mut self) {
        self.blocks_vec.push(Block::new(self.max_bytes - 1));
        for op in self.operations.clone() {
            match op.operation {
                'A' => {
                    for i in 0..self.blocks_vec.len() {
                        let max = self
                            .blocks_vec
                            .iter()
                            .filter(|block| block.is_avalible())
                            .map(|block| block.get_size())
                            .max();
                        if self.blocks_vec[i].get_size() >= op.argument.unwrap() {
                            self.alloc(i, op)
                        } else if max.unwrap() < op.argument.unwrap() {
                            self.error(op.bl_id.unwrap(), op.id, max.unwrap(), 'A')
                        }
                    }
                }
                'D' => self.dealloc(op.bl_id, op.id),
                'O' => self.output("Best fit"),
                'C' => println!("test"),
                _ => todo!(),
            }
        }
    }
    pub fn worst_fit(&mut self) {
        self.blocks_vec.push(Block::new(self.max_bytes - 1));
        for op in self.operations.clone() {
            match op.operation {
                'A' => {
                    for i in 0..self.blocks_vec.len() {
                        let max = self
                            .blocks_vec
                            .iter()
                            .filter(|block| block.is_avalible())
                            .map(|block| block.get_size())
                            .max();
                        if self.blocks_vec[i].get_size() >= op.argument.unwrap() {
                            self.alloc(i, op)
                        } else if max.unwrap() < op.argument.unwrap() {
                            self.error(op.bl_id.unwrap(), op.id, max.unwrap(), 'A')
                        }
                    }
                }
                'D' => self.dealloc(op.bl_id, op.id),
                'O' => self.output("Worst fit"),
                'C' => println!("test"),
                _ => todo!(),
            }
        }
    }

    fn alloc(&mut self, i: usize, op: Operation) {
        let max = self
            .blocks_vec
            .iter()
            .filter(|block| block.is_avalible())
            .map(|block| block.get_size())
            .max();
        if self.blocks_vec[i].can_be_placed(op) {
            let end = self.blocks_vec[i].end;
            self.blocks_vec[i].operation = Some(op);
            let start = self.blocks_vec[i].start;
            self.blocks_vec[i].set_range(start, start + op.argument.unwrap() - 1);
            if self.blocks_vec[i].end < end {
                self.blocks_vec
                    .push(Block::new_full(self.blocks_vec[i].end + 1, end));
            }
        } else if max.unwrap() < op.argument.unwrap() {
            self.error(op.bl_id.unwrap(), op.id, max.unwrap(), 'A')
        }
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
        let max = self
            .blocks_vec
            .iter()
            .filter(|block| block.is_avalible())
            .map(|block| block.get_size())
            .max();

        let sum: i32 = self
            .blocks_vec
            .iter()
            .filter(|block| block.is_avalible())
            .map(|block| block.get_size())
            .sum();

        (1.0 - (max.unwrap() as f64 / sum as f64)) as f64
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
        self.join_blocks()
    }

    fn dealloc_err(&mut self, bl_id: Option<i32>, id: usize) {
        let t = self
            .blocks_vec
            .iter()
            .filter(|block| !block.is_avalible())
            .any(|block| block.operation.unwrap().bl_id.unwrap() == bl_id.unwrap());
        println!("{}", t);
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

    pub fn print_block(&self) -> (Vec<String>, Vec<String>) {
        let blocks = self
            .blocks_vec
            .iter()
            .filter(|block| !block.is_avalible())
            .map(|block| block.display_block())
            .collect::<Vec<_>>();
        let free = self
            .blocks_vec
            .iter()
            .filter(|block| block.is_avalible())
            .map(|block| block.display_block())
            .collect::<Vec<_>>();

        (blocks, free)
    }
}
