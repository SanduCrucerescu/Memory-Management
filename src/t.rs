fn dealloc(&mut self, id: i32) {
    println!("Deallocating {}", id);

    let blocks = self.blocks_vec.clone();

    let mut idx: usize = 0;
    let mut block: Block = Block::new(0);
    for (i, b) in blocks.iter().enumerate() {
        if b.operation.is_some() && b.operation.unwrap().id == id {
            println!("Found block {}", b.operation.unwrap().id);
            idx = i;
            block = b.clone();
            break;
        }
    }
    // let (idx, block): (usize, &Block) = blocks
    //     .iter()
    //     .enumerate()
    //     .find(|(_, x)| x.operation.is_some() && x.operation.unwrap().id == id);

    for i in 0..self.blocks_vec.len() {
        if self.blocks_vec[i].hole.start == block.hole.end + 1
            || self.blocks_vec[i].hole.end + 1 == block.hole.start
        {
            let mergable = self.blocks_vec.get(i).unwrap();
            let start = min(mergable.hole.start, block.hole.start);
            let end = max(mergable.hole.end, block.hole.end);
            let nf_block = Block::new_full(Hole::new(start, end));
            if i > idx {
                self.blocks_vec[idx] = nf_block;
            } else {
                self.blocks_vec[idx - 1] = nf_block;
            }
            return;
        } else {
            self.blocks_vec[idx] = Block::new_full(Hole::new(block.hole.start, block.hole.end));
            return;
        };
    }
}
