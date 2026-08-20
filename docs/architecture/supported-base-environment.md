# MOSAIC Supported Base Environment

**Status:** Proposed

**Issue:** #4 — Define supported base environment

## 1. Purpose

This document defines the Linux environment that MOSAIC officially supports for its initial release.

MOSAIC is intentionally designed around a constrained and predictable base environment. The initial release will focus on providing a reliable experience within this environment rather than attempting to support every Linux distribution, compositor, desktop architecture, or hardware configuration.

Support may be expanded in future releases as the project matures.

---

## 2. Officially Supported Environment

The initial release of MOSAIC officially supports the following environment:

| Area                   | Supported Environment     |
| ---------------------- | ------------------------- |
| Distribution           | Arch Linux                |
| Architecture           | x86_64                    |
| Display Protocol       | Wayland                   |
| Compositor             | Hyprland                  |
| Init / Service Manager | systemd                   |
| Session Management     | systemd-logind            |
| Audio                  | PipeWire                  |
| IPC / Messaging        | D-Bus                     |
| Authentication         | Polkit                    |
| Desktop Integration    | XDG standards and portals |

This environment represents the primary development and testing target for MOSAIC.

## 3. Distribution

### Arch Linux

Arch Linux is the initial supported distribution for MOSAIC.

Arch provides a minimal and modular base that aligns with MOSAIC's design philosophy. It allows MOSAIC to assemble desktop components without requiring an existing desktop environment or large collection of distribution-specific tooling.

MOSAIC may depend on packages provided through the Arch Linux package ecosystem.

The initial release does not guarantee compatibility with Arch-based distributions such as Manjaro, EndeavourOS, Garuda Linux, or other derivatives.

These distributions may work, but they are not considered officially supported unless explicitly added in a future release.

## 4. Display Protocol and Compositor

### Wayland

MOSAIC is designed for Wayland sessions.

X11 is not an official target for the initial release.

### Hyprland

Hyprland is the officially supported compositor for the initial release.

MOSAIC's initial component architecture and configuration model are designed around Hyprland's Wayland environment.

Future versions may support additional compositors where practical, but compositor independence is not a requirement for the initial release.

## 5. Required Runtime Environment

MOSAIC assumes a modern Linux system providing the following foundational services:

* `systemd` for system and user service management.
* `systemd-logind` for session and seat management.
* D-Bus for inter-process communication.
* PipeWire for modern audio and media integration.
* Polkit for privileged desktop operations requiring authentication.
* XDG standards for desktop integration, configuration, and application interoperability.
* XDG desktop portals where required for applications and desktop integration.

Individual MOSAIC components may introduce additional runtime dependencies. These dependencies should be documented by the relevant component rather than treated as universal MOSAIC requirements.

## 6. Hardware Assumptions

MOSAIC does not require specialised hardware.

The initial release assumes a reasonably modern 64-bit PC capable of running Arch Linux and Hyprland.

The system should provide:

* An x86_64 processor.
* A GPU with functional Linux and Wayland support.
* Working graphics drivers appropriate to the installed GPU.
* Sufficient system memory and storage for the operating system and MOSAIC components.

MOSAIC does not guarantee identical behaviour across all GPU vendors, driver versions, firmware configurations, or unusual hardware configurations.

Hardware-specific compatibility issues are outside the primary scope of the initial release unless they directly affect a supported configuration.

## 7. Explicitly Unsupported Environments

The following environments are outside the official scope of the initial release:

* X11-only sessions.
* Compositors other than Hyprland.
* Non-Arch Linux distributions.
* Arch Linux derivatives.
* Non-systemd init systems.
* Unsupported CPU architectures such as ARM.
* Unusual or specialised Linux environments where the required desktop services are unavailable.
* Configurations that require distribution-specific behaviour not present in the supported environment.

An unsupported environment may still be capable of running MOSAIC. However, successful operation in such an environment should not be considered a project compatibility guarantee.

## 8. Support Policy

Official support means that the MOSAIC project will design, document, and test the initial release against the defined base environment.

Issues that occur within the supported environment should be considered valid compatibility concerns and investigated where appropriate.

Issues that occur exclusively within unsupported environments may be closed as outside the project's current support scope.

Expanding the supported environment is a deliberate project decision and should occur through a future architecture or compatibility review rather than through undocumented, ad-hoc support.

## 9. Future Expansion

The supported environment defined here applies only to the initial release.

Future releases may expand support to additional:

* Linux distributions.
* Wayland compositors.
* CPU architectures.
* Hardware configurations.
* Desktop integration environments.

Such expansion should occur only when MOSAIC's architecture can support the additional environment without compromising maintainability or the reliability of the existing supported configuration.

## 10. Summary

MOSAIC's initial release targets a deliberately narrow environment:

> **Arch Linux + x86_64 + Wayland + Hyprland + systemd**

with modern Linux desktop services such as D-Bus, PipeWire, Polkit, and XDG integration.

This constrained scope allows the project to establish a stable and predictable foundation before attempting broader compatibility.
