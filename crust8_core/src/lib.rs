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

#[derive(Debug, PartialEq)]
pub enum Opcode {
    ClearScreen,
    ReturnSubroutine,
    Jump(u16),
    Call(u16),
    SkipIfEqByte { x: usize, byte: u8 },
    SkipIfNeqByte { x: usize, byte: u8 },
    SkipIfEqReg { x: usize, y: usize },
    SetReg { x: usize, byte: u8 },
    AddByteToReg { x: usize, byte: u8 },
    SetRegToReg { x: usize, y: usize },
    OrReg { x: usize, y: usize },
    AndReg { x: usize, y: usize },
    XorReg { x: usize, y: usize },
    AddRegToReg { x: usize, y: usize },
    SubRegFromReg { x: usize, y: usize },
    ShrReg { x: usize },
    SubnRegFromReg { x: usize, y: usize },
    ShlReg { x: usize },
    SkipIfNeqReg { x: usize, y: usize },
    SetI(u16),
    JumpV0(u16),
    RndAndByte { x: usize, byte: u8 },
    DrawSprite { x: usize, y: usize, n: usize },
    SkipIfKeyPressed { x: usize },
    SkipIfKeyNotPressed { x: usize },
    SetRegToDelayTimer { x: usize },
    WaitKeyPress { x: usize },
    SetDelayTimer { x: usize },
    SetSoundTimer { x: usize },
    AddRegToI { x: usize },
    SetIToSpriteAddr { x: usize },
    StoreBCD { x: usize },
    StoreRegs { x: usize },
    LoadRegs { x: usize },
}

impl Opcode {
    pub fn from_u16(op: u16) -> Result<Self, String> {
        let nibbles = (
            (op & 0xF000) >> 12,
            (op & 0x0F00) >> 8,
            (op & 0x00F0) >> 4,
            (op & 0x000F),
        );
        let byte = (op & 0x00FF) as u8;
        let addr = op & 0x0FFF;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Ok(Opcode::ClearScreen),
            (0x0, 0x0, 0xE, 0xE) => Ok(Opcode::ReturnSubroutine),
            (0x1, _, _, _) => Ok(Opcode::Jump(addr)),
            (0x2, _, _, _) => Ok(Opcode::Call(addr)),
            (0x3, x, _, _) => Ok(Opcode::SkipIfEqByte { x: x as usize, byte }),
            (0x4, x, _, _) => Ok(Opcode::SkipIfNeqByte { x: x as usize, byte }),
            (0x5, x, y, 0x0) => Ok(Opcode::SkipIfEqReg { x: x as usize, y: y as usize }),
            (0x6, x, _, _) => Ok(Opcode::SetReg { x: x as usize, byte }),
            (0x7, x, _, _) => Ok(Opcode::AddByteToReg { x: x as usize, byte }),
            (0x8, x, y, 0x0) => Ok(Opcode::SetRegToReg { x: x as usize, y: y as usize }),
            (0x8, x, y, 0x1) => Ok(Opcode::OrReg { x: x as usize, y: y as usize }),
            (0x8, x, y, 0x2) => Ok(Opcode::AndReg { x: x as usize, y: y as usize }),
            (0x8, x, y, 0x3) => Ok(Opcode::XorReg { x: x as usize, y: y as usize }),
            (0x8, x, y, 0x4) => Ok(Opcode::AddRegToReg { x: x as usize, y: y as usize }),
            (0x8, x, y, 0x5) => Ok(Opcode::SubRegFromReg { x: x as usize, y: y as usize }),
            (0x8, x, _, 0x6) => Ok(Opcode::ShrReg { x: x as usize }),
            (0x8, x, y, 0x7) => Ok(Opcode::SubnRegFromReg { x: x as usize, y: y as usize }),
            (0x8, x, _, 0xE) => Ok(Opcode::ShlReg { x: x as usize }),
            (0x9, x, y, 0x0) => Ok(Opcode::SkipIfNeqReg { x: x as usize, y: y as usize }),
            (0xA, _, _, _) => Ok(Opcode::SetI(addr)),
            (0xB, _, _, _) => Ok(Opcode::JumpV0(addr)),
            (0xC, x, _, _) => Ok(Opcode::RndAndByte { x: x as usize, byte }),
            (0xD, x, y, n) => Ok(Opcode::DrawSprite { x: x as usize, y: y as usize, n: n as usize }),
            (0xE, x, 0x9, 0xE) => Ok(Opcode::SkipIfKeyPressed { x: x as usize }),
            (0xE, x, 0xA, 0x1) => Ok(Opcode::SkipIfKeyNotPressed { x: x as usize }),
            (0xF, x, 0x0, 0x7) => Ok(Opcode::SetRegToDelayTimer { x: x as usize }),
            (0xF, x, 0x0, 0xA) => Ok(Opcode::WaitKeyPress { x: x as usize }),
            (0xF, x, 0x1, 0x5) => Ok(Opcode::SetDelayTimer { x: x as usize }),
            (0xF, x, 0x1, 0x8) => Ok(Opcode::SetSoundTimer { x: x as usize }),
            (0xF, x, 0x1, 0xE) => Ok(Opcode::AddRegToI { x: x as usize }),
            (0xF, x, 0x2, 0x9) => Ok(Opcode::SetIToSpriteAddr { x: x as usize }),
            (0xF, x, 0x3, 0x3) => Ok(Opcode::StoreBCD { x: x as usize }),
            (0xF, x, 0x5, 0x5) => Ok(Opcode::StoreRegs { x: x as usize }),
            (0xF, x, 0x6, 0x5) => Ok(Opcode::LoadRegs { x: x as usize }),
            _ => Err(format!("Unknown opcode {:#X}", op)),
        }
    }
}

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
                // TODO: add sound
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

    fn push(&mut self, val: u16) {
        assert!((self.sp as usize) < STACK_SIZE, "Stack overflow");
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        assert!(self.sp > 0, "Stack underflow");
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn execute(&mut self, op: u16) {
        let opcode = Opcode::from_u16(op).unwrap_or_else(|e| panic!("{}", e));
        match opcode {
            Opcode::ClearScreen => self.clear_screen(),
            Opcode::ReturnSubroutine => self.return_subroutine(),
            Opcode::Jump(addr) => self.jump(addr),
            Opcode::Call(addr) => self.call(addr),
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
    }

    fn clear_screen(&mut self) {
        self.screen.fill(false);
    }

    fn return_subroutine(&mut self) {
        self.pc = self.pop();
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.push(self.pc);
        self.pc = addr;
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
        let (res, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
        self.v_reg[x] = res;
        self.v_reg[0xF] = if carry { 1 } else { 0 };
    }

    fn sub_reg_from_reg(&mut self, x: usize, y: usize) {
        let (res, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
        self.v_reg[x] = res;
        self.v_reg[0xF] = if borrow { 0 } else { 1 };
    }

    fn shr_reg(&mut self, x: usize) {
        let lsb = self.v_reg[x] & 1;
        self.v_reg[x] >>= 1;
        self.v_reg[0xF] = lsb;
    }

    fn subn_reg_from_reg(&mut self, x: usize, y: usize) {
        let (res, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
        self.v_reg[x] = res;
        self.v_reg[0xF] = if borrow { 0 } else { 1 };
    }

    fn shl_reg(&mut self, x: usize) {
        let msb = (self.v_reg[x] >> 7) & 1;
        self.v_reg[x] <<= 1;
        self.v_reg[0xF] = msb;
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
        self.pc = (self.v_reg[0] as u16) + addr;
    }

    fn rnd_and_byte(&mut self, x: usize, byte: u8) {
        self.v_reg[x] = random::<u8>() & byte;
    }

    fn draw_sprite(&mut self, x: usize, y: usize, n: usize) {
        let x_pos = self.v_reg[x] as usize;
        let y_pos = self.v_reg[y] as usize;
        self.v_reg[0xF] = 0;

        for row in 0..n {
            let sprite_byte = self.ram[(self.i_reg + row as u16) as usize];
            for bit in 0..8 {
                if (sprite_byte & (0x80 >> bit)) != 0 {
                    let px = (x_pos + bit) % SCREEN_WIDTH;
                    let py = (y_pos + row) % SCREEN_HEIGHT;
                    let idx = px + py * SCREEN_WIDTH;
                    if self.screen[idx] {
                        self.v_reg[0xF] = 1;
                    }
                    self.screen[idx] ^= true;
                }
            }
        }
    }

    fn skip_if_key_pressed(&mut self, x: usize) {
        let key = self.v_reg[x] as usize;
        if key < NUM_KEYS && self.keys[key] {
            self.pc += 2;
        }
    }

    fn skip_if_key_not_pressed(&mut self, x: usize) {
        let key = self.v_reg[x] as usize;
        if key >= NUM_KEYS || !self.keys[key] {
            self.pc += 2;
        }
    }

    fn set_reg_to_delay_timer(&mut self, x: usize) {
        self.v_reg[x] = self.delay_timer;
    }

    fn wait_key_press(&mut self, x: usize) {
        let mut pressed_key = None;
        for (i, &pressed) in self.keys.iter().enumerate() {
            if pressed {
                pressed_key = Some(i as u8);
                break;
            }
        }
        if let Some(key) = pressed_key {
            self.v_reg[x] = key;
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
        let vx = self.v_reg[x];
        self.ram[self.i_reg as usize] = vx / 100;
        self.ram[self.i_reg as usize + 1] = (vx / 10) % 10;
        self.ram[self.i_reg as usize + 2] = vx % 10;
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

#[cfg(test)]
mod tests {

}
