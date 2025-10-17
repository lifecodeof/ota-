# Tasks: Add Functions

**Input**: Design documents from `/specs/003-add-functions/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Following TDD approach per constitution - tests are included and should be written first.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths adjusted based on plan.md structure

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization for function support

- [ ] T001 Verify Rust 1.75 and LLVM dependencies are configured per plan.md
- [ ] T002 [P] Update Cargo.toml with any new dependencies from research.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure extensions that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T003 Extend AST in src/ast.rs with Function and Call node definitions from data-model.md
- [X] T004 Extend symbol table in src/symbol_table.rs to store function definitions
- [X] T005 Update lexer in src/lexer.rs with tokens for function keywords ("fonksiyon", "->")
- [X] T006 Update grammar in src/grammar.pest with rules from contracts/grammar.md
- [X] T007 Extend types.rs if needed for function return types

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Define a Function (Priority: P1) 🎯 MVP

**Goal**: Enable programmers to define reusable code blocks with parameters and body

**Independent Test**: Define a function without parameters and verify it parses without errors

### Tests for User Story 1 ⚠️

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T008 [P] [US1] Unit test for function definition parsing in tests/unit/test_parser.rs
- [X] T009 [P] [US1] Integration test for function definition in tests/integration/test_functions.rs

### Implementation for User Story 1

- [X] T010 [US1] Implement function definition parsing in src/parser.rs using grammar rules
- [X] T011 [US1] Add function storage to symbol table in src/symbol_table.rs
- [X] T012 [US1] Generate LLVM IR for function definitions in src/codegen.rs
- [ ] T013 [US1] Add error handling for invalid function definitions

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Invoke a Function (Priority: P2)

**Goal**: Enable programmers to call defined functions with arguments

**Independent Test**: Invoke a defined function and verify execution

### Tests for User Story 2 ⚠️

- [X] T014 [P] [US2] Unit test for function call parsing in tests/unit/test_parser.rs
- [X] T015 [P] [US2] Integration test for function invocation in tests/integration/test_functions.rs

### Implementation for User Story 2

- [X] T016 [US2] Implement function call parsing in src/parser.rs
- [X] T017 [US2] Generate LLVM IR for function calls in src/codegen.rs
- [X] T018 [US2] Add runtime error handling for undefined function calls
- [X] T019 [US2] Integrate with symbol table for function lookup

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Use Function Return Values (Priority: P3)

**Goal**: Enable programmers to use values returned by functions

**Independent Test**: Define function with return, invoke it, and use the result in expressions

### Tests for User Story 3 ⚠️

- [X] T020 [P] [US3] Unit test for return statement parsing in tests/unit/test_parser.rs
- [X] T021 [P] [US3] Integration test for return value usage in tests/integration/test_functions.rs

### Implementation for User Story 3

- [X] T022 [US3] Implement return statement parsing in src/parser.rs
- [X] T023 [US3] Generate LLVM IR for return statements in src/codegen.rs
- [X] T024 [US3] Support return values in function calls
- [X] T025 [US3] Add type checking for return values

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T026 [P] Update documentation in README.md with function examples
- [ ] T027 Code cleanup and refactoring across src/ files
- [ ] T028 Performance optimization for function compilation
- [ ] T029 [P] Additional unit tests in tests/unit/
- [ ] T030 Run quickstart.md validation and update if needed
- [ ] T031 Full integration test suite for all function features

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User stories proceed in priority order (P1 → P2 → P3)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
- **User Story 2 (P2)**: Depends on User Story 1 (needs function definitions to invoke)
- **User Story 3 (P3)**: Depends on User Story 2 (needs invocation to test returns)

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Parsing before code generation
- Core implementation before error handling
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks can run in parallel (within Phase 2)
- Once Foundational phase completes, user stories proceed sequentially due to dependencies
- All tests for a user story marked [P] can run in parallel
- Different implementation tasks within a story can be parallel if different files

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for function definition parsing in tests/unit/test_parser.rs"
Task: "Integration test for function definition in tests/integration/test_functions.rs"

# Launch implementation tasks:
Task: "Implement function definition parsing in src/parser.rs"
Task: "Add function storage to symbol table in src/symbol_table.rs"
Task: "Generate LLVM IR for function definitions in src/codegen.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2 (after US1 complete)
   - Developer C: User Story 3 (after US2 complete)
3. Stories complete and integrate sequentially due to dependencies

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
