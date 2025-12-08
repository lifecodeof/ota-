# Otağ Programlama Dili

## Giriş

Otağ, Türkçe konuşan programcılar için tasarlanmış modern ve okunabilir bir programlama dilidir. Rust ile geliştirilmiş olan Otağ, hem yeni başlayanların programlamaya giriş yapmasını kolaylaştırmayı hem de deneyimli geliştiricilerin ana dillerinde kod yazabilmelerini sağlamayı hedefler.

Programlama dünyasında İngilizce'nin hâkimiyeti, birçok yetenekli insanın bu alana girmesini zorlaştırmaktadır. Otağ, bu engeli kaldırarak Türkiye'deki yazılım ekosisteminin gelişmesine katkıda bulunmayı amaçlar. Ana dilinizde düşünüp kod yazabilmek, sadece bir kolaylık değil, aynı zamanda daha derin bir anlayış ve yaratıcılık getirir.

## Neden Otağ?

### Anadilde Programlama

Türkçe söz dizimi ile kodunuzu daha doğal ve anlaşılır şekilde yazın. `eğer`, `döngü`, `fonksiyon` gibi anahtar kelimeler, mantığınızı ifade etmeyi kolaylaştırır.

### Modern ve Performanslı

Rust dilinin gücünden yararlanan Otağ, güvenli ve hızlı bir programlama deneyimi sunar. UTF-8 desteği ile Türkçe karakterleri (ğ, ü, ş, ö, ç, ı) eksiksiz destekler.

### Kolay Öğrenme Eğrisi

Sade ve tutarlı söz dizimi ile programlamaya yeni başlayanlar için ideal bir öğrenme ortamı sağlar. Karmaşık kavramları Türkçe terimlerle açıklayarak anlama sürecini hızlandırır.

### Pratik ve Fonksiyonel

Günlük yazılım geliştirme ihtiyaçlarını karşılayacak temel veri tipleri, kontrol yapıları, fonksiyonlar, diziler ve yapılar gibi özellikleri destekler.

## Kimler İçin?

- **Programlamaya Yeni Başlayanlar**: Ana dilinizde öğrenerek kavramları daha kolay kavrayın
- **Eğitimciler**: Türkçe kaynaklarla ders anlatarak öğrencilerinize daha etkili ulaşın
- **Deneyimli Geliştiriciler**: Ana dilinizde prototip geliştirin veya algoritmalar tasarlayın
- **Araştırmacılar**: Türkçe tabanlı hesaplama ve veri işleme uygulamaları oluşturun

## Vizyon

Otağ, sadece bir programlama dili değil, Türkiye'nin yazılım dünyasındaki yerini güçlendirme çabasıdır. Gelecekte daha fazla özellik, geniş bir standart kütüphane ve canlı bir geliştirici topluluğu ile Türk yazılım ekosisteminin vazgeçilmez bir parçası olmayı hedefliyoruz.

**Programlamanın evrensel olduğuna inanıyoruz, ancak öğrenmenin ana dilde başlaması gerektiğine de.**

---

## Başlarken

Bu kullanıcı kılavuzu, Otağ dilinin tüm temel özelliklerini kapsamlı bir şekilde açıklar. Kurulumdan başlayarak, değişken tanımlama, kontrol yapıları, fonksiyonlar ve veri yapılarına kadar her konuyu örneklerle destekleyerek anlatır.

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

## Gelecek Özellikler

Otağ sürekli gelişmektedir. Planlanan özellikler:

- Modül sistemi
- Hata yakalama
- Dosya işlemleri
- Ağ programlama
- Grafiksel arayüz desteği

---

Bu kılavuz Otağ programlama dilinin temel özelliklerini kapsamaktadır. Daha
fazla örnek için `examples/` klasörünü inceleyebilirsiniz.
