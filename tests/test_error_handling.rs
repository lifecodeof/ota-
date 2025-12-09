use std::process::Command;

#[test]
fn test_undefined_variable_error() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "/tmp/test_undefined_var.otağ"])
        .output()
        .expect("Failed to execute command");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Tanımlanmamış değişken"));
    assert!(stderr.contains("Çalışma Zamanı Hatası") || stderr.contains("Anlamsal Hata"));
}

#[test]
fn test_type_mismatch_error() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "/tmp/test_type_mismatch.otağ"])
        .output()
        .expect("Failed to execute command");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Tür uyumsuzluğu") || stderr.contains("toplanamaz"));
}

#[test]
fn test_array_bounds_error() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "/tmp/test_array_bounds.otağ"])
        .output()
        .expect("Failed to execute command");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Dizi indeksi sınırların dışında") || stderr.contains("indeksi"));
}

#[test]
fn test_infinite_loop_protection() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "/tmp/test_infinite_loop.otağ"])
        .output()
        .expect("Failed to execute command");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("maksimum iterasyon") || stderr.contains("Sonsuz döngü"));
}
