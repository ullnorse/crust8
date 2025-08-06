#![allow(unused)]

use rand::random;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const START_ADDR: u16 = 0x200;
const NUM_KEYS: usize = 16;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emulator {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    pub fn new() -> Self {
        let mut emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };

        emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                //TODO: implement a beep
            }
            self.sound_timer -= 1;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = (op & 0x000F);

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => (),

            // Clear screen
            (0, 0, 0xE, 0) => self.screen.fill(false),

            // Return from subroutine
            (0, 0, 0xE, 0xE) => self.pc = self.pop(),

            // Jump to address 0xNNN
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            }

            // Call 0xNNN
            (2, _, _, _) => {
                self.push(self.pc);
                let nnn = op & 0xFFF;
                self.pc = nnn;
            }

            // Skip if VX == 0xNN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            }

            // Skip if VX != 0xNN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;

                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            }

            // Skip if VX == VY
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }

            // VX = 0xNN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            }

            // VX += 0xNN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }

            // VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            }

            // VX |= VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            }

            // VX &= VY
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            }

            // VX ^= VY
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            }

            // VX += VY
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);

                self.v_reg[x] = vx;
                self.v_reg[0xF] = if carry { 1 } else { 0 };
            }

            // VX -= VY
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);

                self.v_reg[x] = vx;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            }

            // VX >>= 1
            (8, _, _, 6) => {
                let x = digit2 as usize;

                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }

            // VX = VY - VX
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);

                self.v_reg[x] = vx;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            }

            // VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit2 as usize;

                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }

            // Skip if VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }

            // I = 0xNNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            }

            // Jump to V0 + 0xNNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            }

            // VX = rand() & 0xNN
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;

                self.v_reg[x] = random::<u8>() & nn;
            }

            // Draw sprite at (VX, VY)
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;

                let num_rows = digit4;
                let mut flipped = false;

                for y_line in 0..num_rows {
                    let addr = self.i_reg + y_line;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;

                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            }

            // SKip if key index in VX is pressed
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as usize;

                if self.keys[vx] {
                    self.pc += 2;
                }
            }

            // SKip if key index in VX isn't pressed
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as usize;

                if !self.keys[vx] {
                    self.pc += 2;
                }
            }

            // VX = Delay timer
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.v_reg[x] = self.delay_timer;
            }

            // Waits for key press, stores index in VX
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;

                for (index, &key) in self.keys.iter().enumerate() {
                    if key {
                        self.v_reg[x] = index as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            }

            // Delay Timer = VX
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.delay_timer = self.v_reg[x];
            }

            // Sound Timer = VX
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.sound_timer = self.v_reg[x];
            }

            // I += VX
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[x] as u16);
            }

            // Set I to address of font character in VX
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.v_reg[x] as u16;
                self.i_reg = c * 5;
            }

            // Stores BCD encoding of VX into RAM address starting at I
            (0xF, _, 3, 3) => {
                // TODO: lookup faster bcd algo
                let x = digit2 as usize;
                let vx = self.v_reg[x] as f32;

                let vx = self.v_reg[x];
                self.ram[self.i_reg as usize] = vx / 100;
                self.ram[self.i_reg as usize + 1] = (vx / 10) % 10;
                self.ram[self.i_reg as usize + 2] = vx % 10;
            }

            // Stores V0 thru VX into RAM address starting at I
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;

                for i in 0..=x {
                    self.ram[self.i_reg as usize + i] = self.v_reg[i];
                }
            }

            // Stores V0 thru VX with RAM values starting at address in I
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;

                for i in 0..=x {
                    self.v_reg[i] = self.ram[self.i_reg as usize + i];
                }
            }

            (_, _, _, _) => unimplemented!("Unimplemented opcode: {op}"),
        }
    }

    fn fetch(&mut self) -> u16 {
        let bytes = [self.ram[self.pc as usize], self.ram[(self.pc + 1) as usize]];
        self.pc += 2;
        u16::from_be_bytes(bytes)
    }

    fn push(&mut self, val: u16) {
        debug_assert!(
            self.sp < 16,
            "Stack overflow at PC={:04X}, SP={:04X}",
            self.pc,
            self.sp
        );

        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        debug_assert!(
            self.sp > 0,
            "Stack underflow at PC={:04X}, SP={:04X}",
            self.pc,
            self.sp
        );

        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_push() {
        let mut emu = Emulator::default();
        emu.push(1);
        assert_eq!(emu.stack[0], 1);
        assert_eq!(emu.sp, 1);

        emu.push(2);
        assert_eq!(emu.stack[1], 2);
        assert_eq!(emu.sp, 2);
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn test_push_full_stack() {
        let mut emu = Emulator::new();

        for i in 0..17 {
            emu.push(i);
        }
    }

    #[test]
    fn test_pop() {
        let mut emu = Emulator::new();

        emu.push(1);
        assert_eq!(emu.sp, 1);
        assert_eq!(emu.pop(), 1);
        assert_eq!(emu.sp, 0);
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn test_pop_stack_empty() {
        let mut emu = Emulator::new();
        emu.pop();
    }
}
