# CPU Emulation

Implementing a CPU in software to establish that functions are also data

## Glossary

- Operation (op)/implemented in hardware/intrinsic operation refers to procedures that are supported by the system natively.

- Registers are containers for data that CPU accesses directly. For most operations, operands must be moved to registers for the operation to function. For the CHIP-8, each register is a u8.

- An opcode is a number that maps to an operation. On the CHIP-8 platform, opcodes include both the operations and operands' registers.

## CHIP-8 Opcodes

u16 values that need to be decoded into one of the three forms:

- the standard case invides opcodes into 4 parts: operation group/major group (u8) `(op & 0xF000) >> 12 `, operation identifier within the group/operation indicator (u8) `op & 0x000F `, left register (u8)`(op & 0x0F00) >> 8 ` and right register (u8) `(op & 0x00F0) >> 4 `

- to perform an operation with a constant, the opcode is divided into three parts: operation identifier (u8), left operand (u8) and the constant (u8)

- an alternative case divides the opcode into 2 parts: major operation group (u8), memory address (u16)
