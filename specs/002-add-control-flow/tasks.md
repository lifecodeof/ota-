# Tasks: Control Flow Statements

**Input**: Design documents from `/specs/002-add-control-flow/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Following TDD principles from constitution, tests are included for each user story to ensure quality and prevent regressions.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths shown below assume single project - adjust based on plan.md structure

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create project structure per implementation plan
- [ ] T002 Initialize Rust project with pest, logos, clap dependencies
- [ ] T003 [P] Configure linting and formatting tools

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Extend AST with control flow nodes in src/ast.rs
- [ ] T005 [P] Add Turkish control flow keywords to lexer in src/lexer.rs
- [ ] T006 Extend grammar with control flow rules in src/grammar.pest
- [ ] T007 Extend parser with control flow parsing functions in src/parser.rs
- [ ] T008 Extend interpreter with control flow execution logic in src/codegen.rs
- [ ] T009 Extend symbol table for loop variable scoping in src/symbol_table.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Basic Conditional Logic (Priority: P1) 🎯 MVP

**Goal**: Enable conditional execution using Turkish keywords "eğer" (if) and "yoksa" (else) for decision-making in programs

**Independent Test**: Can be fully tested by writing programs that use if/else statements to make decisions and output different results based on variable values

### Tests for User Story 1 ⚠️

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T010 [P] [US1] Unit tests for if/else parsing in tests/parser_tests.rs
- [ ] T011 [P] [US1] Unit tests for if/else execution in tests/codegen_tests.rs
- [ ] T012 [P] [US1] Integration test for if/else programs in tests/integration_tests.rs

### Implementation for User Story 1

- [ ] T013 [US1] Implement if_statement parsing in src/parser.rs
- [ ] T014 [US1] Implement if_statement execution in src/codegen.rs
- [ ] T015 [US1] Add if/else example to examples/control-flow.otağ

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - While Loops (Priority: P2)

**Goal**: Enable repetitive execution using Turkish keyword "döngü" (while) for iterative tasks

**Independent Test**: Can be fully tested by writing programs that use while loops to count, accumulate values, or repeat actions until a condition is met

### Tests for User Story 2 ⚠️

- [ ] T016 [P] [US2] Unit tests for while loop parsing in tests/parser_tests.rs
- [ ] T017 [P] [US2] Unit tests for while loop execution in tests/codegen_tests.rs
- [ ] T018 [P] [US2] Integration test for while loop programs in tests/integration_tests.rs

### Implementation for User Story 2

- [ ] T019 [US2] Implement while_loop parsing in src/parser.rs
- [ ] T020 [US2] Implement while_loop execution with infinite loop protection in src/codegen.rs
- [ ] T021 [US2] Add while loop example to examples/loops.otağ

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - For Loops (Priority: P3)

**Goal**: Enable structured iteration over ranges using "için" (for) keyword for range-based loops

**Independent Test**: Can be fully tested by writing programs that iterate over numeric ranges and perform operations on each iteration

### Tests for User Story 3 ⚠️

- [ ] T022 [P] [US3] Unit tests for for loop parsing in tests/parser_tests.rs
- [ ] T023 [P] [US3] Unit tests for for loop execution in tests/codegen_tests.rs
- [ ] T024 [P] [US3] Integration test for for loop programs in tests/integration_tests.rs

### Implementation for User Story 3

- [ ] T025 [US3] Implement for_loop parsing in src/parser.rs
- [ ] T026 [US3] Implement for_loop execution with loop variable management in src/codegen.rs
- [ ] T027 [US3] Add for loop example to examples/loops.otağ

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: User Story 4 - Break and Continue Statements (Priority: P4)

**Goal**: Enable loop control using "durdur" (break) and "devam" (continue) statements

**Independent Test**: Can be fully tested by writing programs that use break to exit loops early and continue to skip iterations

### Tests for User Story 4 ⚠️

- [ ] T028 [P] [US4] Unit tests for break/continue parsing in tests/parser_tests.rs
- [ ] T029 [P] [US4] Unit tests for break/continue execution in tests/codegen_tests.rs
- [ ] T030 [P] [US4] Integration test for break/continue programs in tests/integration_tests.rs

### Implementation for User Story 4

- [ ] T031 [US4] Implement break_statement and continue_statement parsing in src/parser.rs
- [ ] T032 [US4] Implement break/continue execution logic in src/codegen.rs
- [ ] T033 [US4] Add break/continue examples to examples/loops.otağ

**Checkpoint**: All control flow features should now be complete

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T034 [P] Documentation updates in README.md and examples/
- [ ] T035 Code cleanup and refactoring across src/
- [ ] T036 Performance optimization for control flow execution
- [ ] T037 [P] Additional unit tests for edge cases in tests/
- [ ] T038 Security hardening for infinite loop protection
- [ ] T039 Run quickstart.md validation with example programs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Depends on US2 and US3 for loop contexts

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Parsing before execution
- Core implementation before examples
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit tests for if/else parsing in tests/parser_tests.rs"
Task: "Unit tests for if/else execution in tests/codegen_tests.rs"
Task: "Integration test for if/else programs in tests/integration_tests.rs"
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
5. Add User Story 4 → Test independently → Deploy/Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3 + 4
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence</content>
<parameter name="filePath">D:\uni\senior\otağ\specs\002-add-control-flow\tasks.md
