+++
title = "MOSAIC Configuration Architecture"
weight = 2
+++

**Status:** Active
**Issue:** #2 — Define MOSAIC configuration architecture

## 1. Purpose

This document defines how MOSAIC represents, resolves, translates, generates, and eventually applies configuration across its components.

The configuration architecture exists to support the principles established by the main MOSAIC architecture:

* MOSAIC configuration describes **user intent**, rather than generated application files.
* Components remain independently configurable and replaceable.
* Providers translate MOSAIC configuration into implementation-specific configuration.
* Users retain ownership and direct access to their configuration.
* Generated configuration is an implementation artifact, not automatically the source of truth.
* Configuration should be reproducible and portable where the target environment supports the requested components.

The architecture is designed to support a complete configuration lifecycle while allowing individual parts of that lifecycle to be implemented incrementally.

The current implementation validates the core boundary between a MOSAIC configuration model and provider-generated configuration using the Hyprland monitor configuration as the first vertical slice.

---

## 2. Configuration Model

MOSAIC uses a declarative configuration model.

The user declares the desired state of their desktop through MOSAIC configuration. MOSAIC then resolves that configuration and passes the resulting component configuration to the appropriate providers.

The intended complete flow is:

```text
User-owned configuration
        ↓
Configuration loading
        ↓
Parsing / decoding
        ↓
Schema validation
        ↓
Defaults + inheritance + overrides
        ↓
Resolved MOSAIC configuration
        ↓
Component configuration
        ↓
Provider translation
        ↓
Generated / applied implementation configuration
        ↓
External software
```

The important distinction is between the **declared configuration** and the **resolved configuration**.

### Declared configuration

The configuration explicitly supplied by the user, profile, theme, layout, or other configuration source.

### Resolved configuration

The effective configuration after MOSAIC has applied defaults, inheritance, references, and precedence rules.

Providers consume the resolved configuration. They should not need to independently reconstruct MOSAIC's precedence or default-resolution rules.

---

## 3. Current Configuration Boundary

The first implemented configuration path establishes the following boundary:

```text
Configuration
      ↓
Monitor Model
      ↓
Hyprland Provider
      ↓
Hyprland Monitor Renderer
      ↓
Generated Hyprland Configuration
```

The current implementation does not yet perform configuration loading, persistence, filesystem writing, or runtime application.

This vertical slice exists to validate the architectural separation between:

1. MOSAIC configuration semantics
2. Component configuration models
3. Provider-specific translation
4. Generated implementation configuration

The generated Hyprland configuration has been validated against a real Hyprland environment and is directly usable by Hyprland.

This establishes the provider translation boundary as an implemented architectural concept rather than a purely theoretical one.

---

## 4. Configuration Scope

MOSAIC configuration is divided into several conceptual scopes.

```text
Global MOSAIC configuration
        │
        ├── Profile
        │     ├── Components
        │     └── Component configuration
        │
        ├── Theme
        │
        ├── Layout
        │
        └── User overrides
```

These scopes represent different responsibilities rather than necessarily requiring different files or directories.

### 4.1 Global configuration

Global configuration contains settings that apply to the MOSAIC environment as a whole.

Examples may include:

* Active profile
* Active theme
* Active layout
* Configuration behavior
* Global preferences

### 4.2 Profile configuration

A profile describes the collection of components that make up a desktop configuration.

A profile may select:

* A compositor/window manager
* A status bar
* A launcher
* Notifications
* Lock screen
* Idle daemon
* Wallpaper system
* Other supported components

A profile should reference component identities and configuration rather than embedding provider-specific implementation details.

### 4.3 Component configuration

Component configuration describes the desired behavior of an individual MOSAIC component.

For example, a monitor configuration may describe its output, position, mode, scale, rotation, and other desired properties without requiring the configuration itself to be expressed using Hyprland syntax.

### 4.4 Theme configuration

Themes provide visual configuration that may affect multiple components.

A theme can supply values such as:

* Colors
* Fonts
* Icons
* Borders
* Transparency
* Wallpaper
* Component styling

Themes should not replace component configuration. They contribute values that are resolved alongside it.

### 4.5 Layout configuration

Layouts describe the conceptual arrangement of interface elements.

A layout may affect multiple components while remaining independent of their provider implementations.

---

## 5. Configuration Storage

MOSAIC configuration should be stored as human-readable, declarative data.

The initial implementation should prefer a format that is:

* Human-readable
* Easy to edit manually
* Easy to parse reliably
* Capable of representing nested configuration
* Suitable for schema validation
* Stable enough for version-controlled configuration

The architecture does not permanently mandate one serialization format. The configuration model is the architectural contract; the storage format is an implementation detail.

For the initial implementation, the structured text format TOML will be used, provided the format provides deterministic parsing and a well-defined specification. MOSAIC will define its own schema independently of the serialization format.

Generated provider configuration does not need to use the same format as MOSAIC configuration.

A possible future structure is:

```text
~/.config/mosaic/
├── config.toml
├── profiles/
│   └── desktop.toml
├── themes/
│   └── default.toml
├── layouts/
│   └── default.toml
└── overrides/
```

The exact directory structure may evolve as implementation begins, but the ownership distinction must remain.

---

## 6. Configuration Ownership

Every configuration value must have a clear owner.

MOSAIC recognizes three important ownership categories.

### 6.1 User-owned configuration

This is the user's declared MOSAIC configuration and is the primary source of intent.

MOSAIC must not silently overwrite user-owned configuration when applying or regenerating configuration.

### 6.2 Resolved configuration

Resolved configuration is an in-memory representation produced by MOSAIC after applying defaults, precedence, inheritance, and other resolution rules.

Resolved configuration is not itself a persistent user-owned source.

It represents the effective state that providers consume.

### 6.3 Provider-generated configuration

Provider-generated configuration is produced from the resolved MOSAIC model for consumption by external software.

Examples include:

* Waybar configuration
* Hyprland configuration fragments
* SwayNC configuration
* Provider-specific scripts

Generated configuration is an implementation artifact.

The provider may regenerate it whenever the resolved configuration changes.

---

## 7. Source of Truth

The user's MOSAIC configuration is the source of truth for configuration **intent**.

The generated configuration consumed by external software is not automatically authoritative.

```text
                SOURCE OF TRUTH
                      │
                      ▼
             MOSAIC Configuration
                      │
                      ▼
              Resolved Configuration
                      │
                      ▼
             Provider-generated files
                      │
                      ▼
              External application
```

This prevents generated files from becoming a second competing configuration system.

If an advanced user intentionally modifies a generated file, that modification is outside the normal MOSAIC configuration lifecycle unless the provider explicitly supports a managed/manual mode.

MOSAIC should never silently import arbitrary changes from generated files back into the MOSAIC model.

---

## 8. Configuration Loading

MOSAIC configuration is loaded before validation or application.

The loader is responsible for locating configured sources, reading them, parsing them, and producing an in-memory configuration model.

The loader should not apply configuration or invoke providers.

Conceptually:

```text
Storage
  ↓
Loader
  ↓
Raw configuration model
```

Loading errors should identify the source and location of the error where possible.

Examples include:

* Missing configuration file
* Invalid syntax
* Unsupported serialization format
* Unreadable file
* Duplicate or conflicting declarations

A missing optional source may fall back to its defined default. A missing required source must result in a clear configuration error.

**Implementation status:** Future.

---

## 9. Validation

Validation occurs before configuration is applied.

MOSAIC should distinguish between **schema validation** and **semantic validation**.

### 9.1 Schema validation

Schema validation checks whether configuration has the expected structure and data types.

Examples:

* Required fields exist
* Values have the correct type
* Enumerated values are valid
* Object structure is valid
* Unknown fields follow the project's compatibility policy

### 9.2 Semantic validation

Semantic validation checks whether the requested configuration makes sense in the current MOSAIC environment.

Examples:

* A referenced component exists
* A selected provider supports the requested feature
* A layout references an available component
* A profile does not contain incompatible components
* A required dependency is available

Validation should happen before providers are asked to generate or apply configuration.

```text
Load
  ↓
Parse
  ↓
Schema validation
  ↓
Semantic validation
  ↓
Resolution
  ↓
Provider generation
  ↓
Apply
```

Invalid configuration must fail safely rather than partially applying an invalid state where practical.

**Implementation status:** Future.

---

## 10. Defaults

MOSAIC supports defaults so that users do not need to specify every possible configuration value.

Defaults must be defined by the layer that owns the behavior.

For example:

* MOSAIC core defaults belong to the core.
* Component defaults belong to the component model.
* Provider-specific defaults belong to the provider.

MOSAIC semantic defaults and backend defaults are not necessarily the same thing.

A MOSAIC component model defines the semantic state that an omitted value represents. The provider is then responsible for translating that semantic state into backend configuration.

For example, the current monitor model defines defaults such as:

```text
disabled = false
mode     = Preferred
scale    = Auto
position = Auto
rotation = 0°
flip     = false
mirror   = none
```

These defaults belong to the MOSAIC monitor model.

The Hyprland provider then determines how those states should be represented in generated Hyprland configuration.

A provider may omit a value when omission produces the same effective semantic state. Otherwise, the provider must explicitly render the required value.

Defaults are lower precedence than explicit user configuration.

---

## 11. Provider Translation

Providers translate resolved MOSAIC configuration into implementation-specific configuration.

The provider owns knowledge of the target application's syntax, representation, and implementation-specific behavior.

The MOSAIC configuration model should not contain backend syntax merely to support a particular provider.

Conceptually:

```text
MOSAIC configuration
        ↓
Resolved configuration
        ↓
Hyprland provider
        ↓
Hyprland configuration
```

The provider is responsible for answering:

> How does this requested MOSAIC state need to be expressed to the target implementation?

The provider should not be responsible for determining what the user meant.

### 11.1 Default omission

Providers should omit unnecessary properties from generated configuration when doing so preserves the requested semantic state.

For the current Hyprland monitor provider:

```text
MOSAIC default        Generated output
──────────────────    ─────────────────
disabled = false      omitted
mode = Preferred      omitted
scale = Auto          omitted
position = Auto       omitted
rotation = 0°         omitted
flip = false          omitted
mirror = none         omitted
```

Non-default values are rendered explicitly.

This keeps generated configuration concise while ensuring that the MOSAIC model remains the source of semantic meaning.

### 11.2 Backend defaults

Providers must not rely on undocumented backend behavior merely to define MOSAIC semantics.

If the backend's default behavior differs from the semantic default defined by MOSAIC, the provider must explicitly render the value required to preserve MOSAIC behavior.

The rule is therefore:

```text
MOSAIC owns semantic defaults.
Provider owns backend translation.
```

---

## 12. Configuration Precedence

MOSAIC may combine configuration from multiple layers, including built-in defaults, component defaults, profiles, themes, user configuration, and explicit user overrides.

When multiple applicable layers define the same value, MOSAIC resolves the value using a deterministic precedence order.

The default precedence is:

**Built-in defaults → component defaults → profiles/themes → user configuration → explicit user overrides**

Higher-precedence values override lower-precedence values where the configuration domain permits overriding.

Configuration domains may restrict which layers are permitted to provide or override particular values.

For example, a theme may provide colors and fonts but should not silently change the user's selected compositor.

The precedence and ownership rules for non-trivial configuration domains must be documented.

**Implementation status:** Architecture established; full resolution system is future implementation.

---

## 13. User Overrides

User overrides provide an explicit mechanism for changing values supplied by profiles, themes, layouts, or defaults without modifying those reusable sources.

This is important for preserving the composability of MOSAIC.

For example:

```text
Profile: workstation
    status bar position = top

Theme: dark
    status bar background = dark

User override:
    status bar position = bottom
```

The resolved configuration becomes:

```text
status bar position   = bottom
status bar background = dark
```

Overrides should be explicit rather than relying on undocumented file ordering or provider behavior.

**Implementation status:** Future.

---

## 14. Merge Semantics

Configuration values must have defined merge behavior.

MOSAIC should not assume that every configuration object can simply be deep-merged.

Values should fall into one of several categories.

### Scalar values

A scalar value has one effective value after precedence resolution.

Example:

```toml
position = "bottom"
```

The highest-precedence valid value wins.

### Objects

Objects may be merged field-by-field where the schema explicitly permits it.

### Lists

Lists require an explicit merge strategy.

Possible strategies include:

* Replace
* Append
* Prepend
* Merge by identifier

The default should be **replace** unless a schema explicitly defines another behavior.

This prevents surprising list combinations when profiles and overrides are composed.

### Component sets

Components should normally be identified by stable component IDs rather than list position.

For example:

```toml
[[components]]
id = "status-bar"
provider = "waybar"

[[components]]
id = "launcher"
provider = "rofi"
```

This allows configuration to refer to a component independently of its position in a file.

**Implementation status:** Architecture established; implementation is future.

---

## 15. Profiles, Themes, and Layouts

Profiles, themes, and layouts are separate configuration sources that may contribute to the final configuration.

They should remain composable.

Conceptually:

```text
Profile
   +
Theme
   +
Layout
   +
User Overrides
   ↓
Resolved Configuration
```

A profile should define the desktop's component composition.

A theme should primarily define appearance.

A layout should define presentation and arrangement.

A theme or layout should not implicitly replace unrelated profile decisions.

Where these concepts overlap, the configuration schema must define ownership and merge semantics explicitly.

**Implementation status:** Future.

---

## 16. Component Configuration

Each component has a MOSAIC-facing configuration model.

A component configuration should contain information meaningful at the MOSAIC abstraction level, plus explicitly defined extension points where provider-specific options are necessary.

The current monitor model is an example of this boundary.

It describes concepts such as:

* Output
* Enabled/disabled state
* Display mode
* Scale
* Position
* Rotation
* Flip
* Mirroring

The monitor model does not contain Hyprland configuration syntax.

Instead:

```text
MOSAIC Monitor Configuration
            ↓
      Hyprland Provider
            ↓
      Hyprland Monitor Configuration
```

The MOSAIC core should not contain Hyprland's configuration syntax merely to support the Hyprland provider.

---

## 17. Provider-Specific Configuration

Some external applications expose functionality that cannot reasonably be represented by the common MOSAIC model.

Providers may therefore expose an explicitly scoped provider-specific configuration area.

For example:

```toml
[component]
id = "status-bar"
provider = "waybar"

[component.options]
layer = "top"
ipc = true
```

Provider-specific options belong to the provider namespace and must not leak into the generic MOSAIC component model.

This preserves provider independence while still allowing advanced users to access implementation-specific capabilities.

A provider-specific option must never be required for a core MOSAIC concept unless that concept itself is explicitly provider-dependent.

**Implementation status:** Future.

---

## 18. Rendering Versus Writing

Provider rendering and filesystem writing are separate responsibilities.

A provider renderer converts a resolved configuration into a deterministic representation:

```text
Resolved Configuration
        ↓
Provider Renderer
        ↓
String / Generated Representation
```

A separate writer is responsible for writing that representation to the filesystem.

This separation allows provider translation to be tested without requiring filesystem access or a running desktop environment.

The renderer should:

* Produce deterministic output
* Avoid filesystem side effects
* Be testable independently
* Represent the complete requested provider configuration

The writer will eventually be responsible for:

* Selecting the output path
* Checking file ownership
* Writing generated configuration
* Handling backups where required
* Committing changes safely

**Implementation status:** Provider rendering is implemented for the initial Hyprland monitor slice. Filesystem writing is future.

---

## 19. Generated Configuration

Providers may generate configuration files, scripts, symlinks, or other implementation artifacts.

Generated output should be derived from the resolved MOSAIC configuration.

```text
Resolved MOSAIC configuration
            ↓
      Provider renderer
            ↓
      Generated output
```

Generated output should:

* Be reproducible from the same resolved configuration
* Be safe to regenerate
* Be clearly identifiable as generated where practical
* Avoid overwriting unrelated user files
* Remain isolated from the user's source configuration

Where possible, generated files should include a marker indicating that they are MOSAIC-managed.

The current Hyprland renderer produces:

```text
-- This file is managed by MOSAIC.
-- Do not edit this file manually.
--
-- MOSAIC version: <version>
-- Generated automatically.
--
```

The MOSAIC application version is included as provenance information.

The exact marker and format are provider-specific.

---

## 20. User-Editable vs Generated Files

MOSAIC must make the ownership of files understandable.

The preferred model is:

```text
~/.config/mosaic/              User-owned
        │
        ▼
MOSAIC resolved state
        │
        ▼
~/.config/<provider>/          Provider-generated
```

MOSAIC should avoid generating directly into a user's primary hand-maintained configuration file when a separate generated file or include mechanism is available.

For example, a provider may eventually generate:

```text
~/.config/hypr/mosaic.conf
```

while the user retains:

```text
~/.config/hypr/hyprland.conf
```

The user's configuration can then include the generated fragment where appropriate.

This approach reduces destructive file ownership conflicts and makes manual customization practical.

The exact integration mechanism is provider-specific.

---

## 21. Application

Applying configuration means translating the resolved configuration into the external state required to realize it.

Application may involve:

* Generating configuration files
* Updating symlinks
* Creating scripts
* Reloading a provider
* Restarting a component where required
* Applying runtime settings

The provider owns the implementation-specific application process.

MOSAIC Core coordinates the operation but should not contain provider-specific commands.

Conceptually:

```text
Resolved Configuration
        ↓
Component
        ↓
Provider
        ↓
Generate
        ↓
Validate provider output
        ↓
Apply
        ↓
Reload / restart if required
```

Where an application operation can fail, MOSAIC should report the failure and avoid claiming that the configuration was successfully applied.

**Implementation status:** Future.

---

## 22. Transaction and Failure Model

Configuration application should be as atomic as practical.

MOSAIC should prefer:

```text
Resolve
  ↓
Validate everything possible
  ↓
Generate temporary output
  ↓
Validate generated output
  ↓
Commit generated changes
  ↓
Reload / apply
```

rather than modifying live configuration incrementally and discovering errors halfway through the operation.

If a provider cannot support atomic application, the limitation should be documented by that provider.

MOSAIC should preserve the last known valid generated configuration where practical so that a failed update does not unnecessarily leave the desktop unusable.

**Implementation status:** Future.

---

## 23. Backup and Recovery

MOSAIC configuration should be recoverable without requiring generated provider configuration to be treated as the source of truth.

The primary configuration that must be protected is the user-owned MOSAIC configuration.

MOSAIC should support safe recovery from configuration changes that result in an invalid or undesirable state.

At minimum, the architecture should support:

* Preserving the previous known-valid configuration before applying a change
* Restoring the previous configuration after a failed application where practical
* Keeping generated configuration reproducible from the restored MOSAIC configuration
* Detecting invalid configuration before it is applied where possible
* Providing clear diagnostics when automatic recovery cannot be performed

The exact backup mechanism, retention policy, and storage location are implementation details.

**Implementation status:** Future.

---

## 24. Runtime State vs Configuration

MOSAIC must distinguish configuration from runtime state.

Configuration describes the desired state.

Runtime state describes what is currently running.

Examples of runtime state include:

* Active workspace
* Current window
* Process IDs
* Current display state
* Temporary provider state

Runtime state should not normally be written back into the user's configuration unless explicitly requested or required for a supported feature.

This prevents transient desktop state from polluting reproducible configuration.

---

## 25. Configuration Versioning

MOSAIC configuration should carry a schema version.

For example:

```toml
version = 1
```

The version identifies the configuration schema, not the MOSAIC application version.

This allows MOSAIC to detect older configurations and provide migrations where necessary.

Migrations should transform configuration into a newer schema before normal validation and resolution.

```text
Stored configuration
        ↓
Version detection
        ↓
Migration
        ↓
Current schema
        ↓
Validation
```

Migrations should be deterministic and should avoid silently changing user intent.

**Implementation status:** Future.

---

## 26. Reproducibility

A configuration should contain the information required to reproduce the declared MOSAIC desktop configuration on a compatible system.

Reproducibility does not require generated files to be committed or preserved as the primary configuration.

Instead:

```text
MOSAIC configuration
       +
Compatible components/providers
       +
Target environment
       ↓
Equivalent resolved configuration
       ↓
Equivalent generated configuration
```

Where exact reproduction is impossible because of environment differences, MOSAIC should report those differences rather than silently producing an unrelated configuration.

The current Hyprland renderer demonstrates deterministic generation: the same `Configuration` produces the same generated configuration text.

---

## 27. Portability

Configuration should be portable between systems where the referenced components and providers are available.

A configuration that references an unavailable provider should fail with a useful diagnostic rather than silently substituting a different implementation.

For example:

```text
Profile requests:
    status-bar provider = waybar

Target system:
    Waybar unavailable
```

MOSAIC should report that the requested provider is unavailable.

Automatic provider substitution may be considered in the future, but it must be explicit because different providers may not provide equivalent behavior.

---

## 28. Configuration API Boundary

The configuration system should expose a common internal service to MOSAIC interfaces such as the CLI and future GUI.

Conceptually:

```text
              Configuration Service
                 /            \
               CLI            GUI
                 \            /
                  MOSAIC Core
```

The CLI and GUI should not implement their own configuration resolution or precedence logic.

Both should operate through the same configuration model and services.

This ensures that changing configuration through the GUI produces the same result as changing it through the CLI or configuration files.

**Implementation status:** Future.

---

## 29. Direct User Editing

Direct editing of MOSAIC configuration is a first-class supported workflow.

MOSAIC should not require a GUI or CLI to modify configuration.

The intended model is:

```text
                 MOSAIC Configuration
                  /              \
             Manual editing     GUI / CLI
                  \              /
                   Configuration Service
```

Tools should modify the same underlying configuration model rather than creating a second proprietary representation.

This supports experienced Linux users while allowing less technical users to manage the same system through future graphical interfaces.

---

## 30. Configuration Ownership Rules

The following rules are normative for the architecture.

1. **User configuration is the source of truth for user intent.**
2. **Resolved configuration represents effective MOSAIC state and is not itself a persistent source of truth.**
3. **Generated provider configuration is an implementation artifact.**
4. **Providers must not modify unrelated component configuration.**
5. **MOSAIC must not silently overwrite user-owned configuration.**
6. **Generated files should be isolated from hand-maintained files where practical.**
7. **Defaults must have lower precedence than explicit user configuration.**
8. **MOSAIC semantic defaults belong to the model that owns the behavior.**
9. **Providers own translation of MOSAIC semantics into backend-specific representation.**
10. **Precedence must be deterministic.**
11. **Merge behavior must be defined by the configuration schema.**
12. **Configuration must be validated before application.**
13. **Provider-specific configuration must remain inside a provider boundary.**
14. **Runtime state must not become configuration implicitly.**
15. **CLI and GUI interfaces must use the same configuration services and resolution rules.**
16. **Provider renderers should be deterministic and free of filesystem side effects.**
17. **Generated configuration must not silently become the source of truth.**

---

## 31. Complete Configuration Lifecycle

The intended complete lifecycle is:

```text
┌──────────────────────┐
│ User-owned sources   │
│ Profile / Theme /    │
│ Layout / Overrides   │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ Load & Parse         │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ Schema Validation    │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ Semantic Validation  │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ Resolve Defaults,    │
│ Precedence & Merges  │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ Resolved MOSAIC      │
│ Configuration        │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ Component Providers  │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ Generate & Validate  │
│ Provider Output      │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ Apply / Reload       │
└──────────────────────┘
```

Each stage has a distinct responsibility.

No stage should bypass validation or independently reinterpret configuration precedence.

The currently implemented portion is:

```text
Configuration
      ↓
Monitor Model
      ↓
Hyprland Provider
      ↓
Hyprland Renderer
      ↓
Generated Configuration
```

The remaining lifecycle stages will be implemented incrementally.

---

## 32. Architectural Boundaries

The configuration architecture reinforces the boundaries defined by the main MOSAIC architecture.

### Configuration ↔ Core

Configuration provides the declarative representation of user intent. The core provides the models and services used to validate and resolve it.

### Core ↔ Component

The core coordinates components through defined interfaces.

### Component ↔ Provider

A component defines MOSAIC-level behavior. A provider implements that behavior using a specific technology.

### Provider ↔ Renderer

The provider owns implementation-specific translation. The renderer converts that translation into deterministic output without performing filesystem operations.

### Provider ↔ External Software

The provider owns the eventual application of implementation-specific configuration to external software.

### User ↔ Generated Configuration

The user owns MOSAIC configuration. Generated configuration is managed as an implementation artifact and should not displace the user's source configuration.

---

## 33. Initial Implementation Constraints

The first implementation may make pragmatic choices for the initial Arch Linux and Hyprland environment.

These may include:

* A specific serialization format
* A concrete configuration directory
* Provider-specific generated files
* A limited set of supported merge operations
* A limited component schema
* A limited set of provider capabilities

These are implementation decisions, not reasons to weaken the architectural boundaries defined here.

The initial implementation should validate the model against real components such as Hyprland and Waybar while keeping their provider-specific behavior outside the MOSAIC core.

The current Hyprland monitor implementation demonstrates this approach: the MOSAIC monitor model represents user intent while the Hyprland provider translates that model into Hyprland-specific configuration.

---

## 34. Summary

MOSAIC configuration follows one central rule:

> **The user declares what their desktop should be; MOSAIC resolves that intent; providers determine how the underlying software implements it.**

The architecture therefore separates:

```text
User-owned configuration
        ↓
Load
        ↓
Validate
        ↓
Resolve
        ↓
Resolved MOSAIC configuration
        ↓
Component
        ↓
Provider
        ↓
Renderer
        ↓
Generated configuration
        ↓
External software
```

The first implementation has validated this separation through the Hyprland monitor configuration.

The configuration architecture should evolve alongside implementation, but changes to ownership, precedence, defaults, abstraction boundaries, or provider responsibilities should be deliberate and documented.

The architecture describes the intended complete system; implementation should proceed incrementally through independently testable vertical slices.
