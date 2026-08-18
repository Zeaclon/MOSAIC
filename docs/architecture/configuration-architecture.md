# MOSAIC Configuration Architecture

**Status:** Proposed  
**Issue:** #2 — Define MOSAIC configuration architecture

## 1. Purpose

This document defines how MOSAIC stores, reads, validates, resolves, and applies configuration across its components.

The configuration architecture exists to support the principles established by the main MOSAIC architecture:

- MOSAIC configuration describes **user intent**, rather than generated application files.
- Components remain independently configurable and replaceable.
- Providers translate MOSAIC configuration into implementation-specific configuration.
- Users retain ownership and direct access to their configuration.
- Generated configuration is an implementation artifact, not automatically the source of truth.
- Configuration should be reproducible and portable where the target environment supports the requested components.

This document defines the configuration model and lifecycle without prescribing implementation details that belong to individual providers.

---

## 2. Configuration Model

MOSAIC uses a declarative configuration model.

The user declares the desired state of their desktop through MOSAIC configuration. MOSAIC then resolves that configuration, validates it, and passes the resulting component configuration to the appropriate providers.

The conceptual flow is:

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

Providers consume the resolved configuration. They should not need to independently reconstruct MOSAIC's precedence rules.

---

## 3. Configuration Scope

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

### 3.1 Global configuration

Global configuration contains settings that apply to the MOSAIC environment as a whole.

Examples may include:

- Active profile
- Active theme
- Active layout
- Configuration behavior
- Global preferences

### 3.2 Profile configuration

A profile describes the collection of components that make up a desktop configuration.

A profile may select:

- A compositor/window manager
- A status bar
- A launcher
- Notifications
- Lock screen
- Idle daemon
- Wallpaper system
- Other supported components

A profile should reference component identities and configuration rather than embedding provider-specific implementation details.

### 3.3 Component configuration

Component configuration describes the desired behavior of an individual MOSAIC component.

For example, a status bar configuration may describe its position, visibility, modules, and desired appearance without requiring the configuration to be expressed as a Waybar-specific JSON file.

### 3.4 Theme configuration

Themes provide visual configuration that may affect multiple components.

A theme can supply values such as:

- Colors
- Fonts
- Icons
- Borders
- Transparency
- Wallpaper
- Component styling

Themes should not replace component configuration. They contribute values that are resolved alongside it.

### 3.5 Layout configuration

Layouts describe the conceptual arrangement of interface elements.

A layout may affect multiple components while remaining independent of their provider implementations.

---

## 4. Configuration Storage

MOSAIC configuration should be stored as human-readable, declarative data.

The initial implementation should prefer a format that is:

- Human-readable
- Easy to edit manually
- Easy to parse reliably
- Capable of representing nested configuration
- Suitable for schema validation
- Stable enough for version-controlled configuration

The architecture does not permanently mandate one serialization format. The configuration model is the architectural contract; the storage format is an implementation detail.

For the initial implementation, the structured text format TOML will be used, provided the format provides deterministic parsing and a well-defined specification. MOSAIC will define its own schema independently of the serialization format.

Generated provider configuration does not need to use the same format as MOSAIC configuration.

For example:

```text
~/.config/mosaic/
├── config.toml              # User-owned MOSAIC configuration
├── profiles/
│   └── desktop.toml         # Profile definitions
├── themes/
│   └── default.toml         # Theme definitions
├── layouts/
│   └── default.toml         # Layout definitions
└── overrides/               # Optional explicit user overrides
```

The exact directory structure may evolve as implementation begins, but the ownership distinction must remain.

---

## 5. Configuration Ownership

Every configuration value must have a clear owner.

MOSAIC recognizes three important ownership categories.

### 5.1 User-owned configuration

This is the user's declared MOSAIC configuration and is the primary source of intent.

MOSAIC must not silently overwrite user-owned configuration when applying or regenerating configuration.

### 5.2 Provider-generated configuration

Provider-generated configuration is produced from the resolved MOSAIC model for consumption by external software.

Examples include:

- Waybar configuration
- Hyprland configuration fragments
- SwayNC configuration
- Provider-specific scripts

Generated configuration is an implementation artifact.

The provider may regenerate it whenever the resolved configuration changes.

---

## 6. Source of Truth

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

## 7. Configuration Loading

MOSAIC configuration is loaded before validation or application.

The loader is responsible for locating the configured sources, reading them, parsing them, and producing an in-memory configuration model.

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

- Missing configuration file
- Invalid syntax
- Unsupported serialization format
- Unreadable file
- Duplicate or conflicting declarations

A missing optional source may fall back to its defined default. A missing required source must result in a clear configuration error.

---

## 8. Validation

Validation occurs before configuration is applied.

MOSAIC should distinguish between **schema validation** and **semantic validation**.

### 8.1 Schema validation

Schema validation checks whether configuration has the expected structure and data types.

Examples:

- Required fields exist
- Values have the correct type
- Enumerated values are valid
- Object structure is valid
- Unknown fields follow the project's compatibility policy

### 8.2 Semantic validation

Semantic validation checks whether the requested configuration makes sense in the current MOSAIC environment.

Examples:

- A referenced component exists
- A selected provider supports the requested feature
- A layout references an available component
- A profile does not contain incompatible components
- A required dependency is available

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

---

## 9. Defaults

MOSAIC supports defaults so that users do not need to specify every possible configuration value.

Defaults should be defined by the layer that owns the behavior.

For example:

- MOSAIC core defaults belong to the core.
- Component defaults belong to the component model.
- Provider-specific defaults belong to the provider.

Provider defaults must not redefine MOSAIC-level semantics.

A provider may translate an omitted MOSAIC value into whatever implementation-specific value is required, but the resulting behavior must conform to the MOSAIC model.

Defaults are lower precedence than explicit user configuration.

---

## 10. Configuration precedence

MOSAIC may combine configuration from multiple layers, including built-in defaults, component defaults, profiles, themes, user configuration, and explicit user overrides.

When multiple applicable layers define the same value, MOSAIC resolves the value using a deterministic precedence order.

The default precedence is:

**Built-in defaults → component defaults → profiles/themes → user configuration → explicit user overrides**

Higher-precedence values override lower-precedence values where the configuration domain permits overriding.

Configuration domains may restrict which layers are permitted to provide or override particular values. For example, a theme may provide colors and fonts but should not silently change the user's selected compositor.

The precedence and ownership rules for non-trivial configuration domains must be documented.

---

## 11. User Overrides

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

---

## 12. Merge Semantics

Configuration values must have defined merge behavior.

MOSAIC should not assume that every configuration object can simply be deep-merged.

Values should fall into one of several categories:

### Scalar values

A scalar value has one effective value after precedence resolution.

Example:

```yaml
position: bottom
```

The highest-precedence valid value wins.

### Objects

Objects may be merged field-by-field where the schema explicitly permits it.

### Lists

Lists require an explicit merge strategy.

Possible strategies include:

- Replace
- Append
- Prepend
- Merge by identifier

The default should be **replace** unless a schema explicitly defines another behavior.

This prevents surprising list combinations when profiles and overrides are composed.

### Component sets

Components should normally be identified by stable component IDs rather than list position.

For example:

```yaml
components:
  - id: status-bar
    provider: waybar
  - id: launcher
    provider: rofi
```

This allows configuration to refer to a component independently of its position in a file.

---

## 13. Profiles, Themes, and Layouts

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

Where these concepts overlap, the configuration schema must define the ownership and merge semantics explicitly.

---

## 14. Component Configuration

Each component has a MOSAIC-facing configuration model.

A component configuration should contain only information meaningful at the MOSAIC abstraction level, plus explicitly defined extension points where provider-specific options are necessary.

Conceptually:

```yaml
component:
  id: status-bar
  provider: waybar
  enabled: true
  position: top
  modules:
    - workspaces
    - window
    - clock
```

The component model is passed to the selected provider.

The provider translates it into its own representation:

```text
MOSAIC Status Bar Configuration
            ↓
      Waybar Provider
            ↓
      waybar.jsonc
```

The MOSAIC core should not contain Waybar's configuration syntax merely to support the Waybar provider.

---

## 15. Provider-Specific Configuration

Some external applications expose functionality that cannot reasonably be represented by the common MOSAIC model.

Providers may therefore expose an explicitly scoped provider-specific configuration area.

For example:

```yaml
component:
  id: status-bar
  provider: waybar
  options:
    layer: top
    ipc: true
```

Provider-specific options belong to the provider namespace and must not leak into the generic MOSAIC component model.

This preserves provider independence while still allowing advanced users to access implementation-specific capabilities.

A provider-specific option must never be required for a core MOSAIC concept unless that concept itself is explicitly provider-dependent.

---

## 16. Generated Configuration

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

- Be reproducible from the same resolved configuration
- Be safe to regenerate
- Be clearly identifiable as generated where practical
- Avoid overwriting unrelated user files
- Remain isolated from the user's source configuration

Where possible, generated files should include a marker indicating that they are MOSAIC-managed.

For example:

```text
# Generated by MOSAIC. Do not edit directly.
```

The exact marker is provider-specific.

---

## 17. User-Editable vs Generated Files

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

For example, a provider may generate:

```text
~/.config/hypr/mosaic.conf
```

while the user retains:

```text
~/.config/hypr/hyprland.conf
```

The user's configuration can then include the generated fragment where appropriate.

This approach reduces destructive file ownership conflicts and makes manual customization practical.

---

## 18. Application

Applying configuration means translating the resolved configuration into the external state required to realize it.

Application may involve:

- Generating configuration files
- Updating symlinks
- Creating scripts
- Reloading a provider
- Restarting a component where required
- Applying runtime settings

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

---

## 19. Transaction and Failure Model

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

---

## 20. Runtime State vs Configuration

MOSAIC must distinguish configuration from runtime state.

Configuration describes the desired state.

Runtime state describes what is currently running.

Examples of runtime state include:

- Active workspace
- Current window
- Process IDs
- Current display state
- Temporary provider state

Runtime state should not normally be written back into the user's configuration unless explicitly requested or required for a supported feature.

This prevents transient desktop state from polluting reproducible configuration.

---

## 21. Configuration Versioning

MOSAIC configuration should carry a schema version.

For example:

```yaml
version: 1
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

---

## 22. Reproducibility

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

---

## 23. Portability

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

## 24. Configuration API Boundary

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

---

## 25. Direct User Editing

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

## 26. Configuration Ownership Rules

The following rules are normative for the architecture.

1. **User configuration is the source of truth for user intent.**
2. **Generated provider configuration is an implementation artifact.**
3. **Providers must not modify unrelated component configuration.**
4. **MOSAIC must not silently overwrite user-owned configuration.**
5. **Generated files should be isolated from hand-maintained files where practical.**
6. **Defaults must have lower precedence than explicit user configuration.**
7. **Precedence must be deterministic.**
8. **Merge behavior must be defined by the configuration schema.**
9. **Configuration must be validated before application.**
10. **Provider-specific configuration must remain inside a provider boundary.**
11. **Runtime state must not become configuration implicitly.**
12. **CLI and GUI interfaces must use the same configuration services and resolution rules.**

---

## 27. Complete Configuration Lifecycle

The complete lifecycle is:

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

---

## 28. Architectural Boundaries

The configuration architecture reinforces the boundaries defined by the main MOSAIC architecture.

### Configuration ↔ Core

Configuration provides the declarative representation of user intent. The core provides the models and services used to validate and resolve it.

### Core ↔ Component

The core coordinates components through defined interfaces.

### Component ↔ Provider

A component defines the MOSAIC-level behavior. A provider implements that behavior using a specific technology.

### Provider ↔ External Software

The provider owns translation and application of implementation-specific configuration.

### User ↔ Generated Configuration

The user owns MOSAIC configuration. Generated configuration is managed as an implementation artifact and should not displace the user's source configuration.

---

## 29. Initial Implementation Constraints

The first implementation may make pragmatic choices for the initial Arch Linux and Hyprland environment.

These may include:

- A specific serialization format
- A concrete configuration directory
- Provider-specific generated files
- A limited set of supported merge operations
- A limited component schema

These are implementation decisions, not reasons to weaken the architectural boundaries defined here.

The initial implementation should validate the model against real components such as Hyprland and Waybar while keeping their provider-specific behavior outside the MOSAIC core.

---

## 30. Summary

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
Generated configuration
        ↓
External software
```

This separation ensures that configuration remains modular, reproducible, maintainable, provider-independent, and user-owned while still allowing MOSAIC to generate and manage the concrete configuration required by Linux desktop software.

The configuration architecture should evolve alongside implementation, but changes to these ownership, precedence, and abstraction boundaries should be deliberate and documented.
