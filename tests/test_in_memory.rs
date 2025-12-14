// Integration tests using the in-memory testing API
// This demonstrates how to test Otağ programs without creating physical files

use otag::{OtagRuntime, VirtualFileSystem};

#[test]
fn test_simple_inline_execution() {
    // Execute a simple program inline without any files
    let source = r#"
x'ı tamsayı olarak tanımla
x = 42
söyle x
"#;

    let result = OtagRuntime::execute_inline(source);
    assert!(
        result.is_ok(),
        "Failed to execute inline program: {:?}",
        result.err()
    );
}

#[test]
fn test_arithmetic_operations() {
    let source = r#"
a'ı tamsayı olarak tanımla
a = 10

b'ı tamsayı olarak tanımla
b = 20

toplam'ı tamsayı olarak tanımla
toplam = a + b

söyle toplam
"#;

    let result = OtagRuntime::execute_inline(source);
    assert!(result.is_ok(), "Arithmetic test failed: {:?}", result.err());
}

#[test]
fn test_in_memory_import_simple() {
    let mut runtime = OtagRuntime::new();

    // Define a helper module in memory
    runtime.add_source(
        "helpers.otağ",
        r#"
fonksiyon iki_katı(x: tamsayı) -> tamsayı {
    return x + x
}
"#,
    );

    // Main program that uses the helper
    runtime.add_source(
        "program.otağ",
        r#"
kullan "helpers.otağ"

sayı'ı tamsayı olarak tanımla
sayı = 7

sonuç'ı tamsayı olarak tanımla
sonuç = iki_katı(sayı)

söyle sonuç
"#,
    );

    let result = runtime.execute("program.otağ");
    assert!(result.is_ok(), "Import test failed: {:?}", result.err());
}

#[test]
fn test_multi_module_interaction() {
    let mut runtime = OtagRuntime::new();

    // Module 1: Math utilities
    runtime.add_source(
        "math.otağ",
        r#"
fonksiyon çarp(x: tamsayı, y: tamsayı) -> tamsayı {
    sonuç'ı tamsayı olarak tanımla
    sonuç = 0
    
    sayaç'ı tamsayı olarak tanımla
    sayaç = 0
    
    döngü sayaç < y ise
        sonuç = sonuç + x
        sayaç = sayaç + 1
    son
    
    return sonuç
}
"#,
    );

    // Module 2: String utilities
    runtime.add_source(
        "strings.otağ",
        r#"
fonksiyon selamla(isim: metin) -> metin {
    return "Merhaba " + isim
}
"#,
    );

    // Main program using both modules
    runtime.add_source(
        "app.otağ",
        r#"
kullan "math.otağ"
kullan "strings.otağ"

x'ı tamsayı olarak tanımla
x = 3

y'ı tamsayı olarak tanımla
y = 4

çarpım'ı tamsayı olarak tanımla
çarpım = çarp(x, y)

söyle çarpım

mesaj'ı metin olarak tanımla
mesaj = selamla("Dünya")

söyle mesaj
"#,
    );

    let result = runtime.execute("app.otağ");
    assert!(
        result.is_ok(),
        "Multi-module test failed: {:?}",
        result.err()
    );
}

#[test]
fn test_virtual_filesystem_directly() {
    let mut vfs = VirtualFileSystem::new();

    // Add files to VFS
    vfs.add_file("test.otağ", "x'ı tamsayı olarak tanımla");
    vfs.add_file("another.otağ", "y'ı metin olarak tanımla");

    // Verify files exist
    assert!(vfs.file_exists("test.otağ"));
    assert!(vfs.file_exists("another.otağ"));
    assert!(!vfs.file_exists("nonexistent.otağ"));

    // Verify file contents
    assert_eq!(
        vfs.get_file("test.otağ"),
        Some("x'ı tamsayı olarak tanımla")
    );
    assert_eq!(
        vfs.get_file("another.otağ"),
        Some("y'ı metin olarak tanımla")
    );
    assert_eq!(vfs.get_file("nonexistent.otağ"), None);
}

#[test]
fn test_complex_nested_imports() {
    let mut runtime = OtagRuntime::new();

    // Base layer: basic math
    runtime.add_source(
        "math.otağ",
        r#"
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}
"#,
    );

    // Middle layer: uses base
    runtime.add_source(
        "advanced.otağ",
        r#"
kullan "math.otağ"

fonksiyon toplam_üç(a: tamsayı, b: tamsayı, c: tamsayı) -> tamsayı {
    temp'ı tamsayı olarak tanımla
    temp = topla(a, b)
    return topla(temp, c)
}
"#,
    );

    // Top layer: uses middle layer
    runtime.add_source(
        "app.otağ",
        r#"
kullan "advanced.otağ"

sonuç'ı tamsayı olarak tanımla
sonuç = toplam_üç(5, 10, 15)

söyle sonuç
"#,
    );

    let result = runtime.execute("app.otağ");
    assert!(
        result.is_ok(),
        "Complex nested imports failed: {:?}",
        result.err()
    );
}

#[test]
fn test_conditional_logic() {
    let source = r#"
yaş'ı tamsayı olarak tanımla
yaş = 20

eğer yaş >= 18 ise
    söyle "Yetişkin"
son
"#;

    let result = OtagRuntime::execute_inline(source);
    assert!(
        result.is_ok(),
        "Conditional logic test failed: {:?}",
        result.err()
    );
}

#[test]
fn test_while_loop_inline() {
    let source = r#"
sayaç'ı tamsayı olarak tanımla
sayaç = 0

döngü sayaç < 5 ise
    söyle sayaç
    sayaç = sayaç + 1
son
"#;

    let result = OtagRuntime::execute_inline(source);
    assert!(result.is_ok(), "While loop test failed: {:?}", result.err());
}

#[test]
fn test_function_with_return_value() {
    let source = r#"
fonksiyon fibonacci(n: tamsayı) -> tamsayı {
    eğer n <= 1 ise
        return n
    son
    
    a'ı tamsayı olarak tanımla
    a = 0
    
    b'ı tamsayı olarak tanımla
    b = 1
    
    sayaç'ı tamsayı olarak tanımla
    sayaç = 2
    
    döngü sayaç <= n ise
        temp'ı tamsayı olarak tanımla
        temp = a + b
        a = b
        b = temp
        sayaç = sayaç + 1
    son
    
    return b
}

sonuç'ı tamsayı olarak tanımla
sonuç = fibonacci(6)

söyle sonuç
"#;

    let result = OtagRuntime::execute_inline(source);
    assert!(
        result.is_ok(),
        "Function with return value test failed: {:?}",
        result.err()
    );
}
