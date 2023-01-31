#[cfg(test)]
mod tests {
    use parm_assembler::{export_to_logisim, LogisimProgram};

    #[test]
    fn two_strings() {
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
        .file   "two_string.c"
        .globl  run
        .p2align        2
        .type   run,%function
        .code   16
        .thumb_func
run:
        .fnstart
        .pad    #100
        sub     sp, #100
        @APP
        sub     sp, #508
        @NO_APP
        @APP
        sub     sp, #452
        @NO_APP
        ldr     r0, .LCPI0_0
        str     r0, [sp, #12]
        ldr     r0, .LCPI0_1
        str     r0, [sp, #8]
        movs    r0, #0
        str     r0, [sp, #4]
        b       .LBB0_1
.LBB0_1:
        ldr     r0, [sp, #4]
        cmp     r0, #11
        bgt     .LBB0_6
        b       .LBB0_2
.LBB0_2:
        b       .LBB0_3
.LBB0_3:
        ldr     r0, [sp, #12]
        ldr     r1, [sp, #4]
        ldrb    r0, [r0, r1]
        str     r0, [sp, #36]
        b       .LBB0_4
.LBB0_4:
        b       .LBB0_5
.LBB0_5:
        ldr     r0, [sp, #4]
        adds    r0, r0, #1
        str     r0, [sp, #4]
        b       .LBB0_1
.LBB0_6:
        movs    r0, #0
        str     r0, [sp]
        b       .LBB0_7
.LBB0_7:
        ldr     r0, [sp]
        cmp     r0, #7
        bgt     .LBB0_12
        b       .LBB0_8
.LBB0_8:
        b       .LBB0_9
.LBB0_9:
        ldr     r0, [sp, #8]
        ldr     r1, [sp]
        ldrb    r0, [r0, r1]
        str     r0, [sp, #36]
        b       .LBB0_10
.LBB0_10:
        b       .LBB0_11
.LBB0_11:
        ldr     r0, [sp]
        adds    r0, r0, #1
        str     r0, [sp]
        b       .LBB0_7
.LBB0_12:
        b       .LBB0_13
.LBB0_13:
        b       .LBB0_14
.LBB0_14:
        b       .LBB0_14
        .p2align        2
.LCPI0_0:
        .long   .L.str
.LCPI0_1:
        .long   .L.str.1
.Lfunc_end0:
        .size   run, .Lfunc_end0-run
        .cantunwind
        .fnend

        .type   .L.str,%object
        .section        .rodata.str1.1,"aMS",%progbits,1
.L.str:
        .asciz  "Hello world\n"
        .size   .L.str, 13

        .type   .L.str.1,%object
.L.str.1:
        .asciz  "Goodbye!"
        .size   .L.str.1, 9


        .ident  "clang version 8.0.1- (branches/release_80)"
        .section        ".note.GNU-stack","",%progbits
        .addrsig
        .eabi_attribute 30, 6
        "#;

        let actual = export_to_logisim(input).unwrap();

        println!("{:#?}", actual);

        let expected_rom = "v2.0 raw\nb099 b0ff b0f1 2000 9003 200c 9002 2000 9001 e7fe 9801 280b \
         dc0b e7fe e7fe 9803 9901 1846 6830 9009 e7fe e7fe 9801 1c40 9001 e7ee 2000 9000 e7fe 9800 \
          2807 dc0b e7fe e7fe 9802 9900 1846 6830 9009 e7fe e7fe 9800 1c40 9000 e7ee e7fe e7fe e7fd";

        let expected_ram = "v2.0 raw\n0048 0065 006c 006c 006f 0020 0077 006f 0072 006c 0064 \
        000a 0047 006f 006f 0064 0062 0079 0065 0021";

        assert_eq!(
            actual,
            LogisimProgram {
                rom: expected_rom.to_string(),
                ram: expected_ram.to_string(),
            }
        );
    }
}
