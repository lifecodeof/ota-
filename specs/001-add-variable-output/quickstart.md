# Quickstart: Variable Declaration and Output

**Feature**: Add Variable Output  
**Date**: 2025-10-17  

## Overview

This feature adds support for declaring variables with Turkish keywords and outputting their values in Otağ programs.

## Basic Usage

### Declaring Variables

```otağ
x'ı tamsayı olarak tanımla
isim = "Merhaba Dünya"
puan'ı ondalıklı olarak tanımla
puan = 95.5
durum'u mantıksal olarak tanımla
durum = doğru
```

### Outputting Values

```otağ
söyle x
söyle isim
söyle puan + 4.5
söyle durum
```

## Complete Example

```otağ
# Variable declarations
yaş'ı tamsayı olarak tanımla
yaş = 25
mesaj = "Merhaba Otağ!"

# Output
söyle yaş
söyle mesaj
```

## Running Programs

1. Save code to `program.otağ`
2. Compile: `otag-compiler program.otağ`
3. Run: `./program`

## Notes

- Variables are case-sensitive
- Use snake_case for names
- Uninitialized variables default to type-appropriate zeros
