pub struct SimpleRnd {
    values: [i32; 10],
    next: usize,
}

impl SimpleRnd {
    pub fn new() -> Self {
        SimpleRnd {
            values: [1 ,3, 5, 7, 2, 4, 6, 8, 9, 0],
            next: 0,
        }
    }

    pub fn next_rnd(&mut self) -> i32 {
        self.next = self.next + 1;
        if self.next > 9 {
            self.next = 0;
        }

        self.values[self.next]
    }
}
