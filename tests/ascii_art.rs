#[cfg(test)]
mod tests {
    use parm_assembler::{export_to_logisim, LogisimProgram};

    #[test]
    fn string() {
        let input = r#"
            string_label:
            .asciz "Hello, world!"
            .end:
            b .end
        "#;

        let output = export_to_logisim(input).unwrap();

        let expected_rom = "v2.0 raw\ne7fd";

        let expected_ram =
            "v2.0 raw\n0048 0065 006c 006c 006f 002c 0020 0077 006f 0072 006c 0064 0021";

        assert_eq!(
            output,
            LogisimProgram {
                rom: expected_rom.to_owned(),
                ram: expected_ram.to_owned()
            }
        );
    }

    #[test]
    fn complete_program() {
        let input = r#"
        .text
        .syntax unified
        .eabi_attribute 67, "2.09"
        .cpu    cortex-m0
        .eabi_attribute 6, 12
        .eabi_attribute 7, 77
        .eabi_attribute 8, 0
        .eabi_attribute 9, 1
        .eabi_attribute 34, 0
        .eabi_attribute 17, 1
        .eabi_attribute 20, 1
        .eabi_attribute 21, 1
        .eabi_attribute 23, 3
        .eabi_attribute 24, 1
        .eabi_attribute 25, 1
        .eabi_attribute 38, 1
        .eabi_attribute 18, 4
        .eabi_attribute 26, 2
        .eabi_attribute 14, 0
        .file   "ascii_art.c"
        .globl  run
        .p2align        2
        .type   run,%function
        .code   16
        .thumb_func
run:
        .fnstart
        .pad    #4
        sub     sp, #4
        @APP
        sub     sp, #508
        @NO_APP
        @APP
        sub     sp, #452
        @NO_APP
        ldr     r0, .LCPI0_0
        ldrb    r1, [r0]
        str     r1, [sp]
        ldrb    r1, [r0, #1]
        str     r1, [sp]
        ldrb    r1, [r0, #2]
        str     r1, [sp]
        ldrb    r0, [r0, #3]
        str     r0, [sp]
.LBB0_1:
        b       .LBB0_1
        .p2align        2
.LCPI0_0:
        .long   .L.str
.Lfunc_end0:
        .size   run, .Lfunc_end0-run
        .cantunwind
        .fnend

        .type   .L.str,%object
        .section        .rodata.str1.1,"aMS",%progbits,1
.L.str:
        .asciz  "  _____        _____  __  __\n |  __ \\ /\\   |  __ \\|  \\/  |\n | |__) /  \\  | |__) | \\  / |\n |  ___/ /\\ \\ |  _  /| |\\/| |\n | |  / ____ \\| | \\ \\| |  | |\n |_| /_/    \\_|_|  \\_|_|  |_|\n"
        .size   .L.str, 180


        .ident  "clang version 8.0.1- (branches/release_80)"
        .section        ".note.GNU-stack","",%progbits
        .addrsig
        .eabi_attribute 30, 1
        "#;

        let output = export_to_logisim(input).unwrap();

        let expected_rom =
            "v2.0 raw\nb081 b0ff b0f1 2000 6801 9100 6841 9100 6881 9100 68c0 9000 e7fd";

        let expected_ram =
            "v2.0 raw\n0020 0020 005f 005f 005f 005f 005f 0020 0020 0020 0020 0020 0020 0020 0020 \
            005f 005f 005f 005f 005f 0020 0020 005f 005f 0020 0020 005f 005f 000a 0020 007c 0020 \
            0020 005f 005f 0020 005c 0020 002f 005c 0020 0020 0020 007c 0020 0020 005f 005f 0020 \
            005c 007c 0020 0020 005c 002f 0020 0020 007c 000a 0020 007c 0020 007c 005f 005f 0029 \
            0020 002f 0020 0020 005c 0020 0020 007c 0020 007c 005f 005f 0029 0020 007c 0020 005c \
            0020 0020 002f 0020 007c 000a 0020 007c 0020 0020 005f 005f 005f 002f 0020 002f 005c \
            0020 005c 0020 007c 0020 0020 005f 0020 0020 002f 007c 0020 007c 005c 002f 007c 0020 \
            007c 000a 0020 007c 0020 007c 0020 0020 002f 0020 005f 005f 005f 005f 0020 005c 007c \
            0020 007c 0020 005c 0020 005c 007c 0020 007c 0020 0020 007c 0020 007c 000a 0020 007c \
            005f 007c 0020 002f 005f 002f 0020 0020 0020 0020 005c 005f 007c 005f 007c 0020 0020 \
            005c 005f 007c 005f 007c 0020 0020 007c 005f 007c 000a";

        assert_eq!(
            output,
            LogisimProgram {
                rom: expected_rom.to_owned(),
                ram: expected_ram.to_owned()
            }
        );
    }
}
