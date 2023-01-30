#[cfg(test)]
mod tests {
    use parm_assembler::LogisimProgram;

    #[test]
    fn one() {
        let input = "
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
        @r3 value should be 70, 46";

        let output = parm_assembler::export_to_logisim(input).unwrap();

        let expected = "v2.0 raw\n2000 2101 2214 4288 d4ff e7ff 4252 428a dbff e000 2032 e7f4 1883";

        assert_eq!(output, LogisimProgram::with_rom(expected.to_owned()));
    }
}
