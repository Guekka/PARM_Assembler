#[cfg(test)]
mod tests {
    use parm_assembler::{export_to_logisim, LogisimProgram};

    #[test]
    fn one() {
        let input = "
        movs r0, #170
        movs r1, #255
        add sp, #16
        str r0, [sp, #4]
        str r1, [sp, #0]
        sub sp, #4
        ldr r2,[sp, #4]
        @r0 value should be 170, AA,
        @r1 and r2 values should be 255, FF,
        @In RAM, 04 value should be 255, FF,
        @In RAM, 05 value should be 170, AA,
        @Needs shift_add_sub_move and sp_address to be implemented";

        let output = export_to_logisim(input).unwrap();

        let expected = "v2.0 raw\n20aa 21ff b004 9001 9100 b081 9a01";

        assert_eq!(output, LogisimProgram::with_rom(expected.to_owned()));
    }

    #[test]
    fn two() {
        let input = "
        movs r0, #0
        movs r1, #1
        movs r2, #1 
        rsbs r2, r2, #0
        movs r3, #1
        rsbs r3, r3, #0
        lsrs r3, r1
        movs r4, #1
        
        
        
        cmn r3, r1
        @flag should be 1001
        
        cmn r2, r1
        @flag should be 0110
        
        cmp r4, r1
        @flag should be 0110
        
        cmp r0, r1
        @flag should be 1000
        
        cmp r1, r0
        @flag should be 0010
        
        @Needs shift_add_sub_move to be implemented";

        let output = export_to_logisim(input).unwrap();

        let expected = "v2.0 raw\n2000 2101 2201 4252 2301 425b 40cb 2401 42cb 42ca 428c 4288 4281";

        assert_eq!(output, LogisimProgram::with_rom(expected.to_owned()));
    }

    #[test]
    fn three() {
        let input = "
        movs r0, #0
        movs r1, #1
        movs r2, #170
        movs r3, #255
        
        movs r4, #15
        orrs r4, r2
        @r4 value should be 175, AF
        
        movs r5, #45
        muls r5, r2, r5
        @r5 value should be 7650, 1DE2
        
        movs r6, #19
        bics r6, r2
        @r6 value should be 17, 11
        
        mvns r7, r2
        @r7 value should be -171, FFFFFF55
        
        @Needs shift_add_sub_move to be implemented";

        let output = export_to_logisim(input).unwrap();

        let expected = "v2.0 raw\n2000 2101 22aa 23ff 240f 4314 252d 4355 2613 4396 43d7";
        assert_eq!(output, LogisimProgram::with_rom(expected.to_owned()));
    }

    #[test]
    fn four() {
        let input = "
        movs r0, #0
        movs r1, #1
        movs r2, #170
        movs r3, #255
        movs r4, #15
        
        rsbs r4, r4, #0
        @r4 value should be -15, FFFFFFF1
        
        asrs r4, r4, #1
        @r4 value should be -8, FFFFFFF8 
        
        movs r5, #5
        adcs r5, r1
        @r5 value should be 7, 7
        
        sbcs r5, r1
        sbcs r5, r1
        @r5 decimal value should be 4, 4
        
        movs r6, #170
        rors r6, r1
        @r5 value should be 85, 55
        
        tst r2, r6
        @flag Z should be activated, flags value should be 0100
        
        @Needs shift_add_sub_move to be implemented";

        let output = export_to_logisim(input).unwrap();

        let expected =
            "v2.0 raw\n2000 2101 22aa 23ff 240f 4264 1064 2505 414d 418d 418d 26aa 41ce 4232";

        assert_eq!(output, LogisimProgram::with_rom(expected.to_owned()));
    }
}
