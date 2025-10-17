<!--
Sync Impact Report:
- Version change: 1.2.0 → 1.2.1
- Modified sections: Governance (added spec tracking requirement)
- Templates requiring updates: none
- Follow-up TODOs: none
-->
# Otağ Programming Language Constitution

## Core Principles

### I. Turkish Localization Priority
The language must prioritize Turkish keywords, syntax, and documentation. All core features must be accessible and intuitive for Turkish speakers. Localization is not an afterthought but a core design principle.

### II. Simplicity and Readability
Syntax should be simple, readable, and expressive. Avoid unnecessary complexity; prefer clear, concise constructs that are easy to understand and maintain.

### III. Feature Implementation Priority and Beginner-Friendliness
Prioritize rapid feature development and implementation over premature optimization. The language should be close to natural language, especially Turkish, to be beginner-friendly. For example, "x'i tamsayı olarak tanımla" declares x as an integer type.

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

Constitution supersedes all other practices. Amendments require community consensus and documentation. All changes must verify compliance with principles. Agents are responsible for committing their generated work and changes. The language specification resides in spec.md and must be maintained alongside constitution.md. Any additions of new features or fixes to syntax logic errors must update spec.md accordingly.

**Version**: 1.2.1 | **Ratified**: 2025-10-17 | **Last Amended**: 2025-10-17
