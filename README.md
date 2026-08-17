# MOSAIC

### Modular System for Assembling Integrated Configurations

MOSAIC is a modular Linux desktop configuration framework designed to make building, managing, and customizing a Linux desktop easier.

Rather than treating a desktop configuration as a single collection of static dotfiles. Mosaic is designed around independent, reusable components that can be combined into a complete desktop environment.

> **Build your desktop. Your Way. **

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

- MOSAIC is intended to provide a layer between the underlying Linux system and the individual components that make up a user's desktop.

- Potential components include:

- - Window managers / compositors
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

The goal is to allow these components to be composed together into different desktop profiles without tightly coupling them to one another.

## Roadmap

The project is currently being built incrementally.

### Phase 1 - Foundation

- [ ] Establish project structure
- [ ] Define configuration architecture
- [ ] Create initial Hyprland configuration
- [ ] Modularize desktop components
- [ ] Create installation/setup process
- [ ] Document configuration structure

### Phase 2 - Profiles

- [ ] Introduce desktop profiles
- [ ] Support configurable themes
- [ ] Support optional components
- [ ] Add system detection
- [ ] Improve installation and update workflows

### Phase 3 - Expansion

- [ ] Explore support for additional compositors/window managers
- [ ] expand component library
- [ ] Improve portability
- [ ] Develop configuration management tooling

> The roadmap is subject to change as the architecture evolves

## Contributing

MOSAIC is intended to be an open-source project and contributions are welcome.

As the project develops. contribution guidelines and development documentation will be added.

## License

MOSAIC is licensed under the [MIT License](License)

---

**MOSAIC**
*Modular System for Assembling Integrated Configurations*
