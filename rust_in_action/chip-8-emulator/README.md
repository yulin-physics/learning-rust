# CPU Emulation

Implementing a CPU in software to establish that functions are also data. To call a function written in CHIP-8, one must know its memory address (NNN) beforehand.

Within CHIP-8, control flow works by comparing values in registers, then modifying position_in_memory depending on the outcome. There are no while loops or for loops within a CPU. Creating them in programming languages is the art of the compiler writer.

## Glossary

- Operation (op)/implemented in hardware/intrinsic operation refers to procedures that are supported by the system natively.

- Registers are containers for data that CPU accesses directly. For most operations, operands must be moved to registers for the operation to function. For the CHIP-8, each register is a u8.

- An opcode is a number that maps to an operation. On the CHIP-8 platform, opcodes include both the operations and operands' registers.

Sometimes operands will need to be added to the stack, sometimes inserted into defined registers.

## CHIP-8 Opcodes

u16 values that need to be decoded into one of the three forms:

- the standard case invides opcodes into 4 parts: operation group/major group (u8) `(op & 0xF000) >> 12 `, operation identifier within the group/operation indicator (u8) `op & 0x000F `, left register (u8)`(op & 0x0F00) >> 8 ` and right register (u8) `(op & 0x00F0) >> 4 `

- to perform an operation with a constant, the opcode is divided into three parts: operation identifier (u8), left operand (u8) and the constant (u8)

- an alternative case divides the opcode into 2 parts: major operation group (u8), memory address (u16)
