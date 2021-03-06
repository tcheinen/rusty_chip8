// Copyright (C) 2019 Teddy Heinen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::constants;
use crate::cpu::{CPU, mnemonic};
use crate::keyboard::Keyboard;
use crate::display::Display;

pub struct Emulator {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub index: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub delay: u8,
    pub sound: u8,
    pub display: Display,
    pub instruction: u16,
    pub keyboard: Keyboard,
}

impl Emulator {
    pub fn tick(&mut self) {
        self.instruction = ((self.memory[self.pc as usize] as u16) << 8) | self.memory[(self.pc + 1) as usize] as u16;

        let n: u8 = (self.instruction & 0x000f) as u8;
        let nn: u8 = (self.instruction & 0x00ff) as u8;

        println!("{:#x} - {}", self.instruction, mnemonic(self.instruction));
        match self.instruction >> 12 {
            0x0 => {
                match nn {
                    0xE0 => { self.cls() }
                    0xEE => { self.ret() }
                    _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", self.instruction, self.pc) }
                }
            }
            0x1 => { self.jp_addr() }
            0x2 => { self.call_addr() }
            0x3 => { self.se_vx_byte() }
            0x4 => { self.sne_vx_byte() }
            0x5 => { self.se_vx_vy() }
            0x6 => { self.ld_vx_byte() }
            0x7 => { self.add_vx_byte() }
            0x8 => {
                match n {
                    0x0 => { self.ld_vx_vy() }
                    0x1 => { self.or_vx_vy() }
                    0x2 => { self.and_vx_vy() }
                    0x3 => { self.xor_vx_vy() }
                    0x4 => { self.add_vx_vy() }
                    0x5 => { self.sub_vx_vy() }
                    0x6 => { self.shr_vx_vy() }
                    0x7 => { self.subn_vx_vy() }
                    0x8 => { self.shl_vx_vy() }
                    _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", self.instruction, self.pc) }
                }
            }
            0x9 => { self.sne_vx_vy() }
            0xA => { self.ld_i_addr() }
            0xB => { self.jp_v0_addr() }
            0xC => { self.rnd_vx_byte() }
            0xD => { self.drw_vx_vy_nibble() }
            0xE => {
                match nn {
                    0x9e => { self.skp_vx() }
                    0xA1 => { self.sknp_vx() }
                    _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", self.instruction, self.pc) }
                }
            }
            0xF => {
                match nn {
                    0x07 => { self.ld_vx_dt() }
                    0x0A => { self.ld_vx_k() }
                    0x15 => { self.ld_dt_vx() }
                    0x18 => { self.ld_st_vx() }
                    0x1E => { self.add_i_vx() }
                    0x29 => { self.ld_f_vx() }
                    0x33 => { self.ld_b_vx() }
                    0x55 => { self.ld_mem_vx() }
                    0x65 => { self.ld_vx_mem() }
                    _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", self.instruction, self.pc) }
                }
            }
            _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", self.instruction, self.pc) }
        }
    }

    // return lower 12 bits of an opcode
    pub fn get_nnn(&self) -> u16 { self.instruction & 0x0FFF }

    /// return lower byte of opcode
    pub fn get_nn(&self) -> u8 {
        (self.instruction & 0xFF) as u8
    }

    /// return lower four bits of opcode
    pub fn get_n(&self) -> u8 {
        (self.instruction & 0x000F) as u8
    }

    /// return second most significant 4 bits 0x00
    pub fn get_x(&self) -> u8 {
        ((self.instruction & 0x0F00) >> 8) as u8
    }

    /// return third most significant 4 bits 00y0
    pub fn get_y(&self) -> u8 {
        ((self.instruction & 0x00F0) >> 4) as u8
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let mut index = 0x200;
        for byte in rom {
            self.memory[index] = byte;
            index += 1;
        }
    }

    pub fn new() -> Self {
        let mut state = Emulator {
            registers: [0x0; 16],
            memory: [0x0; 4096],
            index: 0,
            pc: 0x200,
            sp: 0,
            stack: [0x0; 16],
            delay: 0,
            sound: 0,
            display: Display::new(),
            instruction: 0,
            keyboard: Keyboard::new(),
        };
        for i in 0..constants::FONTSET.len() {
            state.memory[constants::FONTSET_START as usize + i] = constants::FONTSET[i];
        }
        state
    }
}
