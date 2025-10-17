# Tasks: 005-add-error-handling

**Input**: Design documents from `/specs/005-add-error-handling/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL - not included as not requested in specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths assume single project - adjust based on plan.md structure

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create error handling module structure in src/
- [ ] T002 Initialize error types and reporting infrastructure

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T003 [P] Define custom error types in src/types.rs
- [ ] T004 [P] Implement error reporting utilities in src/error_reporting.rs
- [ ] T005 [P] Add location tracking for source positions in src/location.rs
- [ ] T006 Integrate error handling with existing parser and lexer

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Report Syntax Errors (Priority: P1) 🎯 MVP

**Goal**: Report lexical and parse errors with accurate location information

**Independent Test**: Compile a file with syntax errors and verify error messages show correct line/column

### Implementation for User Story 1

- [ ] T007 [P] [US1] Enhance lexer to report lexical errors in src/lexer.rs
- [ ] T008 [P] [US1] Enhance parser to report parse errors in src/parser.rs
- [ ] T009 [US1] Integrate syntax error reporting with main compiler in src/main.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Report Semantic Errors (Priority: P2)

**Goal**: Report type errors, undefined variables, and other semantic issues

**Independent Test**: Compile code with semantic errors and verify appropriate error messages

### Implementation for User Story 2

- [ ] T010 [P] [US2] Add semantic analysis phase in src/semantic.rs
- [ ] T011 [P] [US2] Implement type checking in src/type_checker.rs
- [ ] T012 [US2] Integrate semantic error reporting with compiler pipeline

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Improve Error Messages (Priority: P3)

**Goal**: Provide clear, actionable error messages with suggestions

**Independent Test**: Verify error messages are user-friendly and helpful

### Implementation for User Story 3

- [ ] T013 [P] [US3] Add Turkish localization for error messages in src/error_messages.rs
- [ ] T014 [P] [US3] Implement error message suggestions in src/suggestions.rs
- [ ] T015 [US3] Enhance error formatting and display

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T016 [P] Add comprehensive error tests in tests/
- [ ] T017 Update grammar.pest for error recovery
- [ ] T018 Add stack trace support for runtime errors
- [ ] T019 Run validation and ensure all error handling works

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
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all implementation for User Story 1 together:
Task: "Enhance lexer to report lexical errors in src/lexer.rs"
Task: "Enhance parser to report parse errors in src/parser.rs"
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
