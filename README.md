# MOSAIC

### Modular System for Assembling Integrated Configurations

MOSAIC is a modular Linux desktop configuration framework designed to make building, managing, and customizing a Linux desktop easier.

Rather than treating a desktop configuration as a single collection of static dotfiles. Mosaic is designed around independent, reusable components that can be combined into a complete desktop environment.

> **Build your desktop. Your Way.**

## Project Status

MOSAIC is currently in early development. if your viewing this, I Literally just started this project idea and hope to continue it.

The initial implementation is being developed around **Hyprland**, with the long-term goal of supporting a broader range of Linux desktop environments, window managers, compositors, and desktop components.

The architecture is intentionally designed to avoid being tied to any single desktop environment or compositor.

## Goals

- **Modular** - Desktop components should be independent and reusable.
- **Configurable** - Users should be able to customize individual components without rebuilding their entire configuration.
- **Reproducible** - A complete desktop configuration should be easy to recreate on another system.
- **Maintainable** - Configuration should remain organized as the project grows.
- **Linux-first** - Designed specifically around the flexibility of the Linux desktop ecosystem.
- **Extensible** - New components and integrations should be easy to add.
- **User-owned** - Mosaic should provide a foundation, not dictate how a user's desktop must look or work.

- ## Architecture

MOSAIC is intended to provide a layer between the underlying Linux system and the individual components that make up a user's desktop.

Potential components include:

- Window managers / compositors
- Status bars
- Application launchers
- Terminals
- Shell configurations
- Notifications
- Wallpapers
- Themes
- Fonts
- Keybindings
- System utilities
- Desktop scripts
- Application configuration

These components should be able to be composed together into different **profiles**, allowing users to create complete desktop configurations without tightly coupling the individual components to one another.

MOSAIC should also preserve the ability for advanced users to directly modify or replace underlying configurations when they want full control.

## Profiles, Themes & Layouts

MOSAIC is designed around the idea that different parts of a desktop configuration should remain separate.

A **profile** describes the components and configuration that make up a particular desktop setup.

A **theme** controls the visual appearance of those components.

A **layout** describes how interface components such as bars and widgets are arranged.

This separation allows users to mix and match configurations.

For example, a user could use one person's profile, another person's theme, and a third person's layout without having to adopt an entire desktop configuration.

In the future, MOSAIC may support importing and exporting these configurations so users can create and share their own desktop setups.

## User Customization

MOSAIC is intended to support both experienced Linux users and people who are less comfortable working directly with configuration files.

Advanced users should be able to edit the underlying configuration directly.

Less technical users should eventually be able to configure their desktop through MOSAIC's graphical tools.

Long-term goals include:

- Graphical configuration
- Theme creation and editing
- Wallpaper management
- Profile creation
- Layout selection
- Visual layout editing
- No-code customization
- Accessibility settings
- Community-created themes, profiles, layouts, and plugins

The goal is not to hide Linux configuration from experienced users, but to make that configuration more approachable for everyone else.

## Development

MOSAIC is being developed incrementally.

The project begins with a Hyprland-based implementation so that the architecture can be tested against a real desktop configuration before attempting broader compositor support.

Development is tracked publicly through the **MOSAIC Roadmap** GitHub Project.

The roadmap contains the current development phases, issues, priorities, and planned work.

As the project evolves, the roadmap will change alongside the architecture.

## Contributing

MOSAIC is intended to be an open-source project, and contributions are welcome.

The project is currently in early development, so the architecture is still evolving.

Development documentation, contribution guidelines, and additional information for contributors will be added as the project matures.

If you are interested in MOSAIC, feel free to follow the project, experiment with it, open issues, or contribute ideas.

## License

MOSAIC is licensed under the [MIT License](License).

---

**MOSAIC**  
*Modular System for Assembling Integrated Configurations*
