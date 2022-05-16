pub fn run() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    // initialise a few registers with values
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    // load opcode 0x8014 to memory, add the value in register 1 to register 0
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;
    // load opcode 0x8024 to memory, add the value in register 2 to register 0
    cpu.memory[2] = 0x80;
    cpu.memory[3] = 0x24;
    // load opcode 0x8034 to memory, add the value in register 3 to register 0
    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}

// logic and arithmetic operations are in the group 0x8, control flow instructions are in the group 0x1
const ARITHMETIC_AND_LOGIC: u8 = 0x8;
const ADD_XY: u8 = 0x4;
const HALT: u8 = 0x0;

struct CPU {
    // current_operation: u16, // opcode
    registers: [u8; 16],
    position_in_memory: usize, // program counter
    memory: [u8; 4096],        // 4kb memory
}

impl CPU {
    fn run(&mut self) {
        loop {
            // interpret u8 as u16 and join together as a single opcode
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
            let raw_op = op_byte1 << 8 | op_byte2;
            let op_major = ((raw_op & 0xF000) >> 12) as u8;
            let x = ((raw_op & 0x0F00) >> 8) as u8;
            let y = ((raw_op & 0x00F0) >> 4) as u8;
            let op_minor = (raw_op & 0x000F) as u8;

            self.position_in_memory += 2;

            match (op_major, op_minor) {
                (HALT, HALT) => {
                    return;
                }
                (ARITHMETIC_AND_LOGIC, ADD_XY) => self.add_xy(x, y),
                _ => unimplemented!("opcode: {:04x}", raw_op),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}
