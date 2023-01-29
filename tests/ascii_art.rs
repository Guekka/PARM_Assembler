#[cfg(test)]
mod tests {
    #[test]
    fn string() {
        let input = r#"
            .asciz "Hello, world!"
            .end:
            b .end
        "#;

        let output = parm_assembler::export_to_logisim(input).unwrap();

        let expected =
            "v2.0 raw\n0048 0065 006c 006c 006f 002c 0020 0077 006f 0072 006c 0064 0021 e7fe";

        assert_eq!(output, expected);
    }
}
