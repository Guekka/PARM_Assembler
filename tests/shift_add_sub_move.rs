#[cfg(test)]
mod tests {
    use parm_assembler::export_to_logisim;

    #[test]
    fn lsls() {
        let input = "lsls r4, r3, #7";
        let expected = "v2.0 raw\n01dc";
        assert_eq!(export_to_logisim(input).unwrap(), expected);
    }

    #[test]
    fn one() {
        let input = "
            movs r0, #0
            movs r1, #1
            movs r2, #170
            movs r3, #255
            
            
            lsls r4, r2, #1
            @r4 value should be 340, 154 
            
            lsrs r5, r2, #1
            @r5 value should be 85, 55 
            
            subs r6, r0, #5
            asrs r6, r6, #1
            @r6 value should be -3 ou FFFFFFFD
            
            adds r7, r6, r1 
            @r7 value should be -2, FFFFFFFE 
        ";

        let output = export_to_logisim(input).unwrap();

        let expected = "v2.0 raw
2000 2101 22aa 23ff 0054 0855 1f46 1076 1877";

        assert_eq!(expected, output);
    }

    #[test]
    fn two() {
        let input = "
        movs r0, #0
        movs r1, #1
        movs r2, #170
        movs r3, #255
        
        subs r4, r3, r2
        @r4 value should be 85, 55
        
        adds r5, r2, #5
        @r4 value should be 175, AF
        
        movs r6, #179
        @r6 value should be 179, B3
        ";

        let output = export_to_logisim(input).unwrap();

        let expected = "v2.0 raw
2000 2101 22aa 23ff 1a9c 1d55 26b3";

        assert_eq!(expected, output);
    }
}
