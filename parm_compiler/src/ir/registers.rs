use std::marker::PhantomData;

use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Registers {
    pub registers: Box<[Register; 256]>,
}
impl Default for Registers {
    fn default() -> Self {
        let mut registers = Box::new(
            [Register {
                used: false,
                register: 0,
            }; 256],
        );
        for (i, register) in registers.iter_mut().enumerate() {
            register.register = i as u8;
        }
        Registers { registers }
    }
}
impl Registers {
    pub fn allocate(&mut self) -> Option<u8> {
        for (i, register) in self.registers.iter_mut().enumerate() {
            if !register.used {
                register.used = true;
                return Some(i as u8);
            }
        }
        None
    }
    pub fn allocate_register(&mut self, register: usize) -> Option<u8> {
        if !self.registers[register].used {
            self.registers[register].used = true;
            return Some(register as u8);
        }
        None
    }
    pub fn get_free_register<'b>(&'b mut self) -> Option<u8> {
        for (i, register) in self.registers.iter_mut().enumerate() {
            if !register.used {
                return Some(i as u8);
            }
        }
        None
    }
    pub fn deallocate(&mut self, register: u8) {
        if self.registers[register as usize].used {
            self.registers[register as usize].used = false;
        } else {
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Register {
    pub used: bool,
    pub register: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegisterRef {
    pub register: u8,
    pub kind: RegisterKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterKind {
    Value,
    Variable,
}
