#[cfg(test)]
mod tests {
    use parm_assembler::{export_to_logisim, LogisimProgram};

    #[test]
    fn one() {
        let input = "
        add sp, #16
        sub sp, #4
        @At the first high state, sp value is set to 4, at the second one it is set to 3";

        let output = export_to_logisim(input).unwrap();

        let expected = "v2.0 raw\nb004 b081";

        assert_eq!(output, LogisimProgram::with_rom(expected.to_owned()));
    }
}
