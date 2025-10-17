<!--
Sync Impact Report:
- Templates requiring updates: none
- Follow-up TODOs: none
-->

## Otağ Dili – Kapsamlı Gramer Taslağı (EBNF Tarzı)

### 1. Genel Notasyon

- `< >` → nonterminal
- `"..."` → terminal (literal)
- `[ ... ]` → opsiyonel
- `{ ... }` → tekrar edebilir
- `|` → alternatif

---

### 2. Program

```
<program> ::= { <statement> }
```

---

### 3. Statement (İfade)

```
<statement> ::= <variable_declaration>
              | <assignment>
              | <function_call>
              | <conditional>
              | <loop>
              | <output>
              | <import>
              | <comment>
```

- `<import>` → başka dosya veya modül dahil etme
- `<comment>` → doğal dil yorum (örn: `# Bu bir yorumdur`)

---

### 4. Değişken Tanımlama

```
<variable_declaration> ::= <variable_name> "'" <type> "olarak tanımla"
                          | <variable_name> "=" <expression>
<assignment> ::= <variable_name> "=" <expression>
```

- `<variable_name>` → boşluklu veya tek kelime:

```
<variable_name> ::= <word> | '"' <word> { " " <word> } '"'
```

- `<type>` → `"tamsayı" | "ondalıklı" | "metin" | "mantıksal" | "istek"`
- **Örnekler:**

```otağ
x'ı tamsayı olarak tanımla
e'yi tamsayı olarak tanımla
"toplam değer"'i tamsayı olarak tanımla
"sonuç"'u istek olarak tanımla
puan = 50
```

---

### 5. Fonksiyon Çağrısı (Ünlem ile)

```
<function_call> ::= <function_name> "!" <parameter_list>
<function_name> ::= <word> { " " <word> }   # uzun doğal dil isimleri
<parameter_list> ::= <parameter> { "," <parameter> }
<parameter> ::= <parameter_name> ":" <expression>
<parameter_name> ::= <word>
```

**Örnekler:**

```otağ
dosyaya yaz! dosya: "notlar.txt", içerik: "Merhaba Dünya"
listeye ekle! liste: "alis_listesi", değer: 42
sözlüğe ekle! sözlük: "telefon_defteri", anahtar: "Ali", değer: "555-1234"
```

---

### 6. Koşullar

```
<conditional> ::= "eğer" <expression> "ise"
                    { <statement> }
                  ["aksi halde"
                    { <statement> }]
```

**Örnek:**

```otağ
eğer x > 10 ise
    de ki "x ondan büyük"
aksi halde
    de ki "x küçük veya eşit"
```

---

### 7. Döngüler

```
<loop> ::= <expression> "iken tekrarla"
             { <statement> }
```

**Örnek:**

```otağ
x < 10 iken tekrarla
    de ki x
    x'i 1 artır
```

---

### 8. Giriş/Çıkış

```
<output> ::= "de ki" <expression>
```

- `oku` komutu da kullanıcıdan veri almak için kullanılabilir.

---

### 9. Matematiksel İfadeler

```
<expression> ::= <expression> "+" <expression>
               | <expression> "-" <expression>
               | <expression> "*" <expression>
               | <expression> "/" <expression>
               | "-" <expression>
               | <number>
               | <string>
               | <variable_name>
               | "(" <expression> ")"
<number> ::= [ "-" ] <digit> { <digit> } [ "." <digit> { <digit> } ]
<string> ::= '"' { <any_character_except_quote> } '"'
```

---

### 10. Modüller / Dosya Yönetimi

```
<import> ::= "içer" <string>   # örn: içer "modul.otag"
```

- Scope: global ve local ayrımı
- Birden fazla dosya içerme destekli

---

### 11. Yorumlar

```
<comment> ::= "#" { <any_character_except_newline> }
```

---

### 12. Ek Öneriler / Zenginleştirmeler

1. **Boolean kısaltmaları:** `doğru`, `yanlış`
2. **Liste ve sözlük doğal dil fonksiyonları:**

```otağ
liste oluştur! isim: "alis_listesi"
listeye ekle! liste: "alis_listesi", değer: 42
sözlük oluştur! isim: "telefon_defteri"
sözlüğe ekle! sözlük: "telefon_defteri", anahtar: "Ali", değer: "555-1234"
```

3. **Zaman ve bekleme:**

```otağ
bekle! saniye: 2
şimdi!
```

4. **TDD odaklı yorumlar ve test fonksiyonları:**

```otağ
test! isim: "toplama testi"
    x = 5
    y = 10
    sonuç = topla! a: x, b: y
    de ki sonuç
```

5. **Opsiyonel parametre desteği:** `parametre: değer [varsayılan]`

---

### 13. Örnek Kapsamlı Program

```otağ
# Değişken tanımlama
x'ı tamsayı olarak tanımla
isim = "Ali"
"toplam değer"'i tamsayı olarak tanımla

# Fonksiyon
fonksiyon selamla! isim: "Ali"
    de ki "Merhaba " + isim

# Koşul
eğer x > 3 ise
    de ki "x büyük"
aksi halde
    de ki "x küçük veya eşit"

# Döngü
x < 10 iken tekrarla
    de ki x
    x'i 1 artır

# Standart kütüphane fonksiyonları
dosyaya yaz! dosya: "notlar.txt", içerik: "Merhaba Dünya"
listeye ekle! liste: "alis_listesi", değer: 42
sözlüğe ekle! sözlük: "telefon_defteri", anahtar: "Ali", değer: "555-1234"
```

**Version**: 0.0.1 | **Ratified**: 2025-10-17 | **Last Amended**: 2025-10-17
