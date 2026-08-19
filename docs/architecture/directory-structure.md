# MOSAIC Directory Structure

**Status:** Proposed  
**Issue:** #3 — Define MOSAIC directory structure

## 1. Purpose

This document defines the directory structure used by MOSAIC across its source repository, installed resources, user configuration, generated configuration, persistent state, runtime data, backups, and documentation.

The directory structure exists to support the principles established by the MOSAIC architecture and configuration architecture:

- MOSAIC is modular and component-oriented.
- User configuration is separate from generated configuration.
- Built-in resources are separate from user-owned resources.
- Generated files are implementation artifacts rather than the source of truth.
- User overrides must survive updates to built-in MOSAIC resources.
- Runtime state must remain separate from persistent configuration.
- The structure should follow established Linux filesystem conventions where practical.
- The repository structure and installed filesystem structure are separate concerns.

This document defines the intended locations and ownership of MOSAIC data. It does not create or implement these directories.

---

## 2. Design Principles

The MOSAIC filesystem structure follows several principles.

### 2.1 Separation of ownership

Files should have a clear owner.

```text
MOSAIC source
    ↓
Built-in resources
    ↓
System installation
    ↓
User configuration
    ↓
Resolved configuration
    ↓
Generated output
    ↓
Runtime state
```

User-owned files must not be overwritten by MOSAIC updates or configuration generation.

### 2.2 Separation of source and generated data

MOSAIC configuration represents user intent.

Generated provider configuration represents the implementation of that intent.

These must not be treated as the same thing.

### 2.3 Standard Linux filesystem conventions

Where practical, MOSAIC should use established Linux filesystem conventions rather than inventing a completely independent filesystem hierarchy.

### 2.4 Modular resources

Components, profiles, themes, and layouts should remain independently identifiable and replaceable.

The directory structure must not assume that MOSAIC is permanently tied to one compositor, window manager, or desktop component.

---

## 3. Repository Structure

The MOSAIC source repository contains the source code, built-in resources, documentation, tests, and project metadata required to develop and build MOSAIC.

The proposed repository structure is:
```
MOSAIC/
├── src/
│   └── mosaic/
│       ├── core/
│       ├── configuration/
│       ├── components/
│       ├── profiles/
│       ├── themes/
│       ├── layouts/
│       ├── providers/
│       └── runtime/
│
├── resources/
│   ├── components/
│   ├── profiles/
│   ├── themes/
│   └── layouts/
│
├── docs/
│   ├── architecture/
│   ├── configuration/
│   ├── components/
│   └── guides/
│
├── tests/
│   ├── unit/
│   ├── integration/
│   └── fixtures/
│
├── packaging/
│
├── scripts/
│
├── LICENSE
├── README.md
└── ...
```

The exact implementation language and package layout are outside the scope of this document.

The important architectural distinction is between **MOSAIC core code**, **reusable MOSAIC resources**, **documentation**, and **tests**.

---

## 4. Source Code

MOSAIC source code belongs under:
```
src/
```

The source tree should contain the implementation of MOSAIC itself.

Conceptually:
```
src/mosaic/
├── core/
├── configuration/
├── components/
├── profiles/
├── themes/
├── layouts/
├── providers/
└── runtime/
```

```core/```

Contains functionality fundamental to MOSAIC itself.

Examples include:
- Application lifecycle
- Command handling
- Logging
- Error handling
- Core abstractions
- Resource discovery

Core code must not contain provider-specific implementation details unless those details are part of an explicitly defined abstraction.

```configuration/```

Contains configuration loading, parsing, validation, resolution, precedence, and related functionality.

This area implements the configuration architecture defined separately in the MOSAIC configuration architecture document.

```components/```

Contains the implementation of MOSAIC component concepts.

Components should describe what a desktop function represents rather than being tightly coupled to one external application.

```providers/```

Contains integrations with external applications.

Examples may eventually include providers for:
- Hyprland
- Waybar
- SwayNC
- wlogout
- hypridle
- Rofi
- Other supported applications

Provider code translates MOSAIC's resolved configuration into implementation-specific configuration.

```runtime/```

Contains functionality concerned with active MOSAIC operation and runtime state.

Runtime code should not become the source of truth for persistent user configuration.

---

## 5. Built-in Resources

Reusable resources supplied by MOSAIC belong under:

```
resources/
```

Conceptually:
```
resources/
├── components/
├── profiles/
├── themes/
└── layouts/
```

These resources are part of the MOSAIC distribution.

They are not user-owned configuration.

### Components

``` resources/components```

Comtains built-in component definitions.

Examples:
```
resources/components/
├── launcher/
├── notification-center/
├── status-bar/
├── lock-screen/
└── idle-daemon/
```

### Profiles

``` resources/profiles/```

Contains built-in profiles describing collections of components.

Examples:
```
resources/profiles/
├── minimal/
├── workstation/
└── gaming/
```

### Themes

```resources/themes/```

Contains built-in visual themes

Examples:
```
resources/themes/
├── default/
├── dark/
└── light/
```

### Layouts

```resources/layouts/```

Contains built-in layout definitions.

Resources should be identified using stable names or IDs rather than relying on filesystem position.

---

## 6. Installed MOSAIC Resources

Built-in MOSAIC resources should be installed separately from user configuration.

The intended system-wide resource location is:

```/usr/share/mosaic/```

Conceptually:
```
/usr/share/mosaic/
├── components/
├── profiles/
├── themes/
└── layouts/
```

These resources are installed by the MOSAIC package.

They should be considered **read-only from the perspective of normal MOSAIC operation**.

Users should not need to modify these files to customize their desktop.

This allows MOSAIC packages to update built-in resources without destroying user configuration.

---

## 7. System Configuration

System-level MOSAIC configuration belongs under:

```/etc/mosaic/```

Conceptually:
```
/etc/mosaic/
├── config.toml
├── profiles/
├── themes/
├── layouts/
└── overrides/
```

System configuration is intended for configuration that applies outside a single user's home directory.

System configuration must remain separate from:

```/usr/share/mosaic/```

because installed resources and configuration have different ownership and lifecycle rules.

```/usr/share/mosaic/``` contains resources supplied by MOSAIC.

```/etc/mosaic/``` contains administrator-controlled configuration.

---

## 8. User Configuration

User-owned MOSAIC configuration belongs under:

```~/.config/mosaic/```

This is the primary location for user configuration.

Conceptually:
```
~/.config/mosaic/
├── config.toml
├── profiles/
├── themes/
├── layouts/
├── components/
└── overrides/
```

The user's MOSAIC configuration is the source of truth for their configuration intent.

MOSAIC must not silently replace or regenerate these files as though they were generated configuration.

---

### 8.1 User Components

User-defined or user-customized component definitions may be stored under:

```~/.config/mosaic/components/```

These resources can extend or override built-in resources according to the configuration precedence rules.

---

### 8.2 User Profiles

User-defined profiles belong under:

```~/.config/mosaic/profiles/```

A user should be able to create a profile without modifying the corresponding built-in MOSAIC profile.

---

### 8.3 User Themes

User-defined themes belong under:

```~/.config/mosaic/themes/```

This allows users to create and maintain their own visual configurations independently of built-in themes.

---

### 8.4 User Layouts

User-defined layouts belong under:

```~/.config/mosaic/layouts/```

Layouts should remain independent of provider-specific generated configuration.

---

### 8.5 User Overrides

Explicit user overrides belong under:

```~/.config/mosaic/overrides/```

Overrides provide a deliberate mechanism for changing values supplied by built-in resources, profiles, themes, layouts, or defaults.

They should not depend on undocumented file ordering.

---

## 9. Persistent User Data

Persistent MOSAIC application data that is not configuration belongs under:

```~/.local/share/mosaic/```

This location is intended for persistent data that MOSAIC needs to retain but that does not represent configuration.

Examples may eventually include:
- Resource metadata
- Downloaded resources
- Cached persistent assets
- User-created data that is not configuration

Configuration should not be stored here when it belongs under ```~/.config/mosaic/```.

---

## 10. Generated Configuration

Generated configuration must be kept separate from user-owned MOSAIC configuration.

MOSAIC should preferably generate provider configuration into provider-managed locations rather than overwriting the user's primary configuration files.

For example:
```
~/.config/hypr/
├── hyprland.conf
└── mosaic.conf
```
where:

```hyprland.conf```

remains user-owned while:

```mosaic.conf```

is generated by MOSAIC.

Where practical, generated files should contain a marker such as:

```# Generated by MOSAIC. Do not edit directly.```

Generated configuration is derived from resolved MOSAIC configuration and may be regenerated at any time.

It must therefore never be treated as the primary source of truth.

---

## 11. Generated State

MOSAIC's own generated or persistent state belongs under:

```~/.local/state/mosaic/```

Conceptually:
```
~/.local/state/mosaic/
├── generated/
└── backups/
```

This location is distinct from both:

```~/.config/mosaic/```

and:

```~/.local/share/mosaic/```

because generated state and configuration have different purposes.

## 12. Backups

MOSAIC-managed backups belong under:

~/.local/state/mosaic/backups/

Backups may contain previous versions of generated configuration or other files MOSAIC has modified as part of an application operation.

Conceptually:
```
~/.local/state/mosaic/backups/
├── <timestamp>/
├── <timestamp>/
└── ...
```

Backup retention and naming policies are implementation concerns.

MOSAIC must not treat backups as active configuration.

Backups exist to support recovery and rollback.

---

## 13. Runtime Data

Ephemeral runtime data belongs under the standard Linux runtime hierarchy:

```/run/mosaic/```

Runtime data may include:
- PID files
- Unix sockets
- Locks
- Temporary runtime metadata
- Active-session information

Runtime data should not survive system shutdown unless explicitly required.

MOSAIC must not use ```/run/mosaic/``` as a persistent configuration location.

---

## 14. Cache Data

Cache data should use the standard user cache location:

```~/.cache/mosaic/```

Cache data may include:
- Temporary downloaded resources
- Parsed resource caches
- Provider discovery caches
- Other regenerable data

Cache contents must be safe to remove.

MOSAIC must never rely on cache data as the only copy of user configuration or persistent state.

---

## 15. Documentation

Project documentation belongs under:

```docs/```

Architecture documentation belongs under:

```docs/architecture/```

Configuration documentation belongs under:

```docs/configuration/```

Component documentation may belong under:

```docs/components/```

User-facing guides belong under:

```docs/guides/```

Documentation should explain architectural contracts and expected behavior without becoming a substitute for the implementation itself.

---

## 16. Directory Ownership Model

The overall ownership model can be summarized as:
```
/usr/share/mosaic/          MOSAIC-provided resources
        │
        ▼
/etc/mosaic/                System configuration
        │
        ▼
~/.config/mosaic/           User configuration
        │
        ▼
Resolved MOSAIC state
        │
        ├──> Provider-generated configuration
        │
        └──> ~/.local/state/mosaic/
                    │
                    ├── generated/
                    └── backups/

~/.local/share/mosaic/      Persistent application data

~/.cache/mosaic/            Regenerable cache data

/run/mosaic/                Ephemeral runtime data
```

Each location has a distinct responsibility.

---

## 17. Source of Truth

The source-of-truth hierarchy is:
```
Built-in resources
        ↓
System configuration
        ↓
User configuration
        ↓
Explicit overrides
        ↓
Resolved MOSAIC configuration
        ↓
Generated provider configuration
        ↓
External applications
```

The exact precedence between individual configuration sources is defined by the MOSAIC configuration architecture.

Generated configuration is always downstream of the resolved MOSAIC configuration.

Generated files must not silently become authoritative simply because they exist on disk.

---

## 18. User Customization and Updates

The directory structure must allow MOSAIC to be updated without destroying user customization.

For example:

```/usr/share/mosaic/themes/dark/```

may be replaced by a future MOSAIC package update.

The user's:

```~/.config/mosaic/themes/dark/```

must remain unaffected.

Likewise, a user-created:

```~/.config/mosaic/profiles/workstation/```

must not be overwritten by changes to the built-in profiles.

This separation is a fundamental requirement of the directory architecture.

---

## 19. Provider Isolation

MOSAIC provider implementations must not require users to place provider-specific configuration inside the MOSAIC source or resource directories.

For example, Waybar configuration should not require:

```~/.config/mosaic/waybar/```

unless the provider explicitly defines such a location.

Instead, the provider should translate the MOSAIC configuration model into the configuration expected by Waybar.

This preserves the distinction between:

```MOSAIC configuration```

and:

```Provider configuration```

and prevents MOSAIC from becoming a collection of unrelated application configuration files.

---

## 20. What Must Not Be Stored in Each Location

| Location                 | Must not contain                                     |
| ------------------------ | ---------------------------------------------------- |
| `/usr/share/mosaic/`     | User configuration or mutable runtime state          |
| `/etc/mosaic/`           | Generated provider output or ephemeral runtime state |
| `~/.config/mosaic/`      | Temporary runtime data or disposable caches          |
| `~/.local/share/mosaic/` | Primary user configuration                           |
| `~/.local/state/mosaic/` | Primary user configuration                           |
| `~/.cache/mosaic/`       | Data required to reconstruct user configuration      |
| `/run/mosaic/`           | Persistent configuration or long-term backups        |

The purpose of these restricitons is to keep ownership and lifecycle predictable.

---

## 21. Proposed Final Structure

The resulting MOSAIC filesystem architecture is:
```
Repository
MOSAIC/
├── src/
├── resources/
├── docs/
├── tests/
├── packaging/
└── scripts/


System
/usr/share/mosaic/
├── components/
├── profiles/
├── themes/
└── layouts/

/etc/mosaic/
├── config.toml
├── profiles/
├── themes/
├── layouts/
├── components/
└── overrides/


User
~/.config/mosaic/
├── config.toml
├── components/
├── profiles/
├── themes/
├── layouts/
└── overrides/

~/.local/share/mosaic/

~/.local/state/mosaic/
├── generated/
└── backups/

~/.cache/mosaic/

/run/mosaic/
```

This structure provides clear separation between:
- MOSAIC source code
- Built-in resources
- System configuration
- User configuration
- User overrides
- Generated configuration
- Persistent application data
- Cache data
- Backups
- Runtime data
- Documentation

---

## 22. Architectural Decision

MOSAIC will use a layered filesystem structure based on standard Linux conventions.

The key architectural rule is:

> **User-owned configuration, MOSAIC-provided resources, generated configuration, persistent state, cache data, backups, and runtime data must remain separate.**

This separation allows MOSAIC to remain modular, reproducible, updateable, and safe for user customization while avoiding destructive ownership conflicts with the applications it manages.

The exact implementation of individual directories may evolve as MOSAIC develops, but the ownership and lifecycle boundaries defined by this document should remain stabl
