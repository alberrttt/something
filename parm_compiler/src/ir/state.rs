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
pub enum RegIdx {
    Index(u8),
    #[default]
    None,
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
                return Some(RegIdx::Index(idx.try_into().unwrap()));
            }
        }
        None
    }
    pub fn is_unused(&self, idx: u8) -> bool {
        !self.registers[idx as usize].used
    }
    pub fn get(&self, idx: u8) -> &Register {
        &self.registers[idx as usize]
    }

    pub fn free(&mut self, idx: RegIdx) -> Result<(), ()> {
        let idx = match idx {
            RegIdx::Index(idx) => idx,
            _ => return Err(()),
        };
        let register = &mut self.registers[idx as usize];
        if !register.used {
            return Err(());
        }
        register.used = false;
        Ok(())
    }
    pub fn use_reg(&mut self, idx: RegIdx) -> Result<(), ()> {
        let idx = match idx {
            RegIdx::Index(idx) => idx,
            _ => return Err(()),
        };
        let register = &mut self.registers[idx as usize];
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
        let index = match index {
            RegIdx::Index(index) => index,
            _ => panic!("Invalid register index"),
        };
        &self.registers[index as usize]
    }
}
impl IndexMut<RegIdx> for Registers {
    fn index_mut(&mut self, index: RegIdx) -> &mut Self::Output {
        let index = match index {
            RegIdx::Index(index) => index,
            _ => panic!("Invalid register index"),
        };
        &mut self.registers[index as usize]
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(non_snake_case)]
pub struct State {
    pub registers: Registers,

    pub stack: Vec<u64>,
}
