use crate::opcodes;
use hashbrown::HashMap;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF], // -1
}

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

trait Memory {
    fn memory_read(&self, address: u16) -> u8;

    fn memory_write(&mut self, address: u16, data: u8);

    /// Implement NES Little-Endian addressing for reading
    fn memory_read_u16(&self, position: u16) -> u16 {
        let lsb = self.memory_read(position) as u16;
        let msb = self.memory_read(position + 1) as u16;
        (msb << 8) | (lsb as u16) //
    }

    /// Implement NES Little-Endian addressing for writing
    fn memeory_read_u16(&mut self, position: u16, data: u16) {
        let msb = (data >> 8) as u8;
        let lsb = (data & 0xFF) as u8;
        self.memory_write(position, lsb);
        self.memory_write(position + 1, msb);
    }
}

impl Memory for CPU {
    fn memory_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn memory_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF], // -1
        }
    }

    /// Operand and Addressing Handling
    /// ===============================
    /// The 6502 uses a 16-bit address bus, where each byte is represented by
    /// two hex characters from $0000 - $FFFF
    /// Current reference: https://skilldrick.github.io/easy6502/#addressing
    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            // Zero Page (C0)
            // ==============
            // All instructions which support absolute addressing (excluding
            // the jump instructions) also have the option to take a single-
            // byte address.
            AddressingMode::ZeroPage => self.memory_read(self.program_counter) as u16,

            // Absolute (C000)
            // ===============
            // With absolute addressing, the full memory locatoin is used as
            // the argument to the instruction.
            AddressingMode::Absolute => self.memory_read_u16(self.program_counter),

            // Zero Page X (C0, X)
            // ===================
            // In this mode, a zero page address is given, and then the value
            // of the X register is added.
            AddressingMode::ZeroPageX => {
                let position = self.memory_read(self.program_counter);
                let address = position.wrapping_add(self.register_x) as u16;
                address
            }

            // Zero Page Y (C0, Y)
            // ===================
            // This is the equivalent of zero page, X, but can only be used
            // with LDX and STX
            AddressingMode::ZeroPageY => {
                let position = self.memory_read(self.program_counter);
                let address = position.wrapping_add(self.register_y) as u16;
                address
            }

            // Absolute X (C000, X)
            // ====================
            // Absolute adressing version of Zero Page X
            AddressingMode::AbsoluteX => {
                let base = self.memory_read_u16(self.program_counter);
                let address = base.wrapping_add(self.register_x as u16);
                address
            }

            // Absolute Y (C000, Y)
            // ====================
            // Absolute addressing version of Zero Page Y
            // Cannot be used with STX but can be used with LDA and STA
            AddressingMode::AbsoluteY => {
                let base = self.memory_read_u16(self.program_counter);
                let address = base.wrapping_add(self.register_y as u16);
                address
            }

            // Indexed Indirect ($C0, X)
            // =========================
            // Takes the zero page address, add the value of the X register
            // then use that to loop up a two-byte address.
            AddressingMode::IndirectX => {
                let base = self.memory_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lsb = self.memory_read(ptr as u16);
                let msb = self.memory_read(ptr.wrapping_add(1) as u16);
                (msb as u16) << 8 | (lsb as u16)
            }

            // Indirect Indexed (C0, Y)
            // ========================
            // Y address is added to the pre-dereferenced zero page address
            AddressingMode::IndirectY => {
                let base = self.memory_read(self.program_counter);

                let lsb = self.memory_read(base as u16);
                let msb = self.memory_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (msb as u16) << 8 | (lsb as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }

            // Default error handling.
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    /// ADC (0x69) - Add with Carry
    /// ===========================
    /// Adds the contents of a memory location to the accumulator together
    /// with the carry bit. If an overflow occures the carry bit is set,
    /// this enables multiple byte addition to be performed.
    ///
    /// Symbol |        Label      |        Description
    ///     C  | Carry Flag        | Set if overflow in bit 7
    ///     Z  | Zero Flag         | Set if A = 0
    ///     I  | Interrupt         | Not affected
    ///     D  | Decimal Mode Flag | Not affecded
    ///     B  | Break Command     | Not affected
    ///     V  | Overflow Flag     | Set if sign bit is incorrect
    ///     N  | Negative Flag     | Set if bit 7 is set
    fn adc(&mut self, mode: &AddressingMode) {
        todo!()
    }

    /// BRK (0x00) - Force Interrupt
    /// ============================
    /// The BRK instruction forces the generation of an interrupt request.
    /// The program count and processor status are pushed on the stack then
    /// the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break
    /// flag in the status is set to one.
    ///
    /// Symbol |        Label      |        Description
    ///    C   | Carry Flag        | Not affected
    ///    Z   | Zero Flag         | Not affected
    ///    I   | Interrupt Disable | Not affected
    ///    D   | Decimal Mode Flag | Not affected
    ///    B   | Break Command     | Set to 1
    ///    V   | Overflow Flag     | Not affected
    ///    N   | Negative Flag     | Not affected
    fn brk(&mut self) {
        todo!()
    }

    /// INX (0xE8) - Load Accumulator
    /// =============================
    /// Loads a byte of memory into the accumulator setting the
    /// zero and negative flags as appropriate.
    ///
    /// Symbol |        Label      |        Description
    ///    C   | Carry Flag        | Not affected
    ///    Z   | Zero Flag         | Set if A = 0
    ///    I   | Interrupt Disable | Not affected
    ///    D   | Decimal Mode Flag | Not affected
    ///    B   | Break Command     | Not affected
    ///    V   | Overflow Flag     | Not affected
    ///    N   | Negative Flag     | Set if bit 7 of A is set
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.set_zero_negative(self.register_x);
    }

    /// LDA (0xA9) - Load Accumulator
    /// =============================
    /// Loads a byte of memory into the accumulator setting the
    /// zero and negative flags as appropriate.
    ///
    /// Symbol |        Label      |        Description
    ///    C   | Carry Flag        | Not affected
    ///    Z   | Zero Flag         | Set if A = 0
    ///    I   | Interrupt Disable | Not affected
    ///    D   | Decimal Mode Flag | Not affected
    ///    B   | Break Command     | Not affected
    ///    V   | Overflow Flag     | Not affected
    ///    N   | Negative Flag     | Set if bit 7 of A is set
    fn lda(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(&mode);
        let value = self.memory_read(address);

        self.register_a = value;
        self.set_zero_negative(self.register_a);
    }

    /// STA (0x85) - Store Accumulator
    /// ============================
    /// Stires tge contents of the accumulator into memory
    ///
    /// Symbol |        Label      |        Description
    ///    C   | Carry Flag        | Not affected
    ///    Z   | Zero Flag         | Not affected
    ///    I   | Interrupt Disable | Not affected
    ///    D   | Decimal Mode Flag | Not affected
    ///    B   | Break Flag        | Not affected
    ///    V   | Overflow Flag     | Not affected
    ///    N   | Negative Flag     | Not affected
    fn sta(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        self.memory_write(address, self.register_a);
    }

    /// TAX (0xAA) - Transfer Accumulator to X
    /// ======================================
    /// Copies the current contents of the accumulator into the
    /// X register and sets the zero and negative flags as
    /// appropriate
    ///
    /// Symbol |        Label      |        Description
    ///    C   | Carry Flag        | Not affected
    ///    Z   | Zero Flag         | Set if X = 0
    ///    I   | Interrupt Disable | Not affected
    ///    D   | Decimal Mode Flag | Not affected
    ///    B   | Break Command     | Not affected
    ///    V   | Overflow Flag     | Not affected
    ///    N   | Negative Flag     | Set if bit 7 of A is set
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.set_zero_negative(self.register_x);
    }

    /// Flag Setting
    fn set_zero_negative(&mut self, result: u8) {
        // Set Zero Flag (Z) if result = 0
        if result == 0 {
            self.status = self.status | 0b0000_0010; // 2
        } else {
            self.status = self.status & 0b1111_1101; // -3
        }

        // Set Negative Flag (N) if bit 7 of result is set
        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000; // -128
        } else {
            self.status = self.status & 0b0111_1111; // 127
        }
    }

    /// Memory

    fn memory_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn memory_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    /// [0x8000 .. 0xFFFF] is reserved for program ROM
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        // TODO Document magic number
        self.program_counter = self.memory_read_u16(0xFFFC); //
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        loop {
            let code = self.memory_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognised", code));

            match code {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                }

                /* STA */
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }

                0xAA => self.tax(),

                0xE8 => self.inx(),

                0x00 => self.brk(),

                _ => todo!(),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.memory_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }
}
