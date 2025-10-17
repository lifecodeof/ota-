use std::process::Command;

#[test]
fn test_function_definition_integration() {
    let program = r#"
fonksiyon selamla(isim: metin) {
    söyle "Merhaba " + isim
}

selamla("Dünya")
"#;

    // Write to temp file
    std::fs::write("temp.otag", program).expect("Failed to write temp file");

    // Run compiler (assuming it outputs to stdout for now)
    let output = Command::new("cargo")
        .args(&["run", "--", "temp.otag"])
        .output()
        .expect("Failed to run compiler");

    // Check if compilation succeeded (no error in stderr)
    assert!(output.stderr.is_empty(), "Compilation failed: {}", String::from_utf8_lossy(&output.stderr));

    // For now, just check that it doesn't crash
    // In future, check actual output when execution is implemented
    assert!(true);
}

#[test]
fn test_function_invocation_integration() {
    let program = r#"
fonksiyon topla(a: tamsayı, b: tamsayı) {
    return a + b
}

sonuç = topla(5, 3)
söyle sonuç
"#;

    // Write to temp file
    std::fs::write("temp2.otag", program).expect("Failed to write temp file");

    // Run compiler
    let output = Command::new("cargo")
        .args(&["run", "--", "temp2.otag"])
        .output()
        .expect("Failed to run compiler");

    // Check compilation
    assert!(output.stderr.is_empty(), "Compilation failed: {}", String::from_utf8_lossy(&output.stderr));
}
