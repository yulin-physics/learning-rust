pub fn run() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    // initialise a few registers with values
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // opcode 0x2100, CALL the function at 0x100
    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;

    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;

    // opcode 0x8014, add register 1's value to register 0
    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    // opcode 00EE: RETURN
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

struct CPU {
    // current_operation: u16, // opcode
    registers: [u8; 16],
    position_in_memory: usize, // program counter
    memory: [u8; 4096],        // 4kb memory
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn run(&mut self) {
        loop {
            // interpret u8 as u16 and join together as a single opcode
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
            let raw_op = op_byte1 << 8 | op_byte2;

            let x = ((raw_op & 0x0F00) >> 8) as u8;
            let y = ((raw_op & 0x00F0) >> 4) as u8;
            let op_minor = (raw_op & 0x000F) as u8;
            let addr = raw_op & 0x0FFF;

            println!(
                "pos: {:04x} & {:04x}",
                self.position_in_memory,
                self.position_in_memory + 1
            );
            self.position_in_memory += 2;
            println!("opcode: {:04x}", raw_op);

            // range matching
            match raw_op {
                0x0000 => {
                    return;
                }
                0x00EE => self.ret(),
                0x2000..=0x2FFF => self.call(addr),
                0x8000..=0x8FFF => match op_minor {
                    4 => {
                        self.add_xy(x, y);
                    }
                    _ => unimplemented!("opcode: {:04x}", raw_op),
                },
                _ => unimplemented!("opcode: {:04x}", raw_op),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    // CALL opcodes, 0x2NNN where NNN is a memory address. Modifies position_in_memory to point to the address of the function
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    // RETURN modifies position_in_memory to the memory address of the previous CALL
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }
}
