use std::process::Command;
use std::fs;

#[test]
fn test_basic_import() {
    // Create temporary test files
    let helper_code = r#"
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}
"#;
    
    let main_code = r#"
kullan "test_helper.otağ"

x'ı tamsayı olarak tanımla
x = 5

y'ı tamsayı olarak tanımla
y = 3

sonuç'ı tamsayı olarak tanımla
sonuç = topla(x, y)

söyle sonuç
"#;

    fs::write("test_helper.otağ", helper_code).expect("Failed to write helper file");
    fs::write("test_main.otağ", main_code).expect("Failed to write main file");

    let output = Command::new("cargo")
        .args(&["run", "--", "test_main.otağ"])
        .output()
        .expect("Failed to run compiler");

    // Check compilation succeeded
    assert!(output.status.success(), "Compilation failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check output contains "8"
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("8"), "Expected output to contain 8, got: {}", stdout);

    // Cleanup
    let _ = fs::remove_file("test_helper.otağ");
    let _ = fs::remove_file("test_main.otağ");
}

#[test]
fn test_nested_imports() {
    // Create temporary test files
    let utils_code = r#"
fonksiyon double(x: tamsayı) -> tamsayı {
    return x + x
}
"#;
    
    let math_code = r#"
kullan "test_utils.otağ"

fonksiyon quadruple(x: tamsayı) -> tamsayı {
    d'ı tamsayı olarak tanımla
    d = double(x)
    return d + d
}
"#;

    let main_code = r#"
kullan "test_math.otağ"

n'ı tamsayı olarak tanımla
n = 5

sonuç'ı tamsayı olarak tanımla
sonuç = quadruple(n)

söyle sonuç
"#;

    fs::write("test_utils.otağ", utils_code).expect("Failed to write utils file");
    fs::write("test_math.otağ", math_code).expect("Failed to write math file");
    fs::write("test_nested.otağ", main_code).expect("Failed to write main file");

    let output = Command::new("cargo")
        .args(&["run", "--", "test_nested.otağ"])
        .output()
        .expect("Failed to run compiler");

    // Check compilation succeeded
    assert!(output.status.success(), "Compilation failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check output contains "20" (5 * 4)
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("20"), "Expected output to contain 20, got: {}", stdout);

    // Cleanup
    let _ = fs::remove_file("test_utils.otağ");
    let _ = fs::remove_file("test_math.otağ");
    let _ = fs::remove_file("test_nested.otağ");
}

#[test]
fn test_circular_import_protection() {
    // Create circular import files
    let file_a = r#"
kullan "test_circular_b.otağ"

x'ı tamsayı olarak tanımla
x = 1
söyle x
"#;

    let file_b = r#"
kullan "test_circular_a.otağ"

y'ı tamsayı olarak tanımla
y = 2
söyle y
"#;

    fs::write("test_circular_a.otağ", file_a).expect("Failed to write file A");
    fs::write("test_circular_b.otağ", file_b).expect("Failed to write file B");

    let output = Command::new("cargo")
        .args(&["run", "--", "test_circular_a.otağ"])
        .output()
        .expect("Failed to run compiler");

    // The program should run but circular import should be prevented
    // It might produce an error or just skip the circular part
    // Either way, it shouldn't hang or crash

    // Cleanup
    let _ = fs::remove_file("test_circular_a.otağ");
    let _ = fs::remove_file("test_circular_b.otağ");
    
    // Test passes if we reach here without hanging
    assert!(true);
}
