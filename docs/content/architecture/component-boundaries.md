+++
title = "MOSAIC Component Boundaries"
weight = 6
+++


## Purpose

This document defines the architectural boundaries between MOSAIC subsystems, external desktop components, and underlying platform services.

Clear boundaries help prevent unnecessary coupling, duplicated functionality, and uncontrolled responsibility between components. MOSAIC is designed to integrate and configure an existing Linux desktop environment rather than replace the components that already provide its underlying functionality.

These boundaries define:

* What MOSAIC owns and maintains.
* What MOSAIC configures or integrates.
* What remains the responsibility of external component providers.
* How components communicate with one another.
* How dependencies between MOSAIC subsystems should be managed.

The goal is to allow MOSAIC to remain modular while continuing to use established desktop components through stable and clearly defined integration points.

## Architectural Model

MOSAIC operates as an integration and orchestration layer within an existing Linux desktop environment.

It does not replace the underlying operating system, Wayland compositor, service manager, audio system, or other desktop infrastructure. Instead, MOSAIC provides a structured system for configuring, coordinating, and extending supported components.

The architecture is divided into four responsibility categories:

1. MOSAIC-owned components.
2. MOSAIC-managed external components.
3. External platform services.
4. User-owned configuration and applications.

Each category has a distinct ownership and responsibility boundary.

## MOSAIC-Owned Components

MOSAIC-owned components are implemented and maintained as part of the MOSAIC project.

MOSAIC is responsible for their architecture, interfaces, configuration, behaviour, compatibility, and maintenance.

These components may include:

* The MOSAIC configuration model.
* Configuration loading and validation.
* Component discovery and orchestration.
* MOSAIC-specific services and daemons.
* Shared subsystem interfaces.
* Configuration generation and management.
* Theme and appearance coordination.
* The graphical configuration interface.
* Documentation and compatibility definitions.

MOSAIC-owned components should expose clear public interfaces to other MOSAIC subsystems and should avoid requiring consumers to understand their internal implementation.

A subsystem should depend on another subsystem's public contract rather than its internal structure.

## MOSAIC-Managed External Components

MOSAIC-managed components are external projects that MOSAIC configures, coordinates, or integrates but does not own.

Examples may include:

* Hyprland.
* Waybar.
* QuickShell.
* SwayNC.
* hypridle.
* hyprlock.
* hyprpaper.
* wlogout.

MOSAIC may provide configuration, startup coordination, theme information, or integration logic for these components.

However, MOSAIC does not assume responsibility for their internal implementation.

For example, MOSAIC may configure Hyprland to use a particular monitor layout, keybinding, animation, or window rule. Hyprland remains responsible for interpreting that configuration and providing compositor functionality.

Likewise, MOSAIC may configure Waybar or QuickShell as part of the desktop environment without becoming responsible for implementing their rendering systems or internal application behaviour.

Integration does not imply ownership.

## External Platform Services

External platform services provide functionality that MOSAIC depends on but does not replace.

These services remain responsible for their own implementation, lifecycle, security, and platform-specific behaviour.

Examples include:

* Linux.
* systemd.
* D-Bus.
* Polkit.
* PipeWire.
* WirePlumber.
* The Wayland protocol.
* XDG desktop standards.

MOSAIC may interact with these services through their documented interfaces.

For example:

* systemd may manage MOSAIC or desktop services.
* D-Bus may provide communication between compatible services.
* Polkit may provide authorisation for privileged operations.
* PipeWire and WirePlumber remain responsible for audio and media routing.
* Hyprland and other Wayland components remain responsible for compositor-specific functionality.

MOSAIC should not duplicate functionality already provided by these platform services unless a clear architectural requirement exists.

## User-Owned Configuration and Applications

MOSAIC should distinguish between configuration required for the operation of the MOSAIC environment and configuration that belongs to the user.

Users remain responsible for:

* Personal applications.
* Application-specific configuration.
* Unsupported third-party tools.
* Custom scripts outside the MOSAIC architecture.
* Local modifications that intentionally diverge from supported MOSAIC behaviour.

MOSAIC may provide supported extension points for user customisation, but user customisation should not require modification of MOSAIC internals.

Where possible, supported user configuration should remain separate from generated or MOSAIC-managed configuration.

This separation reduces the risk of updates overwriting user changes and allows MOSAIC to clearly identify the configuration required to support a known environment.

## Configuration Boundaries

MOSAIC may configure external components through supported and documented mechanisms.

These mechanisms may include:

* Native configuration files.
* Environment variables.
* Command-line interfaces.
* Documented IPC interfaces.
* D-Bus interfaces.
* systemd service definitions.
* Public APIs.

MOSAIC should avoid depending on undocumented implementation details or modifying the private state of another component.

In particular, a component should not require knowledge of another component's internal configuration structure unless that structure forms part of a documented integration contract.

Configuration ownership should also be clear.

MOSAIC-generated configuration should be identifiable as MOSAIC-managed and should not be confused with unrelated user configuration.

Where generated configuration is required, the preferred approach is to keep generated output separate from user-owned configuration and use supported inclusion or extension mechanisms where available.

## Communication Boundaries

MOSAIC components should communicate through defined interfaces appropriate to the responsibility being shared.

The communication mechanism should be selected based on the type of interaction rather than allowing arbitrary dependencies between subsystems.

Supported mechanisms may include:

* Shared MOSAIC configuration interfaces.
* Defined internal APIs.
* Documented IPC protocols.
* D-Bus.
* Command-line interfaces.
* Environment variables.
* Files used as explicit configuration or state interfaces.
* systemd service relationships.

A component should communicate only with the information or capability it requires.

For example, a theme subsystem may provide a theme definition or resolved appearance values to a consumer without requiring that consumer to understand how the theme was selected or generated.

Likewise, a desktop component should interact with another component through a defined contract rather than directly modifying that component's internal state.

## Dependency Rules

MOSAIC subsystems should follow the following dependency rules:

1. A subsystem should depend on public interfaces rather than implementation details.

2. Dependencies should flow through defined architectural boundaries.

3. A subsystem should not directly modify the private configuration or runtime state of another subsystem.

4. Circular dependencies between subsystems should be avoided.

5. Shared functionality should be provided through an explicit shared interface rather than duplicated across components.

6. External component integrations should be isolated where practical so that changes to one provider do not unnecessarily affect unrelated MOSAIC subsystems.

7. A component should own its internal state and expose only the information or operations required by consumers.

8. Communication mechanisms should use documented and supported interfaces wherever possible.

These rules are intended to keep individual subsystems replaceable and reduce the impact of changes across the wider architecture.

## Component Ownership Model

The following model defines the general relationship between MOSAIC and the components it integrates.

| Component Category      | Example                       | MOSAIC Owns It | MOSAIC May Configure It      | Primary Runtime Responsibility |
| ----------------------- | ----------------------------- | -------------- | ---------------------------- | ------------------------------ |
| MOSAIC subsystem        | Configuration service         | Yes            | Yes                          | MOSAIC                         |
| Wayland compositor      | Hyprland                      | No             | Yes                          | Hyprland                       |
| Desktop shell component | Waybar or QuickShell          | No             | Yes                          | Component provider             |
| Desktop service         | SwayNC or hypridle            | No             | Yes                          | Component provider             |
| Platform service        | systemd or D-Bus              | No             | Integrates with it           | Platform provider              |
| Media service           | PipeWire or WirePlumber       | No             | Integrates with it           | Media stack                    |
| User application        | Third-party application       | No             | Generally no                 | Application provider           |
| User customisation      | Personal scripts or overrides | No             | Through supported interfaces | User                           |

The exact set of supported components may change as MOSAIC evolves, but the ownership model remains consistent.

## Avoiding Unnecessary Coupling

A component integration should not create dependencies beyond those required to provide the intended functionality.

For example, a status bar does not need direct knowledge of the internal implementation of a lock screen. Both components may instead consume shared configuration or interact with a common MOSAIC service.

Similarly, a theme change should not require every component to communicate directly with every other component. A central theme interface or service can provide the required information while allowing individual consumers to remain independent.

The preferred architectural pattern is:

```text
                    MOSAIC Interface
                           │
              ┌────────────┼────────────┐
              │            │            │
         Component A   Component B   Component C
```

Rather than:

```text
Component A ─────── Component B
     │                   │
     │                   │
     └──────── Component C
```

The first model establishes a defined communication boundary. The second model creates direct relationships that become increasingly difficult to manage as additional components are introduced.

Direct communication between components is not prohibited where it is necessary and supported, but it should not become the default architecture.

## External Provider Boundaries

MOSAIC must respect the responsibility boundaries of external providers.

For example, Hyprland remains responsible for:

* Window management.
* Input handling.
* Monitor management.
* Wayland compositor behaviour.
* Rendering and compositor-specific functionality.
* Hyprland IPC and configuration interpretation.

MOSAIC may configure or request behaviour from Hyprland through supported mechanisms, but should not duplicate compositor functionality.

The same principle applies to other external components.

MOSAIC should integrate existing functionality where appropriate rather than reimplementing functionality simply to bring it under MOSAIC ownership.

A new MOSAIC-owned implementation should only be introduced when existing components cannot provide the required functionality or when ownership is necessary to meet a defined architectural requirement.

## Replaceability

Where practical, MOSAIC integrations should be designed so that external components can be replaced without requiring unrelated subsystems to be redesigned.

This does not require MOSAIC to support every possible provider.

Instead, integrations should be isolated behind boundaries that prevent provider-specific assumptions from spreading throughout the architecture.

For example, a subsystem that requires information about the active workspace should depend on a workspace interface rather than requiring knowledge of Hyprland-specific IPC throughout the MOSAIC codebase.

A Hyprland integration layer may implement that interface while a future alternative integration could provide the same contract through a different mechanism.

Replaceability should be considered where it provides meaningful architectural value, not as an abstraction requirement for every individual feature.

## Boundary Decisions

When introducing a new feature or component, the following questions should be considered:

1. Is this functionality owned by MOSAIC or by an existing external component?

2. If an external component provides the functionality, does MOSAIC need to configure it, integrate with it, or simply coexist with it?

3. What is the public interface between the components?

4. Does the integration depend on documented and supported mechanisms?

5. Does the dependency introduce unnecessary knowledge of another component's internal implementation?

6. Can the component be changed or replaced without affecting unrelated subsystems?

7. Is configuration ownership clearly defined?

8. Does the feature belong in an existing subsystem rather than creating a new dependency?

These questions should be considered before introducing new cross-component dependencies.

## Summary

MOSAIC is responsible for the systems it owns and for coordinating the supported components that form the MOSAIC desktop environment.

External components remain responsible for their native functionality. MOSAIC configures and integrates those components through documented interfaces without assuming ownership of their internal implementation.

Subsystems should communicate through defined contracts and supported mechanisms rather than direct access to private state or implementation details.

The central architectural principle is:

> **MOSAIC coordinates the desktop environment without unnecessarily owning, duplicating, or tightly coupling the components that provide it.**

These boundaries provide the foundation for implementing MOSAIC as a modular and maintainable desktop platform while allowing individual components and integrations to evolve independently.
