# Feature Specification: Add Data Structures

**Feature Branch**: `004-add-data-structures`  
**Created**: 2025-10-17  
**Status**: Draft  
**Input**: User description: "Add Data Structures including structs"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Declare and Use Arrays (Priority: P1)

As an Otağ programmer, I want to declare arrays to store collections of values of the same type, so that I can work with multiple related data items efficiently.

**Why this priority**: Arrays are fundamental data structures that enable handling collections, which is essential for most practical programs beyond simple variables.

**Independent Test**: Can be fully tested by declaring an array, assigning values to elements, and outputting array contents, verifying correct storage and access.

**Acceptance Scenarios**:

1. **Given** a new Otağ program, **When** I write `sayılar = [1, 2, 3, 4, 5]` followed by `söyle sayılar[0]`, **Then** the program outputs 1.
2. **Given** an array declaration `isimler = ["Ali", "Veli", "Ayşe"]`, **When** I write `söyle isimler[2]`, **Then** the program outputs "Ayşe".
3. **Given** an array `puanlar = [85, 92, 78]`, **When** I write `puanlar[1] = 95` followed by `söyle puanlar[1]`, **Then** the program outputs 95.

---

### User Story 2 - Declare and Use Structs (Priority: P2)

As an Otağ programmer, I want to define custom data structures (structs) to group related data of different types, so that I can create more complex data models.

**Why this priority**: Structs enable structured data representation, allowing programmers to model real-world entities with multiple attributes.

**Independent Test**: Can be fully tested by defining a struct, creating instances, accessing fields, and modifying values, verifying correct field access and type safety.

**Acceptance Scenarios**:

1. **Given** a struct definition `Öğrenci { isim: metin, yaş: tamsayı, not: ondalıklı }`, **When** I create `öğrenci1 = Öğrenci { isim: "Ahmet", yaş: 20, not: 85.5 }` and write `söyle öğrenci1.isim`, **Then** the program outputs "Ahmet".
2. **Given** a struct instance `kitap = Kitap { başlık: "Otağ Rehberi", yazar: "Anonim", sayfa: 100 }`, **When** I write `kitap.sayfa = 120` followed by `söyle kitap.sayfa`, **Then** the program outputs 120.
3. **Given** nested structs, **When** I define a struct containing another struct and access nested fields, **Then** the program correctly accesses the nested data.

---

### User Story 3 - Array and Struct Operations (Priority: P3)

As an Otağ programmer, I want to perform operations on arrays and structs like iteration and copying, so that I can manipulate collections and structured data effectively.

**Why this priority**: Operations enable practical use of data structures in algorithms and data processing tasks.

**Independent Test**: Can be fully tested by performing operations like iterating over arrays, copying structs, and verifying results.

**Acceptance Scenarios**:

1. **Given** an array `sayılar = [1, 2, 3]`, **When** I use a loop to iterate `için sayı in sayılar ise söyle sayı`, **Then** the program outputs each number on separate lines.
2. **Given** a struct instance `orijinal = Nokta { x: 10, y: 20 }`, **When** I create `kopya = orijinal` and modify `kopya.x = 15`, **Then** `orijinal.x` remains 10 while `kopya.x` is 15.
3. **Given** arrays of structs, **When** I access struct fields within array elements, **Then** the program correctly retrieves the data.

---

### Edge Cases

- What happens when accessing an array index that is out of bounds?
- How does the system handle declaring arrays with mixed types?
- What occurs when trying to access a non-existent struct field?
- How are type mismatches handled in struct field assignments?
- What happens when declaring structs with circular references?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support declaring arrays with square bracket syntax `[value1, value2, ...]` for same-type elements.
- **FR-002**: System MUST support accessing array elements using index notation `array[index]`.
- **FR-003**: System MUST support modifying array elements by assignment `array[index] = value`.
- **FR-004**: System MUST support defining structs with field declarations using syntax `StructName { field1: type1, field2: type2 }`.
- **FR-005**: System MUST support creating struct instances with field initialization `StructName { field1: value1, field2: value2 }`.
- **FR-006**: System MUST support accessing struct fields using dot notation `instance.field`.
- **FR-007**: System MUST support modifying struct fields by assignment `instance.field = value`.
- **FR-008**: System MUST enforce type safety for array elements and struct fields, including recursive type checking for nested structures.
- **FR-009**: System MUST provide clear error messages for invalid array/struct operations.
- **FR-010**: System MUST support arrays and structs in control flow and function contexts.
- **FR-011**: System MUST support arrays and structs as function parameters and return values.

### Key Entities *(include if feature involves data)*

- **Array**: Represents a fixed-size collection of elements of the same type, accessible by index; supports basic types (integer, string, float, boolean).
- **Struct**: Represents a custom data type with named fields of potentially different types; enables grouping related data into logical units.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Programmers can declare and use arrays in programs within 3 minutes of learning the syntax.
- **SC-002**: 95% of valid array and struct operations execute without runtime errors.
- **SC-003**: Programs using arrays and structs compile successfully for all supported type combinations.
- **SC-004**: Users report improved ability to model real-world data, with 80% satisfaction in handling collections and structured data.

