# Quickstart: Error Handling

**Feature**: 005-add-error-handling  
**Date**: 2025-10-17  

## Basic Try-Catch Example

```otağ
dene {
    x = 5 / 0
} yakala hata {
    söyle("Hata yakalandı: " + hata.mesaj)
} sonunda {
    söyle("Her zaman çalışır")
}
```

## Throwing Custom Errors

```otağ
eğer x < 0 ise {
    fırlat "Negatif sayı hatası"
}
```

## Error Propagation

```otağ
fonksiyon böl(a, b) {
    eğer b == 0 ise {
        fırlat "Sıfıra bölme hatası"
    }
    döndür a / b
}
```

## Best Practices
- Use specific error messages
- Include location information
- Provide suggestions for fixes
- Handle errors gracefully
