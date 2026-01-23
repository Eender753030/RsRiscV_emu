#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RegisterFile {
    regs: [u32; 32],
}

impl RegisterFile {
    pub fn write(&mut self, id: u8, data: u32) {
        if id == 0 {
            return;
        }
        self.regs[id as usize] = data;
    }

    pub fn reset(&mut self) {
        self.regs.fill(0);
    }

    pub fn iter(&self) -> IteratorRegisterFile<'_> {
        IteratorRegisterFile { id: 0, regs: self }
    }

    pub fn inspect(&self) -> [u32; 32] {
        self.regs
    }
}

impl std::ops::Index<u8> for RegisterFile {
    type Output = u32;
    fn index(&self, index: u8) -> &Self::Output {
        &self.regs[index as usize]
    }
}

pub struct IteratorRegisterFile<'a> {
    id: u8,
    regs: &'a RegisterFile,
}

impl <'a>Iterator for IteratorRegisterFile<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.id < 32 {
            let next = Some(self.regs[self.id]);
            self.id += 1;
            next
        } else {
            None
        }
    }
}