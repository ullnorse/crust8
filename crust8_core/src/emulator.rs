use crate::constants::*;
use crate::opcode::{Opcode, EmulatorError};
use rand::random;

pub struct Emulator {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
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
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };
        emu.load_fontset();
        emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram.fill(0);
        self.screen.fill(false);
        self.v_reg.fill(0);
        self.i_reg = 0;
        self.sp = 0;
        self.stack.fill(0);
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.load_fontset();
    }

    fn load_fontset(&mut self) {
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = start + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    pub fn tick(&mut self) -> Result<(), EmulatorError> {
        let op = self.fetch();
        self.execute(op)
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // sound TODO
            }
            self.sound_timer -= 1;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        if idx < NUM_KEYS {
            self.keys[idx] = pressed;
        }
    }

    fn fetch(&mut self) -> u16 {
        let high = self.ram[self.pc as usize];
        let low = self.ram[(self.pc + 1) as usize];
        self.pc += 2;
        u16::from_be_bytes([high, low])
    }

    fn push(&mut self, val: u16) -> Result<(), EmulatorError> {
        if (self.sp as usize) >= STACK_SIZE {
            return Err(EmulatorError::StackOverflow);
        }
        self.stack[self.sp as usize] = val;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<u16, EmulatorError> {
        if self.sp == 0 {
            return Err(EmulatorError::StackUnderflow);
        }
        self.sp -= 1;
        Ok(self.stack[self.sp as usize])
    }

    fn execute(&mut self, op: u16) -> Result<(), EmulatorError> {
        let opcode = Opcode::try_from(op)?;
        match opcode {
            Opcode::ClearScreen => self.clear_screen(),
            Opcode::ReturnSubroutine => self.return_subroutine()?,
            Opcode::Jump(addr) => self.jump(addr),
            Opcode::Call(addr) => self.call(addr)?,
            Opcode::SkipIfEqByte { x, byte } => self.skip_if_eq_byte(x, byte),
            Opcode::SkipIfNeqByte { x, byte } => self.skip_if_neq_byte(x, byte),
            Opcode::SkipIfEqReg { x, y } => self.skip_if_eq_reg(x, y),
            Opcode::SetReg { x, byte } => self.set_reg(x, byte),
            Opcode::AddByteToReg { x, byte } => self.add_byte_to_reg(x, byte),
            Opcode::SetRegToReg { x, y } => self.set_reg_to_reg(x, y),
            Opcode::OrReg { x, y } => self.or_reg(x, y),
            Opcode::AndReg { x, y } => self.and_reg(x, y),
            Opcode::XorReg { x, y } => self.xor_reg(x, y),
            Opcode::AddRegToReg { x, y } => self.add_reg_to_reg(x, y),
            Opcode::SubRegFromReg { x, y } => self.sub_reg_from_reg(x, y),
            Opcode::ShrReg { x } => self.shr_reg(x),
            Opcode::SubnRegFromReg { x, y } => self.subn_reg_from_reg(x, y),
            Opcode::ShlReg { x } => self.shl_reg(x),
            Opcode::SkipIfNeqReg { x, y } => self.skip_if_neq_reg(x, y),
            Opcode::SetI(addr) => self.set_i(addr),
            Opcode::JumpV0(addr) => self.jump_v0(addr),
            Opcode::RndAndByte { x, byte } => self.rnd_and_byte(x, byte),
            Opcode::DrawSprite { x, y, n } => self.draw_sprite(x, y, n),
            Opcode::SkipIfKeyPressed { x } => self.skip_if_key_pressed(x),
            Opcode::SkipIfKeyNotPressed { x } => self.skip_if_key_not_pressed(x),
            Opcode::SetRegToDelayTimer { x } => self.set_reg_to_delay_timer(x),
            Opcode::WaitKeyPress { x } => self.wait_key_press(x),
            Opcode::SetDelayTimer { x } => self.set_delay_timer(x),
            Opcode::SetSoundTimer { x } => self.set_sound_timer(x),
            Opcode::AddRegToI { x } => self.add_reg_to_i(x),
            Opcode::SetIToSpriteAddr { x } => self.set_i_to_sprite_addr(x),
            Opcode::StoreBCD { x } => self.store_bcd(x),
            Opcode::StoreRegs { x } => self.store_regs(x),
            Opcode::LoadRegs { x } => self.load_regs(x),
        }
        Ok(())
    }

    fn clear_screen(&mut self) {
        self.screen.fill(false);
    }

    fn return_subroutine(&mut self) -> Result<(), EmulatorError> {
        self.pc = self.pop()?;
        Ok(())
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) -> Result<(), EmulatorError> {
        self.push(self.pc)?;
        self.pc = addr;
        Ok(())
    }

    fn skip_if_eq_byte(&mut self, x: usize, byte: u8) {
        if self.v_reg[x] == byte {
            self.pc += 2;
        }
    }

    fn skip_if_neq_byte(&mut self, x: usize, byte: u8) {
        if self.v_reg[x] != byte {
            self.pc += 2;
        }
    }

    fn skip_if_eq_reg(&mut self, x: usize, y: usize) {
        if self.v_reg[x] == self.v_reg[y] {
            self.pc += 2;
        }
    }

    fn set_reg(&mut self, x: usize, byte: u8) {
        self.v_reg[x] = byte;
    }

    fn add_byte_to_reg(&mut self, x: usize, byte: u8) {
        self.v_reg[x] = self.v_reg[x].wrapping_add(byte);
    }

    fn set_reg_to_reg(&mut self, x: usize, y: usize) {
        self.v_reg[x] = self.v_reg[y];
    }

    fn or_reg(&mut self, x: usize, y: usize) {
        self.v_reg[x] |= self.v_reg[y];
    }

    fn and_reg(&mut self, x: usize, y: usize) {
        self.v_reg[x] &= self.v_reg[y];
    }

    fn xor_reg(&mut self, x: usize, y: usize) {
        self.v_reg[x] ^= self.v_reg[y];
    }

    fn add_reg_to_reg(&mut self, x: usize, y: usize) {
        let (result, overflow) = self.v_reg[x].overflowing_add(self.v_reg[y]);
        self.v_reg[0xF] = if overflow { 1 } else { 0 };
        self.v_reg[x] = result;
    }

    fn sub_reg_from_reg(&mut self, x: usize, y: usize) {
        self.v_reg[0xF] = if self.v_reg[x] > self.v_reg[y] { 1 } else { 0 };
        self.v_reg[x] = self.v_reg[x].wrapping_sub(self.v_reg[y]);
    }

    fn shr_reg(&mut self, x: usize) {
        self.v_reg[0xF] = self.v_reg[x] & 0x1;
        self.v_reg[x] >>= 1;
    }

    fn subn_reg_from_reg(&mut self, x: usize, y: usize) {
        self.v_reg[0xF] = if self.v_reg[y] > self.v_reg[x] { 1 } else { 0 };
        self.v_reg[x] = self.v_reg[y].wrapping_sub(self.v_reg[x]);
    }

    fn shl_reg(&mut self, x: usize) {
        self.v_reg[0xF] = (self.v_reg[x] & 0x80) >> 7;
        self.v_reg[x] <<= 1;
    }

    fn skip_if_neq_reg(&mut self, x: usize, y: usize) {
        if self.v_reg[x] != self.v_reg[y] {
            self.pc += 2;
        }
    }

    fn set_i(&mut self, addr: u16) {
        self.i_reg = addr;
    }

    fn jump_v0(&mut self, addr: u16) {
        self.pc = addr + self.v_reg[0] as u16;
    }

    fn rnd_and_byte(&mut self, x: usize, byte: u8) {
        let rnd: u8 = random();
        self.v_reg[x] = rnd & byte;
    }

    fn draw_sprite(&mut self, x: usize, y: usize, n: usize) {
        self.v_reg[0xF] = 0;
        for row in 0..n {
            let sprite_byte = self.ram[self.i_reg as usize + row];
            for col in 0..8 {
                let pixel = (sprite_byte >> (7 - col)) & 1;
                if pixel == 1 {
                    let px = (self.v_reg[x] as usize + col) % SCREEN_WIDTH;
                    let py = (self.v_reg[y] as usize + row) % SCREEN_HEIGHT;
                    let idx = py * SCREEN_WIDTH + px;
                    if self.screen[idx] {
                        self.v_reg[0xF] = 1;
                    }
                    self.screen[idx] ^= true;
                }
            }
        }
    }

    fn skip_if_key_pressed(&mut self, x: usize) {
        if self.keys[self.v_reg[x] as usize] {
            self.pc += 2;
        }
    }

    fn skip_if_key_not_pressed(&mut self, x: usize) {
        if !self.keys[self.v_reg[x] as usize] {
            self.pc += 2;
        }
    }

    fn set_reg_to_delay_timer(&mut self, x: usize) {
        self.v_reg[x] = self.delay_timer;
    }

    fn wait_key_press(&mut self, x: usize) {
        if let Some(key) = self.keys.iter().position(|&pressed| pressed) {
            self.v_reg[x] = key as u8;
        } else {
            self.pc -= 2;
        }
    }

    fn set_delay_timer(&mut self, x: usize) {
        self.delay_timer = self.v_reg[x];
    }

    fn set_sound_timer(&mut self, x: usize) {
        self.sound_timer = self.v_reg[x];
    }

    fn add_reg_to_i(&mut self, x: usize) {
        self.i_reg = self.i_reg.wrapping_add(self.v_reg[x] as u16);
    }

    fn set_i_to_sprite_addr(&mut self, x: usize) {
        self.i_reg = (self.v_reg[x] as u16) * 5;
    }

    fn store_bcd(&mut self, x: usize) {
        let val = self.v_reg[x];
        self.ram[self.i_reg as usize] = val / 100;
        self.ram[self.i_reg as usize + 1] = (val % 100) / 10;
        self.ram[self.i_reg as usize + 2] = val % 10;
    }

    fn store_regs(&mut self, x: usize) {
        for i in 0..=x {
            self.ram[self.i_reg as usize + i] = self.v_reg[i];
        }
    }

    fn load_regs(&mut self, x: usize) {
        for i in 0..=x {
            self.v_reg[i] = self.ram[self.i_reg as usize + i];
        }
    }
}