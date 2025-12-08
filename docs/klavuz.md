# Otağ Programlama Dili Kullanıcı Kılavuzu

## Giriş

Otağ, Türkçe konuşan programcılar için tasarlanmış basit ve okunabilir bir
programlama dilidir. Rust ile yazılmış bir derleyicisi vardır ve UTF-8 karakter
desteği ile tam Türkçe karakterleri (ğ, ü, ş, ö, ç, ı) destekler.

## Kurulum

### Ön Gereksinimler

- Rust 1.75 veya daha yeni sürüm
- Cargo (Rust ile birlikte gelir)

### Derleme

```bash
# Depoyu klonlayın
git clone https://github.com/lifecodeof/otag.git
cd otag

# Derleyiciyi oluşturun
cargo build --release
```

## Temel Kullanım

### İlk Programınız

`merhaba.otağ` adında bir dosya oluşturun:

```otağ
mesaj = "Merhaba Otağ!"
söyle mesaj
```

Çalıştırın:

```bash
cargo run merhaba.otağ
```

Çıktı:

```
Merhaba Otağ!
```

## Dil Söz Dizimi

### Değişken Tanımlama

Otağ'da değişkenler Türkçe iyelik eki ile tanımlanır:

```otağ
# Tamsayı değişken
yaş'ı tamsayı olarak tanımla
yaş = 25

# Metin değişken
isim = "Ahmet"

# Ondalıklı sayı
puan'ı ondalıklı olarak tanımla
puan = 85.5

# Mantıksal değer
durum'u mantıksal olarak tanımla
durum = doğru
```

### Veri Tipleri

- `tamsayı` - Tam sayı (i32)
- `metin` - Metin (string)
- `ondalıklı` - Ondalıklı sayı (f64)
- `mantıksal` - Mantıksal değer (true/false)

### İfadeler ve Aritmetik

```otağ
x = 10
y = 5

toplam = x + y
fark = x - y
çarpım = x * y
bölüm = x / y

söyle toplam   # 15
söyle çarpım   # 50
```

### Çıktı Verme

`söyle` komutu ile ekrana çıktı verebilirsiniz:

```otağ
söyle "Merhaba Dünya"
söyle 42
söyle x + y
```

## Kontrol Akışı

### Koşul İfadeleri

```otağ
yaş = 20

eğer yaş >= 18 ise
    söyle "Yetişkin"
yoksa
    söyle "Çocuk"
son
```

### Döngüler

```otağ
sayaç = 0

döngü sayaç < 5 ise
    söyle sayaç
    sayaç = sayaç + 1
son

söyle "Döngü tamamlandı!"
```

## Fonksiyonlar

### Fonksiyon Tanımlama

```otağ
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}

fonksiyon merhaba(isim: metin) -> metin {
    return "Merhaba " + isim + "!"
}
```

### Fonksiyon Çağırma

```otağ
sonuç = topla(5, 3)
söyle sonuç  # 8

mesaj = merhaba("Ahmet")
söyle mesaj  # Merhaba Ahmet!
```

## Veri Yapıları

### Diziler (Arrays)

```otağ
# Dizi tanımlama
sayılar = [1, 2, 3, 4, 5]

# Dizi elemanına erişim
söyle sayılar[0]  # 1
söyle sayılar[2]  # 3

# Dizi elemanı değiştirme
sayılar[1] = 10
söyle sayılar[1]  # 10
```

### Yapılar (Structs)

```otağ
# Yapı tanımlama
Öğrenci {
    isim: metin,
    yaş: tamsayı,
    not: ondalıklı
}

# Yapı örneği oluşturma
öğrenci1 = Öğrenci {
    isim: "Ahmet",
    yaş: 20,
    not: 85.5
}

# Yapı alanlarına erişim
söyle öğrenci1.isim  # Ahmet
söyle öğrenci1.yaş   # 20

# Yapı alanı değiştirme
öğrenci1.not = 90.0
söyle öğrenci1.not   # 90.0
```

### Dizi İşlemleri

```otağ
sayılar = [1, 2, 3, 4, 5]

# Dizi üzerinde döngü
için sayı in sayılar ise
    söyle sayı
son
```

## Örnek Programlar

### Basit Hesap Makinesi

```otağ
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}

fonksiyon çıkar(a: tamsayı, b: tamsayı) -> tamsayı {
    return a - b
}

x = 10
y = 5

söyle "Toplam: " + topla(x, y)
söyle "Fark: " + çıkar(x, y)
```

### Öğrenci Not Sistemi

```otağ
Öğrenci {
    isim: metin,
    vize: ondalıklı,
    final: ondalıklı
}

öğrenciler = [
    Öğrenci { isim: "Ahmet", vize: 75.0, final: 80.0 },
    Öğrenci { isim: "Ayşe", vize: 85.0, final: 90.0 }
]

fonksiyon ortalamaHesapla(vize: ondalıklı, final: ondalıklı) -> ondalıklı {
    return vize * 0.4 + final * 0.6
}

için öğrenci in öğrenciler ise
    ortalama = ortalamaHesapla(öğrenci.vize, öğrenci.final)
    söyle öğrenci.isim + " ortalaması: " + ortalama
son
```

## Hata Yönetimi

Otağ derleyicisi aşağıdaki durumlarda hatalar verir:

- Tanımlanmamış değişken kullanımı
- Tip uyumsuzluğu
- Dizi sınırları dışında erişim
- Söz dizimi hataları

Hata mesajları Türkçe olarak gösterilir ve hatanın konumunu belirtir.

## Gelişmiş Özellikler

### İç İçe Yapılar

```otağ
Adres {
    sokak: metin,
    şehir: metin,
    postaKodu: tamsayı
}

Kişi {
    isim: metin,
    yaş: tamsayı,
    adres: Adres
}

kişi = Kişi {
    isim: "Mehmet",
    yaş: 30,
    adres: Adres {
        sokak: "Ana Cadde",
        şehir: "İstanbul",
        postaKodu: 34000
    }
}

söyle kişi.adres.şehir  # İstanbul
```

## İpuçları ve En İyi Uygulamalar

1. **Değişken İsimleri**: Anlamlı ve açıklayıcı isimler kullanın
2. **Fonksiyonlar**: Kodunuzu yeniden kullanılabilir parçalara bölün
3. **Yorumlar**: Karmaşık kod parçalarını açıklayın (Otağ # ile yorum destekler)
4. **Tip Güvenliği**: Derleme zamanında tip kontrolü yapın
5. **Test Etme**: Kodunuzu düzenli olarak test edin

## Sorun Giderme

### Yaygın Hatalar

- **"Değişken tanımlanmamış"**: Değişkeni kullanmadan önce tanımladığınızdan
  emin olun
- **"Tip uyumsuzluğu"**: Atamaların ve işlemlerin tiplerinin uyumlu olduğundan
  emin olun
- **"Söz dizimi hatası"**: Noktalama işaretlerini ve anahtar kelimeleri doğru
  kullandığınızdan emin olun

### Yardım Alma

Sorularınız için:

- GitHub Issues sayfasını kullanın
- Örnek kodları inceleyin
- Test dosyalarını çalıştırın

## Modül Sistemi

Otağ, kodunuzu farklı dosyalara bölerek düzenli ve yeniden kullanılabilir hale
getirmenize olanak tanır.

### Modül İçe Aktarma

Başka bir Otağ dosyasındaki fonksiyonları ve tanımları kullanmak için `kullan`
anahtar kelimesini kullanın:

```otağ
kullan "matematik.otağ"
```

Tüm içe aktarma yolları, kaynak dosyaya göre görecelidir.

### Örnek: Basit Modül

`matematik.otağ` dosyası:

```otağ
# Matematik fonksiyonları modülü

fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}

fonksiyon double(x: tamsayı) -> tamsayı {
    return x + x
}
```

`main.otağ` dosyası:

```otağ
kullan "matematik.otağ"

x'ı tamsayı olarak tanımla
x = 10

y'ı tamsayı olarak tanımla
y = 5

sonuç'ı tamsayı olarak tanımla
sonuç = topla(x, y)

söyle sonuç  # 15
```

### İç İçe İçe Aktarmalar

Modüller başka modülleri içe aktarabilir. Otağ, tüm bağımlılıkları otomatik
olarak yükler:

`utils.otağ`:

```otağ
fonksiyon double(x: tamsayı) -> tamsayı {
    return x + x
}
```

`advanced_math.otağ`:

```otağ
kullan "utils.otağ"

fonksiyon quadruple(x: tamsayı) -> tamsayı {
    d'ı tamsayı olarak tanımla
    d = double(x)
    return d + d
}
```

`main.otağ`:

```otağ
kullan "advanced_math.otağ"

n'ı tamsayı olarak tanımla
n = 5

sonuç'ı tamsayı olarak tanımla
sonuç = quadruple(n)

söyle sonuç  # 20
```

### Döngüsel İçe Aktarma Koruması

Otağ, döngüsel içe aktarmaları (A dosyası B'yi, B dosyası A'yı içe aktarır)
otomatik olarak algılar ve sonsuz döngüleri önler. Her dosya yalnızca bir kez
yüklenir.

## Gelecek Özellikler

Otağ sürekli gelişmektedir. Planlanan özellikler:

- Hata yakalama
- Dosya işlemleri
- Ağ programlama
- Grafiksel arayüz desteği

---

Bu kılavuz Otağ programlama dilinin temel özelliklerini kapsamaktadır. Daha
fazla örnek için `examples/` klasörünü inceleyebilirsiniz.
