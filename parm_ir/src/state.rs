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
    pub registers: [Register; 16],
}
use std::ops::{Index, IndexMut};
impl Registers {
    pub fn get_unused(&self) -> Option<RegIdx> {
        for (idx, register) in self.registers.iter().enumerate() {
            if !register.used {
                return Some(RegIdx { index: idx as u8 });
            }
        }
        None
    }
    pub fn is_unused(&self, idx: RegIdx) -> bool {
        !self.registers[idx.index as usize].used
    }
    pub fn get(&self, idx: RegIdx) -> &Register {
        &self.registers[idx.index as usize]
    }

    pub fn free(&mut self, idx: RegIdx) -> Result<(), ()> {
        let register = &mut self.registers[idx.index as usize];
        if !register.used {
            return Err(());
        }
        register.used = false;
        Ok(())
    }
    pub fn use_reg(&mut self, idx: RegIdx) -> Result<(), ()> {
        let register = &mut self.registers[idx.index as usize];
        if register.used {
            return Err(());
        }
        register.used = true;
        Ok(())
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
