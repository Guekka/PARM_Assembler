#[cfg(test)]
mod tests {
    use parm_assembler::LogisimProgram;

    #[test]
    fn caesar() {
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
        .eabi_attribute 21, 0
        .eabi_attribute 23, 3
        .eabi_attribute 24, 1
        .eabi_attribute 25, 1
        .eabi_attribute 38, 1
        .eabi_attribute 18, 4
        .eabi_attribute 26, 2
        .eabi_attribute 14, 0
        .file   "fibonaci.c"
        .globl  run
        .p2align        1
        .type   run,%function
        .code   16
        .thumb_func
run:
        .fnstart
        .pad    #96
        sub     sp, #96
        @APP
        sub     sp, #508
        @NO_APP
        @APP
        sub     sp, #452
        @NO_APP
        movs    r0, #0
        str     r0, [sp, #8]
        movs    r0, #1
        str     r0, [sp, #4]
        b       .LBB0_1
.LBB0_1:
        ldr     r0, [sp, #4]
        str     r0, [sp, #36]
        b       .LBB0_2
.LBB0_2:
        ldr     r0, [sp, #76]
        str     r0, [sp, #28]
        ldr     r0, [sp, #28]
        cmp     r0, #0
        bne     .LBB0_6
        b       .LBB0_3
.LBB0_3:
        b       .LBB0_4
.LBB0_4:
        movs    r0, #48
        str     r0, [sp, #32]
        b       .LBB0_5
.LBB0_5:
        b       .LBB0_18
.LBB0_6:
        movs    r0, #0
        str     r0, [sp, #24]
        str     r0, [sp, #20]
        b       .LBB0_7
.LBB0_7:
        ldr     r0, [sp, #20]
        cmp     r0, #7
        bhi     .LBB0_17
        b       .LBB0_8
.LBB0_8:
        ldr     r0, [sp, #28]
        movs    r1, #15
        ands    r0, r1
        str     r0, [sp, #16]
        ldr     r0, [sp, #28]
        lsrs    r0, r0, #4
        str     r0, [sp, #28]
        ldr     r0, [sp, #24]
        cmp     r0, #0
        bne     .LBB0_13
        b       .LBB0_9
.LBB0_9:
        ldr     r0, [sp, #16]
        cmp     r0, #0
        beq     .LBB0_11
        b       .LBB0_10
.LBB0_10:
        movs    r0, #1
        str     r0, [sp, #24]
        b       .LBB0_12
.LBB0_11:
        b       .LBB0_16
.LBB0_12:
        b       .LBB0_13
.LBB0_13:
        b       .LBB0_14
.LBB0_14:
        ldr     r0, [sp, #16]
        adds    r0, #48
        str     r0, [sp, #32]
        b       .LBB0_15
.LBB0_15:
        b       .LBB0_16
.LBB0_16:
        ldr     r0, [sp, #20]
        adds    r0, r0, #1
        str     r0, [sp, #20]
        b       .LBB0_7
.LBB0_17:
        b       .LBB0_18
.LBB0_18:
        b       .LBB0_19
.LBB0_19:
        movs    r0, #10
        str     r0, [sp, #32]
        b       .LBB0_20
.LBB0_20:
        ldr     r0, [sp, #8]
        ldr     r1, [sp, #4]
        adds    r0, r0, r1
        str     r0, [sp]
        ldr     r0, [sp, #4]
        str     r0, [sp, #8]
        ldr     r0, [sp]
        str     r0, [sp, #4]
        b       .LBB0_1
.Lfunc_end0:
        .size   run, .Lfunc_end0-run
        .cantunwind
        .fnend

        .ident  "clang version 10.0.0-4ubuntu1 "
        .section        ".note.GNU-stack","",%progbits
        .addrsig
        .eabi_attribute 30, 6
"#;

        let expected = "v2.0 raw\nb098 b0ff b0f1 2000 9002 2001 9001 e7fe 9801 9009 e7fe 9813 9007 9807 2800 d104 e7fe e7fe 2030 9008 e7fe e025 2000 9006 9005 e7fe 9805 2807 d81d e7fe 9807 210f 4008 9004 9807 0900 9007 9806 2800 d108 e7fe 9804 2800 d002 e7fe 2001 9006 e7ff e005 e7fe e7fe 9804 3030 9008 e7fe e7fe 9805 1c40 9005 e7dc e7fe e7fe 200a 9008 e7fe 9802 9901 1840 9000 9801 9002 9800 9001 e7bc";

        let output = parm_assembler::export_to_logisim(input).unwrap();

        assert_eq!(output, LogisimProgram::with_rom(expected.to_owned()));
    }
}
