<!--
Sync Impact Report:
- Version change: 1.0.0 → 1.1.0
- Modified sections: Technical Standards (added compiler tech), Development Workflow (added testing encouragement)
- Templates requiring updates: none
- Follow-up TODOs: none
-->
# Otağ Programming Language Constitution

## Core Principles

### I. Turkish Localization Priority
The language must prioritize Turkish keywords, syntax, and documentation. All core features must be accessible and intuitive for Turkish speakers. Localization is not an afterthought but a core design principle.

### II. Simplicity and Readability
Syntax should be simple, readable, and expressive. Avoid unnecessary complexity; prefer clear, concise constructs that are easy to understand and maintain.

### III. Performance and Efficiency (NON-NEGOTIABLE)
The language must compile efficiently and produce performant executables. Resource usage (memory, CPU) should be optimized by default.

### IV. Safety and Reliability
Provide strong type safety, memory safety where possible, and predictable behavior. Errors should be caught at compile time when feasible.

### V. Extensibility and Modularity
Support modular design, libraries, and extensions. Allow users to build upon the language without modifying core.

## Technical Standards

- Full UTF-8 support with proper Turkish character handling (i, İ, ğ, etc.).
- Standard library must include Turkish-specific utilities.
- Cross-platform compatibility (Windows, Linux, macOS).
- Compiler implemented in Rust using LLVM backend.

## Development Workflow

- Open source development with community contributions.
- Code reviews required for all changes.
- Automated testing and CI/CD pipelines.
- Encourage usage of modern test frameworks and existing tools for comprehensive testing and quality assurance.

## Governance

Constitution supersedes all other practices. Amendments require community consensus and documentation. All changes must verify compliance with principles.

**Version**: 1.1.0 | **Ratified**: 2025-10-17 | **Last Amended**: 2025-10-17
