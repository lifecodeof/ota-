# Feature Specification: Add Variable Output

**Feature Branch**: `001-add-variable-output`  
**Created**: 2025-10-17  
**Status**: Draft  
**Input**: User description: "Implement basic variable declaration and output functionality in Otağ language, allowing users to declare variables using Turkish keywords like 'tanımla' and display values with 'söyle', enabling simple programs that store and print data."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Declare and Output a Variable (Priority: P1)

As a programmer learning Otağ, I want to declare a variable using Turkish keywords and output its value so that I can create simple programs that store and display data.

**Why this priority**: This is the foundational functionality for any program in Otağ, enabling basic data storage and display without which no useful programs can be written.

**Independent Test**: Can be fully tested by writing a single-line program with variable declaration and output, compiling it, and verifying the output matches the declared value.

**Acceptance Scenarios**:

1. **Given** a new Otağ program file, **When** I write `x'ı tamsayı olarak tanımla` followed by `söyle x`, **Then** the program compiles and outputs the default value for an uninitialized integer variable (assumed 0).
2. **Given** a new Otağ program file, **When** I write `isim = "Merhaba"` followed by `söyle isim`, **Then** the program compiles and outputs "Merhaba".

---

### User Story 2 - Declare Variables of Different Types (Priority: P2)

As a programmer, I want to declare variables of different data types (integers, strings, etc.) so that I can store various kinds of data in my programs.

**Why this priority**: Supports basic data handling needs, allowing programs to work with numbers and text, which is essential for most simple applications.

**Independent Test**: Can be tested independently by declaring variables of each supported type and outputting them, verifying correct type handling.

**Acceptance Scenarios**:

1. **Given** a program with `puan'ı tamsayı olarak tanımla` and `puan = 100`, **When** executed with `söyle puan`, **Then** outputs 100.
2. **Given** a program with `mesaj = "Otağ dili"` and `durum'u mantıksal olarak tanımla` with `durum = doğru`, **When** executed with `söyle mesaj` and `söyle durum`, **Then** outputs "Otağ dili" and true.

---

### User Story 3 - Output Expressions Involving Variables (Priority: P3)

As a programmer, I want to output expressions that include variables and simple operations so that I can display computed results.

**Why this priority**: Adds basic computation capability, making programs more useful for calculations and dynamic output.

**Independent Test**: Can be tested by declaring variables, performing simple operations in output, and verifying correct results.

**Acceptance Scenarios**:

1. **Given** variables `a = 5` and `b = 10`, **When** executing `söyle a + b`, **Then** outputs 15.
2. **Given** string variable `selam = "Merhaba"`, **When** executing `söyle selam + " Dünya"`, **Then** outputs "Merhaba Dünya".

---

### Edge Cases

- What happens when a variable is declared with an invalid type keyword (e.g., "x'ı bilinmeyen olarak tanımla")?
- How does the system handle outputting an undeclared variable?
- What occurs when declaring a variable with a name that includes invalid characters or starts with a number?
- How are type mismatches handled in assignments (e.g., assigning a string to an integer variable)?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow declaring variables using the "tanımla" keyword with supported types: tamsayı (integer), metin (string), ondalıklı (float), mantıksal (boolean).
- **FR-002**: System MUST allow declaring variables with direct assignment syntax (e.g., `x = 5`).
- **FR-003**: System MUST support outputting expressions using the "söyle" keyword, displaying results to the console.
- **FR-004**: System MUST support basic arithmetic and string concatenation in output expressions.
- **FR-005**: System MUST provide clear error messages for syntax errors in declarations or invalid operations.

### Key Entities *(include if feature involves data)*

- **Variable**: Represents a named storage location with a type and value; supports snake_case naming with UTF-8 characters; can be declared explicitly with type or implicitly through assignment.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Programmers can write, compile, and execute a simple Otağ program with variable declarations and output in under 5 minutes.
- **SC-002**: 100% of valid programs with variable declarations and output statements execute without runtime errors.
- **SC-003**: Compiler successfully parses all supported declaration and output syntax without syntax errors.
- **SC-004**: Users report high satisfaction (90% positive) with the Turkish keyword intuitiveness for basic operations.

