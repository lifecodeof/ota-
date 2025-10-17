# Feature Specification: Control Flow Statements

**Feature Branch**: `002-add-control-flow`
**Created**: 2025-10-17
**Status**: Draft
**Input**: User description: "Control Flow Statements - Add eğer (if), yoksa (else), döngü (while/for loops)"

## Clarifications

### Session 2025-10-17
- Q: What are the performance expectations for control flow execution? → A: Minimal performance requirements - Any working implementation is acceptable, performance is not a priority
- Q: How should the system handle infinite loops? → A: Detect and prevent - System automatically stops loops that appear infinite after a certain number of iterations
- Q: Should break and continue statements be included in the control flow feature? → A: Include basic break/continue - Support break (exit loop) and continue (skip to next iteration) statements
- Q: What should be the exact syntax for specifying ranges in for loops? → A: Turkish keywords - Use "dan/de" for from/to, "adım" for step (e.g., "0'dan 5'e adım 2")
- Q: What standards should error messages follow to be considered "clear"? → A: Descriptive with suggestions - Include what was wrong and suggest fixes (e.g., "Expected 'ise' after condition, found 'is'")

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Conditional Logic (Priority: P1)

Otağ programmers need to execute different code paths based on conditions, using Turkish keywords "eğer" (if) and "yoksa" (else) for conditional execution.

**Why this priority**: This is the most fundamental control flow construct - without conditionals, programs cannot make decisions, making the language very limited.

**Independent Test**: Can be fully tested by writing programs that use if/else statements to make decisions and output different results based on variable values.

**Acceptance Scenarios**:

1. **Given** a variable `yaş = 20`, **When** writing `eğer yaş >= 18 ise söyle "Yetişkin" yoksa söyle "Çocuk"`, **Then** program outputs "Yetişkin"
2. **Given** a boolean variable `doğru_mu = yanlış`, **When** writing `eğer doğru_mu ise söyle "Evet" yoksa söyle "Hayır"`, **Then** program outputs "Hayır"
3. **Given** a comparison `5 > 3`, **When** writing `eğer 5 > 3 ise söyle "Doğru" yoksa söyle "Yanlış"`, **Then** program outputs "Doğru"
4. **Given** nested conditions, **When** using multiple if/else blocks, **Then** the correct branch executes based on the first true condition

---

### User Story 2 - While Loops (Priority: P2)

Otağ programmers need to repeat code execution while a condition remains true, using the Turkish keyword "döngü" (while) for iterative execution.

**Why this priority**: While loops enable repetitive tasks and are essential for most programming scenarios. They're simpler to implement than for loops and cover most iteration needs.

**Independent Test**: Can be fully tested by writing programs that use while loops to count, accumulate values, or repeat actions until a condition is met.

**Acceptance Scenarios**:

1. **Given** a counter variable `sayac = 0`, **When** writing `döngü sayac < 5 ise sayac = sayac + 1; söyle sayac`, **Then** program outputs numbers 1 through 5
2. **Given** an accumulator `toplam = 0`, **When** writing `döngü toplam < 10 ise toplam = toplam + 2; söyle toplam`, **Then** program outputs 2, 4, 6, 8, 10
3. **Given** a condition that becomes false, **When** writing `döngü yanlış ise söyle "Bu çalışmaz"`, **Then** loop body never executes
4. **Given** nested while loops, **When** using loops inside loops, **Then** both loops execute correctly with proper nesting

---

### User Story 3 - For Loops (Priority: P3)

Otağ programmers need to iterate over ranges or collections with a more structured loop syntax, using "için" (for) keyword for range-based iteration.

**Why this priority**: For loops provide a convenient syntax for iterating over known ranges, making certain patterns easier to write and read.

**Independent Test**: Can be fully tested by writing programs that iterate over numeric ranges and perform operations on each iteration.

**Acceptance Scenarios**:

1. **Given** a range `0'dan 5'e`, **When** writing `için i in 0'dan 5'e ise söyle i`, **Then** program outputs 0, 1, 2, 3, 4, 5
2. **Given** a range with step, **When** writing `için i in 0'dan 10'e adım 2 ise söyle i`, **Then** program outputs 0, 2, 4, 6, 8, 10
3. **Given** calculations in loop, **When** writing `için i in 1'dan 3'e ise söyle i * 2`, **Then** program outputs 2, 4, 6

---

### Edge Cases

- What happens when if/else conditions contain invalid expressions?
- System automatically stops loops that appear infinite after a certain number of iterations
- What happens when loop variables are modified inside the loop?
- How are nested control structures handled?
- What happens with empty loop bodies?
- How does system handle break/continue statements?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support "eğer" (if) statements with boolean conditions
- **FR-002**: System MUST support "yoksa" (else) clauses for alternative execution paths
- **FR-003**: System MUST support "döngü" (while) loops that execute while conditions remain true
- **FR-004**: System MUST support "için" (for) loops for range-based iteration
- **FR-005**: System MUST evaluate boolean expressions in control flow conditions
- **FR-006**: System MUST support nested control structures (loops inside conditionals, etc.)
- **FR-007**: System MUST handle empty control blocks without syntax errors
- **FR-008**: System MUST provide descriptive error messages with suggestions for malformed control structures (e.g., "Expected 'ise' after condition, found 'is'")
- **FR-009**: System MUST detect and prevent infinite loops by automatically stopping loops after a reasonable number of iterations
- **FR-010**: System MUST support "durdur" (break) statements to exit loops prematurely
- **FR-011**: System MUST support "devam" (continue) statements to skip to the next loop iteration
- **FR-012**: System MUST use Turkish keywords for for loop ranges: "dan" (from), "e" (to), "adım" (step)

### Key Entities *(include if feature involves data)*

- **ControlBlock**: Represents if/else or loop bodies containing statements
- **Condition**: Boolean expression that determines execution path
- **LoopVariable**: Variable used in for loops (optional, can be auto-generated)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can write conditional programs that execute different paths based on variable values
- **SC-002**: Users can create loops that iterate until conditions are met
- **SC-003**: Programs with control flow statements execute in correct order (conditions evaluated before bodies)
- **SC-004**: Nested control structures work correctly with proper scoping
- **SC-005**: Error messages for control flow syntax are clear and helpful
- **SC-006**: Control flow statements execute without excessive delays for typical educational programs

