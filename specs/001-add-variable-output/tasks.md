# Tasks: Add Variable Output

**Input**: Design documents from `/specs/001-add-variable-output/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No test tasks included - tests not requested in feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Current Status Summary

- ✅ **Phase 1 (Setup)**: Complete
- ✅ **Phase 2 (Foundational)**: Complete  
- ✅ **Phase 3 (US1 - Basic Variables)**: Complete
- ✅ **Phase 4 (US2 - Multiple Types)**: Complete (with interpreter approach)
- 🔄 **Phase 5 (US3 - Expressions)**: Ready to start
- ⏳ **Phase 6 (Polish)**: Pending

**Next Priority**: Begin User Story 3 (Expressions with variables)

---

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- Single compiler project: `otağ-compiler/src/`, `otağ-compiler/tests/` at repository root
- Adjust based on plan.md structure

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create Rust project structure per implementation plan
- [x] T002 Initialize Cargo.toml with dependencies (logos, pest, inkwell)
- [x] T003 [P] Configure rustfmt and clippy

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Define type system in otağ-compiler/src/types.rs
- [x] T005 Define AST nodes in otağ-compiler/src/ast.rs
- [x] T006 Setup basic LLVM context in otağ-compiler/src/codegen.rs
- [x] T007 Create symbol table in otağ-compiler/src/symbol_table.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Declare and Output a Variable (Priority: P1) 🎯 MVP

**Goal**: Allow users to declare variables with Turkish keywords and output their values

**Independent Test**: Compile and run a program with one variable declaration and output statement, verifying correct output

### Implementation for User Story 1

- [x] T008 [US1] Implement lexer with Turkish keywords in otağ-compiler/src/lexer.rs
- [x] T009 [US1] Add parser rules for variable declaration in otağ-compiler/src/parser.rs
- [x] T010 [US1] Add codegen for variable allocation in otağ-compiler/src/codegen.rs
- [x] T011 [US1] Add parser rules for output statement in otağ-compiler/src/parser.rs
- [x] T012 [US1] Add codegen for output (printf) in otağ-compiler/src/codegen.rs
- [x] T013 [US1] Integrate lexer, parser, codegen in otağ-compiler/src/main.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Declare Variables of Different Types (Priority: P2)

**Goal**: Support declaring variables of different data types (integers, strings, floats, booleans)

**Independent Test**: Compile and run a program declaring variables of each supported type and outputting them

### Implementation for User Story 2

- [x] T014 [US2] Extend type system for all supported types in otağ-compiler/src/types.rs
- [x] T015 [US2] Update AST for type variants in otağ-compiler/src/ast.rs
- [x] T016 [US2] Switch to interpreter for easier testing in otağ-compiler/src/codegen.rs (replaces LLVM codegen)
- [x] T017 [US2] Add comprehensive unit tests for all data types

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

**Status Update**: US2 implementation complete with interpreter approach for better testability. All data types (int, string, float, bool) supported with comprehensive tests.

---

## Phase 5: User Story 3 - Output Expressions Involving Variables (Priority: P3)

**Goal**: Support outputting expressions that include variables and basic operations

**Independent Test**: Compile and run a program with variable assignments and output expressions (e.g., variable + literal)

### Implementation for User Story 3

 - [x] T018 [US3] Add expression parsing (variables, literals, +) in otağ-compiler/src/parser.rs
 - [x] T019 [US3] Add expression AST nodes in otağ-compiler/src/ast.rs
 - [x] T020 [US3] Add expression codegen in otağ-compiler/src/codegen.rs
 - [x] T021 [US3] Update output statement to handle expressions in otağ-compiler/src/parser.rs

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T022 [P] Documentation updates in README.md
- [ ] T023 Code cleanup and refactoring
- [ ] T024 [P] Run quickstart.md validation
- [ ] T025 Add error handling improvements

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
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Builds on US1 foundation but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Builds on US1/US2 but independently testable

### Within Each User Story

- Lexer/parser/codegen tasks can be parallelized where files differ
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks can run in parallel (different files)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Within US1: T008, T009, T010, T011, T012 can run in parallel
- Within US2: T014, T015, T016, T017 can run in parallel
- Within US3: T018, T019, T020, T021 can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch implementation tasks for User Story 1 together:
Task: "Implement lexer with Turkish keywords in otağ-compiler/src/lexer.rs"
Task: "Add parser rules for variable declaration in otağ-compiler/src/parser.rs"
Task: "Add codegen for variable allocation in otağ-compiler/src/codegen.rs"
Task: "Add parser rules for output statement in otağ-compiler/src/parser.rs"
Task: "Add codegen for output (printf) in otağ-compiler/src/codegen.rs"
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
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
