# parm_assembler

PARM Assembler is a library for assembling a subset of the ARM instruction set.
It is designed to be used in conjunction with logisim-evolution, a digital logic simulator.
It was written as a part of the Computer Architecture course.

## Examples
```rust
        const INPUT: &str = "
            movs r0, #0
            movs r1, #1
            .goto:
            movs r2, #20
            cmp r0, r1
            bMI .then1
            b .endif1
            .then1:
            rsbs r2, r2, #0
            .endif1:
            cmp r2, r1
            bLT .then2
            b .endif2
            .then2:
            movs r0, #50
            b .goto
            .endif2:
            adds r3, r0, r2
            @a comment";

        let output = parm_assembler::export_to_logisim(INPUT).unwrap();

        // logisim-evolution expects the output to be in the following format:
        let expected = "v2.0 raw\n2000 2101 2214 4288 d4ff e7ff 4252 428a dbff e000 2032 e7f4 1883";

        assert_eq!(expected, output);
```

## Supported instructions

See the [assignment](https://bitbucket.org/edge-team-leat/parm_public/src/21ae509e77e4e70bc79301eb59c3f1f9567fb62e/doc/main.pdf) for a full list of supported instructions.

## Overview

- Each instruction is an enum variant.
- Each instruction is associated to a struct representing its operands.
- Each operand is an enum variant.
- Lines are parsed into a vector of `Instruction`s using the `nom` crate.
- The `Instruction`s are then converted into a byte vector using the `bitvec` crate, each one being 16 bits long.
- The byte vector is then converted into a string of hexadecimal numbers.
