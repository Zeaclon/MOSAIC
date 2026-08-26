# MOSAIC Development Environment

**Status:** Proposed

**Issue:** #5 — Establish local development environment

## 1. Purpose

This document defines the development and test environments required to build, validate, and test MOSAIC consistently.

The purpose of the MOSAIC development environment is to provide a reproducible and documented foundation from which developers can:

* Obtain and work with the MOSAIC source code
* Install the required development tooling and dependencies
* Build MOSAIC
* Format and validate project files
* Execute automated tests
* Test provider implementations
* Validate MOSAIC against its supported base environment

The development environment must not depend on undocumented assumptions about an individual developer's workstation.

This document defines the overall development and testing model. Detailed procedures may be moved into dedicated development documentation as the project grows.

---

## 2. Scope

The MOSAIC development environment must define or provide guidance for:

* Source code acquisition and repository setup
* Development and test environment classes
* Required development tools
* Language runtimes
* Package and dependency management
* Code formatting
* Linting and static analysis
* Building MOSAIC
* Unit testing
* Integration testing
* Provider testing
* Implementation-specific testing
* Supported-environment validation
* Test environment isolation
* Environment reproducibility
* Continuous integration
* Standard development commands
* Recommended IDE and editor setup
* Development workflow

This document is the entry point for MOSAIC development-environment documentation.

---

## 3. Development Environment and User Environment

The MOSAIC development environment is separate from the MOSAIC runtime environment.

Development and testing dependencies must not automatically become dependencies of a normal MOSAIC installation.

### 3.1 User Environment

A MOSAIC user should only require the components necessary for the MOSAIC configuration and implementation they choose to use.

For example, if MOSAIC later adds a Sway provider, a user choosing that provider should not be required to install Hyprland solely because MOSAIC also supports a Hyprland provider.

Users should not be required to install:

* Development language runtimes
* Build tools
* Formatters
* Linters
* Test frameworks
* Unrelated providers
* Unrelated desktop implementations
* Test environments

unless those components are required by their selected MOSAIC installation.

### 3.2 Development Environment

A MOSAIC developer may require additional tooling and environments to build and test MOSAIC.

This may include:

* Build tools
* Language runtimes
* Development dependencies
* Test frameworks
* Multiple provider implementations
* Isolated graphical environments
* Virtual machines
* Other environment-specific testing infrastructure

The development environment may therefore be substantially larger than a normal MOSAIC installation.

This distinction is fundamental to MOSAIC's modular architecture.

---

## 4. Development and Test Environment Classes

MOSAIC development and testing should not be treated as a single monolithic environment.

Different tests have different requirements.

The project should distinguish between environment classes according to the type of validation being performed.

### 4.1 Core Development Environment

The core development environment is the primary environment used by a developer to work on MOSAIC.

It should support, where applicable:

* Repository work
* Dependency installation
* Formatting
* Linting
* Static analysis
* Building
* Unit testing
* Tests that do not require a specific desktop implementation

The core development environment should contain only the tooling required for the components being developed and tested.

It must not require every implementation supported by MOSAIC to be installed.

### 4.2 Integration Test Environment

An integration test environment is used to validate interactions between MOSAIC components.

For example:

```text
Configuration
    ↓
Loader
    ↓
Validator
    ↓
Resolver
    ↓
Provider
    ↓
Generated configuration
```

An integration environment may be identical to the core development environment when the tests do not require additional software.

Where isolation is required, the integration environment may be created separately.

### 4.3 Provider Test Environment

A provider test environment is used to test a specific MOSAIC provider.

A provider test environment may require the target implementation or implementation-specific validation tools.

For example:

```text
Hyprland provider tests
        ↓
Hyprland environment

Sway provider tests
        ↓
Sway environment
```

A developer working on one provider should not necessarily be required to install every implementation supported by MOSAIC.

Provider environments should be established only where required by the provider being developed or tested.

### 4.4 Supported-Environment Validation Environment

A supported-environment validation environment is a clean environment used to verify that MOSAIC works on an environment it claims to support.

This environment must not depend on undocumented packages, services, configuration, environment variables, or files from a developer's personal workstation.

The supported-environment validation environment should be as close as practical to a real supported MOSAIC installation.

---

## 5. Development Environment Principles

The development and testing environments should follow the principles below.

### 5.1 Reproducibility

Another developer must be able to establish an equivalent environment by following documented project requirements.

Development and test environments must not depend on undocumented personal configuration.

### 5.2 Explicit Dependencies

Required tools and dependencies must be declared explicitly.

A dependency must not be considered available merely because it happens to exist on a developer's workstation.

### 5.3 Separation of Concerns

Development dependencies, test dependencies, build dependencies, and runtime dependencies must be distinguished where practical.

A dependency required only for development or testing must not automatically become a user installation dependency.

### 5.4 Automation

Common development operations should be executable through documented and standardized commands.

Developers should not be required to remember complex sequences of unrelated commands.

### 5.5 IDE Independence

MOSAIC must not require a particular IDE or editor.

Project formatting, linting, building, and testing must be executable independently of an IDE.

### 5.6 Environment-Specific Testing

Tests must clearly identify the environment requirements necessary to execute them.

A test should not silently assume the presence of:

* Arch Linux
* Wayland
* A graphical session
* A specific compositor
* A particular provider implementation
* A user-specific configuration

unless that requirement is part of the test.

---

## 6. Supported Development Host

The primary development environment should be based on the MOSAIC supported base environment.

The authoritative supported base environment is defined in [MOSAIC Supported Base Environment](../architecture/supported-base-environment.md).

Development and implementation validation should therefore primarily target the currently supported MOSAIC environment.

The initial supported environment includes:

* Arch Linux x86_64
* Wayland
* Hyprland
* systemd
* systemd-logind
* PipeWire
* D-Bus
* Polkit
* XDG standards and portals

The ability to develop or run limited tests on another operating system does not automatically mean that MOSAIC supports that operating system as a runtime environment.

Additional development hosts may be supported in the future.

---

## 7. Toolchain and Language Runtimes

The MOSAIC development documentation must identify all tools required to build, validate, and test the implemented project.

This includes, where applicable:

* Version control tools
* System package-management tools
* Build tools
* Language runtimes
* Language-specific package managers
* Code-generation tools
* Test frameworks
* Documentation tooling
* Formatting tools
* Linters
* Static-analysis tools

Tools and runtimes must only become required when they are required by an implemented MOSAIC component or documented project process.

The project must not establish hypothetical future technologies as mandatory development requirements.

### 7.1 Language-Specific Tooling

When a language is formally adopted by MOSAIC, its development requirements must define:

* Supported language or runtime version
* Version-management strategy
* Package manager
* Dependency installation method
* Formatter
* Linter
* Static-analysis tools
* Test framework
* Build process
* Standard development commands

These requirements should be documented as part of the relevant component or development documentation.

---

## 8. Dependency Management

MOSAIC must maintain an authoritative and reproducible source of truth for declared dependencies.

Dependencies should be distinguishable according to their purpose.

### 8.1 Runtime Dependencies

Required by an installed MOSAIC component during normal operation.

### 8.2 Build Dependencies

Required to build, compile, package, or otherwise construct MOSAIC.

### 8.3 Test Dependencies

Required only to execute automated tests or test environments.

### 8.4 Development Dependencies

Required for development activities such as formatting, linting, static analysis, or documentation generation.

The exact dependency-management mechanism is not prescribed by this document.

The mechanism may vary according to the language and technology used by an implemented component.

The project must, however, ensure that declared dependencies can be reproduced without relying on undocumented workstation state.

---

## 9. Formatting, Linting, and Static Analysis

MOSAIC must define canonical formatting and validation tooling for each adopted implementation language and relevant project file format.

Formatting and validation must be executable outside of any particular IDE.

The project should provide standardized commands for:

* Formatting project files
* Checking formatting
* Running linters
* Running static analysis
* Running other required validation checks

The exact tooling should be selected when the relevant implementation technology is adopted.

Documentation must also have defined validation and formatting requirements.

---

## 10. Build System and Standard Commands

MOSAIC must provide a consistent method for building and validating the project.

The project should expose a small and predictable command interface for common development operations.

Where applicable, this should include commands equivalent to:

```text
build
clean
check
format
lint
test
test-unit
test-integration
test-provider
test-environment
```

The exact command names and command-runner implementation are not prescribed by this document.

The project may use scripts, a build system, a task runner, a Makefile, or another suitable mechanism.

The requirement is that the project exposes a documented and consistent interface independent of a developer's IDE.

---

## 11. Testing Strategy

MOSAIC requires multiple levels of testing.

A successful build does not establish that MOSAIC works correctly.

Testing should progressively validate MOSAIC from individual components through to actual supported-environment behaviour.

### 11.1 Static Validation

Static validation checks project correctness without executing the complete MOSAIC system.

Examples include:

* Formatting validation
* Linting
* Static analysis
* Configuration schema validation
* Documentation validation
* Dependency validation

Static validation should normally be fast and suitable for routine local and continuous-integration execution.

### 11.2 Unit Testing

Unit tests validate individual components in isolation.

Examples may include:

* Configuration parsing
* Configuration validation
* Configuration merging
* Configuration resolution
* Provider selection
* Provider translation logic
* Utility functions

Unit tests should not require unrelated implementations or a complete graphical desktop environment unless the component being tested explicitly requires them.

Unit tests should normally be fast enough to execute routinely during development and continuous integration.

### 11.3 Integration Testing

Integration tests validate interactions between MOSAIC components.

For example:

```text
Configuration
    ↓
Loader
    ↓
Validator
    ↓
Resolver
    ↓
Provider
    ↓
Generated configuration
```

Integration tests should verify that MOSAIC components interact correctly according to their defined interfaces.

Integration testing may require more dependencies than unit testing but should avoid requiring a full desktop environment where the interaction being tested does not depend on one.

---

## 12. Provider Testing

MOSAIC providers translate abstract MOSAIC configuration into implementation-specific behaviour or configuration.

Provider testing must distinguish between testing the translation process and testing the target implementation itself.

A provider producing different output from another provider does not necessarily indicate incorrect behaviour.

Providers should instead be evaluated according to the MOSAIC capabilities and contracts they claim to support.

Provider testing should be divided into appropriate levels.

### 12.1 Provider Translation Tests

Provider translation tests verify that a provider correctly translates MOSAIC configuration into its expected implementation-specific representation.

For example:

```text
MOSAIC configuration
        ↓
Provider
        ↓
Implementation-specific output
```

These tests should normally be executable without running the complete target desktop implementation.

### 12.2 Provider Validation Tests

Provider validation tests verify that generated output satisfies the structural, syntactic, or other validation requirements that can be checked without performing a complete runtime behaviour test.

The validation mechanism depends on the target implementation.

Examples may include:

* Configuration syntax validation
* Schema validation
* Command validation
* Provider-specific structural checks

A provider validation test must not claim to verify runtime behaviour unless the target implementation is actually exercised.

### 12.3 Provider Runtime Tests

Provider runtime tests validate the generated result against the actual target implementation.

For example:

```text
MOSAIC configuration
        ↓
Hyprland provider
        ↓
Generated Hyprland configuration
        ↓
Hyprland
        ↓
Runtime validation
```

An implementation-specific runtime test should execute against the actual implementation being validated, unless a formally defined substitute provides equivalent validation.

A generic Wayland environment must not automatically be treated as a replacement for the target compositor or implementation.

### 12.4 Provider Behaviour Tests

Where practical, MOSAIC should verify observable behaviour rather than only successful process execution.

For example, successful configuration loading may be necessary but may not prove that the intended behaviour occurred.

The exact behaviour-testing mechanism will depend on the capability and implementation being tested.

---

## 13. Provider Conformance

Provider conformance refers to whether a provider correctly satisfies the MOSAIC provider contract and the capabilities it claims to support.

MOSAIC should maintain a clear relationship between:

* MOSAIC capabilities
* Provider-supported capabilities
* Expected provider behaviour
* Tests that validate those capabilities

A provider must not claim to support a MOSAIC capability unless that capability has appropriate validation.

Where multiple providers implement the same MOSAIC capability, they should be tested against the same abstract behavioural requirements where practical.

Equivalent behaviour does not require identical implementation-specific configuration.

---

## 14. Implementation-Specific Test Environments

Some MOSAIC components cannot be fully tested without their target implementation being available.

For example, a Hyprland provider may require Hyprland to determine whether generated configuration is accepted and behaves correctly.

Implementation-specific test environments may therefore be required.

These environments may contain:

* The target implementation
* Required graphical services
* Required session infrastructure
* Implementation-specific validation tools
* Other dependencies necessary to perform the required tests

These components are development and testing dependencies and must not automatically become dependencies of a normal MOSAIC installation.

MOSAIC should avoid requiring every provider implementation on every developer workstation.

Instead, developers and automated systems should establish the implementation-specific environments required for the work being performed.

---

## 15. Test Environment Isolation

Different tests require different forms of isolation.

The project should select the isolation mechanism according to the test requirements.

Potential mechanisms include:

* Local development environments
* Containers
* Virtual machines
* Dedicated test systems
* Reproducible system images
* Self-hosted continuous-integration runners
* Other suitable isolated environments

No single mechanism is suitable for every MOSAIC test.

### 15.1 Containers

Containers may be appropriate for tests that require:

* Reproducible dependencies
* Isolated build environments
* Static validation
* Unit testing
* Some integration testing

Containers must not automatically be assumed to accurately reproduce a complete graphical Linux desktop or login session.

### 15.2 Virtual Machines

Virtual machines may be appropriate where tests require:

* A clean Arch Linux system
* systemd and system services
* A realistic login environment
* Wayland
* A compositor
* PipeWire
* D-Bus
* Polkit
* XDG portals
* Full MOSAIC installation validation

The exact virtual-machine strategy should be selected when implementation requirements justify it.

---

## 16. Clean-Environment Validation

MOSAIC must periodically validate that it can be built and tested without relying on accidental dependencies from a developer workstation.

Clean-environment validation should identify assumptions such as:

* Undeclared package dependencies
* Missing services
* Missing environment variables
* User-specific configuration
* Incorrect filesystem assumptions
* Incorrect permissions
* Incorrect service dependencies
* Incorrect graphical-session assumptions
* Dependencies available only because they happened to be installed previously

Clean-environment validation should distinguish between at least two objectives.

### 16.1 Clean Build and Test Validation

A clean environment should be able to:

```text
Obtain MOSAIC source
        ↓
Install declared development dependencies
        ↓
Build MOSAIC
        ↓
Run static validation
        ↓
Run unit tests
        ↓
Run applicable integration tests
```

A container or other lightweight isolated environment may be suitable where the tests do not require a complete desktop system.

### 16.2 Clean Supported-System Validation

A clean supported environment should be able to:

```text
Start with a clean supported system
        ↓
Install the supported base environment
        ↓
Install MOSAIC
        ↓
Apply MOSAIC configuration
        ↓
Start the target implementation
        ↓
Validate required behaviour
```

A virtual machine or equivalent full-system environment may be appropriate for this class of testing.

---

## 17. Test Environment Matrix

MOSAIC should maintain a test environment matrix as supported implementations are added.

The matrix should identify the requirements of each test category.

For example:

| Test Category               | Generic Environment | Arch      | Wayland     | Target Implementation | Clean System |
| --------------------------- | ------------------- | --------- | ----------- | --------------------- | ------------ |
| Static validation           | Yes                 | No        | No          | No                    | No           |
| Unit tests                  | Usually             | No        | No          | No                    | No           |
| Integration tests           | Usually             | Sometimes | Sometimes   | No                    | No           |
| Provider translation tests  | Usually             | Sometimes | No          | No                    | No           |
| Provider validation tests   | Usually             | Sometimes | Sometimes   | Sometimes             | No           |
| Provider runtime tests      | No                  | Yes       | Yes         | Yes                   | No           |
| Supported-system validation | No                  | Yes       | As required | As required           | Yes          |

The matrix is illustrative and will evolve as MOSAIC gains additional components and implementations.

A test must document any environment requirements that differ from the normal development environment.

---

## 18. Continuous Integration

The MOSAIC development process should eventually reproduce appropriate automated validation through continuous integration.

Continuous integration should execute all tests that can be reliably automated in the available environment.

The initial automated validation pipeline should include, where applicable:

```text
Source checkout
        ↓
Dependency installation
        ↓
Formatting validation
        ↓
Linting
        ↓
Static analysis
        ↓
Build
        ↓
Unit tests
        ↓
Integration tests
        ↓
Provider translation tests
        ↓
Applicable provider validation tests
```

Implementation-specific runtime testing and full supported-system testing may require a separate strategy.

Possible approaches include:

* Specialized CI environments
* Self-hosted runners
* Virtual-machine testing
* Dedicated test systems
* Scheduled validation
* Release-candidate validation

CI must not be treated as proof that all supported runtime behaviour has been tested unless the relevant supported environment is actually reproduced.

---

## 19. IDE and Editor Recommendations

MOSAIC must remain IDE-agnostic.

Developers may use any suitable IDE or editor that supports the project's languages and tooling.

JetBrains IDEs are suitable for developers who prefer the JetBrains ecosystem.

Other supported development workflows may use:

* Visual Studio Code
* VSCodium
* Neovim
* Vim
* Emacs
* Other suitable editors

IDE configuration should provide convenience rather than define project behaviour.

Where possible, IDE integrations should invoke the same project formatter, linter, build, and test tooling available from the command line.

Project-wide editor conventions should be represented through portable configuration where appropriate, such as `.editorconfig`.

---

## 20. Development Workflow

The standard MOSAIC development workflow should follow a predictable process.

```text
Clone repository
        ↓
Establish required development environment
        ↓
Install declared dependencies
        ↓
Create feature branch
        ↓
Implement change
        ↓
Format
        ↓
Lint and run static analysis
        ↓
Build
        ↓
Run unit tests
        ↓
Run integration tests
        ↓
Run provider tests where applicable
        ↓
Run implementation-specific tests where applicable
        ↓
Validate against supported environment where applicable
        ↓
Commit changes
        ↓
Open pull request
        ↓
Continuous integration validation
```

Not every change requires every test category.

The required validation should be determined by the components affected by the change.

Changes to provider implementations should require provider-specific validation.

Changes affecting supported runtime behaviour may require supported-environment validation.

---

## 21. Future Documentation Structure

This document serves as the entry point for MOSAIC development-environment documentation.

As MOSAIC grows, detailed sections may be moved into dedicated documents.

A possible future structure is:

```text
docs/
├── development/
│   ├── development-environment.md
│   ├── building.md
│   ├── dependency-management.md
│   ├── testing.md
│   ├── unit-testing.md
│   ├── integration-testing.md
│   ├── provider-testing.md
│   ├── environment-testing.md
│   └── contributing.md
│
└── architecture/
    ├── mosaic-architecture.md
    ├── configuration-architecture.md
    └── ...
```

The development environment document should remain the primary entry point and link to detailed development documentation rather than duplicating it.

---

## 22. Relationship to User Installation Documentation

Development documentation must remain separate from user installation documentation.

The user installation documentation should explain how to install and configure MOSAIC for normal use.

The development documentation should explain how to:

* Establish a development environment
* Obtain the source code
* Build MOSAIC
* Run tests
* Test providers
* Validate implementations
* Contribute changes

The MOSAIC README should provide clear links to both user and developer documentation.

This separation prevents development and testing dependencies from being confused with normal MOSAIC installation requirements.

---

## 23. Open Decisions

The following implementation decisions remain open until MOSAIC adopts the relevant technologies:

* Primary implementation language or languages
* Required language runtime versions
* Version-management strategy
* Dependency-management mechanisms
* Build system
* Task runner or standardized command mechanism
* Formatters
* Linters and static-analysis tools
* Unit-test frameworks
* Integration-test frameworks
* Provider test framework
* Provider capability and conformance representation
* Implementation-specific validation mechanisms
* Container strategy
* Virtual-machine or system-image strategy
* CI platform and workflow
* Automated supported-environment testing
* Documentation tooling

These decisions should be made when they become necessary and documented in the appropriate development or architecture documentation.

The project should avoid selecting technologies solely because they appear in this document as possible examples.

---

## 24. Requirements for Completion

Issue #5 should not be considered complete merely because a list of development packages exists.

The development environment must establish:

* The distinction between user and development environments
* The classes of development and test environments required by MOSAIC
* How developers establish the required environment for their work
* How required tools and language runtimes are identified
* How dependencies are declared and reproduced
* How code and documentation are formatted and validated
* How MOSAIC is built
* How unit tests are executed
* How integration tests are executed
* How provider translation is tested
* How provider output is validated
* How actual provider runtime behaviour is tested
* How provider conformance is defined
* How implementation-specific environments are used
* How clean build and test environments are validated
* How clean supported-system environments are validated
* How containers and virtual machines may serve different testing purposes
* How CI fits into the overall validation strategy
* How developers may use different IDEs while following the same project workflow
* Where detailed development and testing documentation will live as the project grows

The objective is to establish a development and testing system in which a developer can confidently state:

> **"I established the documented environment required for my work, built MOSAIC successfully, executed the required validation, tested the affected implementation where necessary, and verified the change in the environment MOSAIC claims to support."**
