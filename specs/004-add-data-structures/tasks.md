# Tasks: Add Data Structures

**Input**: Design documents from `/specs/004-add-data-structures/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Following TDD workflow from constitution - tests are included and should be written first.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths follow the compiler structure from plan.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Feature-specific setup and basic extensions

- [ ] T001 Extend Token enum in src/lexer.rs for array/struct syntax ([, ], {, }, ., :)
- [ ] T002 Extend Type enum in src/types.rs for Array and Struct types
- [ ] T003 Extend Expression enum in src/ast.rs for new AST nodes
- [ ] T004 Extend Statement enum in src/ast.rs for struct definitions

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core AST and type system extensions that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 [P] Implement ArrayLiteral AST node in src/ast.rs
- [ ] T006 [P] Implement StructLiteral AST node in src/ast.rs
- [ ] T007 [P] Implement ArrayAccess AST node in src/ast.rs
- [ ] T008 [P] Implement StructAccess AST node in src/ast.rs
- [ ] T009 [P] Implement StructDefinition AST node in src/ast.rs
- [ ] T010 Update symbol table in src/symbol_table.rs to store struct definitions
- [ ] T011 Extend type checker in src/types.rs for array/struct validation

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Declare and Use Arrays (Priority: P1) 🎯 MVP

**Goal**: Enable declaration and basic usage of arrays with Turkish syntax

**Independent Test**: Compile and run a program that declares an array, accesses elements, and outputs values

### Tests for User Story 1 ⚠️

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T012 [P] [US1] Unit test for array literal parsing in tests/unit/test_parser.rs
- [ ] T013 [P] [US1] Unit test for array access parsing in tests/unit/test_parser.rs
- [ ] T014 [P] [US1] Unit test for array type checking in tests/unit/test_types.rs
- [ ] T015 [P] [US1] Unit test for array code generation in tests/unit/test_codegen.rs
- [ ] T016 [US1] Integration test for array declaration and access in tests/integration/test_arrays.rs

### Implementation for User Story 1

- [ ] T017 [US1] Implement array literal parsing in src/parser.rs
- [ ] T018 [US1] Implement array access parsing in src/parser.rs
- [ ] T019 [US1] Implement array type checking in src/types.rs
- [ ] T020 [US1] Implement array code generation in src/codegen.rs
- [ ] T021 [US1] Update lexer in src/lexer.rs for array tokens

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Declare and Use Structs (Priority: P2)

**Goal**: Enable definition and basic usage of structs with Turkish syntax

**Independent Test**: Compile and run a program that defines a struct, creates instances, accesses fields, and outputs values

### Tests for User Story 2 ⚠️

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T022 [P] [US2] Unit test for struct definition parsing in tests/unit/test_parser.rs
- [ ] T023 [P] [US2] Unit test for struct literal parsing in tests/unit/test_parser.rs
- [ ] T024 [P] [US2] Unit test for struct access parsing in tests/unit/test_parser.rs
- [ ] T025 [P] [US2] Unit test for struct type checking in tests/unit/test_types.rs
- [ ] T026 [P] [US2] Unit test for struct code generation in tests/unit/test_codegen.rs
- [ ] T027 [US2] Integration test for struct definition and usage in tests/integration/test_structs.rs

### Implementation for User Story 2

- [ ] T028 [US2] Implement struct definition parsing in src/parser.rs
- [ ] T029 [US2] Implement struct literal parsing in src/parser.rs
- [ ] T030 [US2] Implement struct access parsing in src/parser.rs
- [ ] T031 [US2] Implement struct type checking in src/types.rs
- [ ] T032 [US2] Implement struct code generation in src/codegen.rs
- [ ] T033 [US2] Update lexer in src/lexer.rs for struct tokens

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Array and Struct Operations (Priority: P3)

**Goal**: Enable advanced operations like iteration and copying for arrays and structs

**Independent Test**: Compile and run programs that iterate over arrays, copy structs, and perform operations on collections

### Tests for User Story 3 ⚠️

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T034 [P] [US3] Unit test for array iteration in loops in tests/unit/test_parser.rs
- [ ] T035 [P] [US3] Unit test for struct copying in tests/unit/test_codegen.rs
- [ ] T036 [P] [US3] Unit test for nested structures in tests/unit/test_types.rs
- [ ] T037 [US3] Integration test for array/struct operations in tests/integration/test_operations.rs

### Implementation for User Story 3

- [ ] T038 [US3] Implement array iteration in control flow parsing in src/parser.rs
- [ ] T039 [US3] Implement struct copying operations in src/codegen.rs
- [ ] T040 [US3] Implement nested structure support in src/types.rs
- [ ] T041 [US3] Add bounds checking for array operations in src/codegen.rs

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T042 [P] Add comprehensive error messages for array/struct operations in src/types.rs
- [ ] T043 [P] Optimize code generation for arrays and structs in src/codegen.rs
- [ ] T044 [P] Add documentation comments for new AST nodes in src/ast.rs
- [ ] T045 [P] Update main.rs to handle new syntax in CLI if needed
- [ ] T046 Run quickstart.md validation and update if needed
- [ ] T047 Execute full test suite and ensure all tests pass

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Depends on US1 and US2 for operations

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD workflow)
- Parsing before type checking
- Type checking before code generation
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, US1 and US2 can start in parallel
- US3 depends on US1 and US2 completion
- All tests for a user story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for array literal parsing in tests/unit/test_parser.rs"
Task: "Unit test for array access parsing in tests/unit/test_parser.rs"
Task: "Unit test for array type checking in tests/unit/test_types.rs"
Task: "Unit test for array code generation in tests/unit/test_codegen.rs"

# Then implementation tasks sequentially:
Task: "Implement array literal parsing in src/parser.rs"
Task: "Implement array access parsing in src/parser.rs"
Task: "Implement array type checking in src/types.rs"
Task: "Implement array code generation in src/codegen.rs"
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
   - Developer A: User Story 1 (Arrays)
   - Developer B: User Story 2 (Structs)
3. Stories complete and integrate independently
4. Developer C: User Story 3 (Operations) after US1/US2 done

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
