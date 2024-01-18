#[derive(Debug, Clone, PartialEq, Default)]
pub struct Register {
    pub used: bool,
}
impl Register {
    pub fn new() -> Self {
        Self { used: false }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy, PartialOrd, Ord)]
pub struct RegIdx {
    pub index: u8,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Registers {
    pub registers: [Register; 8],
}
use std::ops::{Index, IndexMut};
impl Registers {
    pub fn get_unused(&mut self) -> Option<RegIdx> {
        for (i, reg) in self.registers.iter_mut().enumerate() {
            if !reg.used {
                reg.used = true;
                return Some(RegIdx { index: i as u8 });
            }
        }
        None
    }
    pub fn free(&mut self, idx: RegIdx) {
        self.registers[idx.index as usize].used = false;
    }
}
impl Index<RegIdx> for Registers {
    type Output = Register;
    fn index(&self, index: RegIdx) -> &Self::Output {
        &self.registers[index.index as usize]
    }
}
impl IndexMut<RegIdx> for Registers {
    fn index_mut(&mut self, index: RegIdx) -> &mut Self::Output {
        &mut self.registers[index.index as usize]
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(non_snake_case)]
pub struct State {
    pub registers: Registers,

    pub stack: Vec<u64>,
}
