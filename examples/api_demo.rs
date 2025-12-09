// Example demonstrating the in-memory testing API capabilities
// This is a standalone example file showing how to use OtagRuntime

use otag::{OtagRuntime, error_reporting::OtagError};

fn main() -> Result<(), OtagError> {
    println!("=== Otağ In-Memory Testing API Demo ===\n");

    // Example 1: Simple inline execution
    println!("1. Simple inline execution:");
    let simple_program = r#"
mesaj'ı metin olarak tanımla
mesaj = "Merhaba, Otağ dünyası!"
söyle mesaj
"#;
    OtagRuntime::execute_inline(simple_program)?;
    println!();

    // Example 2: Program with variables and arithmetic
    println!("2. Variables and arithmetic:");
    let math_program = r#"
x'ı tamsayı olarak tanımla
x = 10

y'ı tamsayı olarak tanımla
y = 20

toplam'ı tamsayı olarak tanımla
toplam = x + y

söyle "Toplam:"
söyle toplam
"#;
    OtagRuntime::execute_inline(math_program)?;
    println!();

    // Example 3: Multi-module program with virtual imports
    println!("3. Multi-module program with virtual imports:");
    let mut runtime = OtagRuntime::new();

    // Define utility modules
    runtime.add_source(
        "math_utils.otağ",
        r#"
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}
"#,
    );

    runtime.add_source(
        "string_utils.otağ",
        r#"
fonksiyon mesaj_olustur() -> metin {
    return "Modül sistemi çalışıyor!"
}
"#,
    );

    // Main application using both modules
    runtime.add_source(
        "app.otağ",
        r#"
kullan "math_utils.otağ"
kullan "string_utils.otağ"

# Math operations
a'ı tamsayı olarak tanımla
a = 5

b'ı tamsayı olarak tanımla
b = 3

toplam'ı tamsayı olarak tanımla
toplam = topla(a, b)

söyle toplam

# String operations
mesaj'ı metin olarak tanımla
mesaj = mesaj_olustur()
söyle mesaj
"#,
    );

    runtime.execute("app.otağ")?;
    println!();

    // Example 4: Nested imports
    println!("4. Nested module imports:");
    let mut runtime2 = OtagRuntime::new();

    runtime2.add_source(
        "base.otağ",
        r#"
fonksiyon iki_katı(x: tamsayı) -> tamsayı {
    return x + x
}
"#,
    );

    runtime2.add_source(
        "advanced.otağ",
        r#"
kullan "base.otağ"

fonksiyon dört_katı(x: tamsayı) -> tamsayı {
    temp'ı tamsayı olarak tanımla
    temp = iki_katı(x)
    return iki_katı(temp)
}
"#,
    );

    runtime2.add_source(
        "main.otağ",
        r#"
kullan "advanced.otağ"

sayı'ı tamsayı olarak tanımla
sayı = 7

sonuç'ı tamsayı olarak tanımla
sonuç = dört_katı(sayı)

söyle sonuç
"#,
    );

    runtime2.execute("main.otağ")?;
    println!();

    println!("=== Demo completed successfully! ===");
    Ok(())
}
