# Quickstart: Control Flow Statements

**Feature**: Control Flow Statements
**Date**: 2025-10-17

## Overview

Control flow statements allow Otağ programs to make decisions and repeat actions. This feature adds Turkish-localized keywords for conditional execution and loops.

## Basic Conditional Logic

### If/Else Statements

```otağ
yaş'ı tamsayı olarak tanımla
yaş = 20

eğer yaş >= 18 ise
    söyle "Yetişkin"
yoksa
    söyle "Çocuk"
son
```

**Output**: `Yetişkin`

### Boolean Conditions

```otağ
doğru_mu = yanlış

eğer doğru_mu ise
    söyle "Evet"
yoksa
    söyle "Hayır"
son
```

**Output**: `Hayır`

## While Loops

### Basic Counting Loop

```otağ
sayac = 0

döngü sayac < 5 ise
    sayac = sayac + 1
    söyle sayac
son
```

**Output**:
```
1
2
3
4
5
```

### Accumulator Pattern

```otağ
toplam = 0

döngü toplam < 10 ise
    toplam = toplam + 2
    söyle toplam
son
```

**Output**:
```
2
4
6
8
10
```

## For Loops

### Simple Range Iteration

```otağ
için i in 0'dan 5'e ise
    söyle i
son
```

**Output**:
```
0
1
2
3
4
```

### Range with Custom Step

```otağ
için sayı in 0'dan 10'e adım 2 ise
    söyle sayı
son
```

**Output**:
```
0
2
4
6
8
```

### Calculations in Loops

```otağ
için i in 1'dan 4'e ise
    söyle i * 3
son
```

**Output**:
```
3
6
9
12
```

## Break and Continue

### Using Break

```otağ
sayac = 0

döngü doğru ise
    sayac = sayac + 1
    eğer sayac > 3 ise
        durdur
    son
    söyle sayac
son
```

**Output**:
```
1
2
3
```

### Using Continue

```otağ
için i in 0'dan 6'e ise
    eğer i == 3 ise
        devam
    son
    söyle i
son
```

**Output**:
```
0
1
2
4
5
```

## Complete Example

```otağ
# Age checking with loops
yaş'ı tamsayı olarak tanımla
yaş = 16

eğer yaş >= 18 ise
    söyle "Yetişkin - tüm özellikler kullanılabilir"

    # Demonstrate for loop
    söyle "Sayılar:"
    için i in 1'dan 4'e ise
        söyle i
    son
yoksa
    söyle "Çocuk modu - sınırlı özellikler"

    # Demonstrate while loop with break
    sayac = 0
    döngü doğru ise
        sayac = sayac + 1
        eğer sayac > 3 ise
            durdur
        son
        söyle sayac
    son
son
```

## Running Programs

1. Save code to `control-flow.otağ`
2. Compile and run: `cargo run control-flow.otağ`

## Syntax Rules

- **Conditions**: Must evaluate to `doğru` (true) or `yanlış` (false)
- **Blocks**: Always end with `son` keyword
- **Ranges**: Use `dan` (from) and `e` (to), optional `adım` (step)
- **Break/Continue**: Only valid inside loops
- **Infinite Loop Protection**: Loops automatically stop after 10,000 iterations

## Error Messages

The compiler provides helpful error messages:
- "Expected 'ise' after condition, found 'is'"
- "Break statement only valid inside loops"
- "Loop exceeded maximum iterations (10000)"
