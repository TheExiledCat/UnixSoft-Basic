---
hide:
  - toc
---

# Getting Started

---

## Installation

USB can be installed using either the source code and the `cargo` toolchain or by downloading one of the preinstalled binaries from the `release` page and adding it to `PATH`

### Verify installation

To verify the installation, run:

```bash
usbasic --version
```

You should see the installed version printed to stdout

## Your first USB project

usbasic is not just a compiler, its also a fully fledged cli tool used to create projects, install tools and more.

To create a project:

```basic
usbasic init <directory>
```

This will create the following directory and its structure if given (can also be used in an existing directory):

- `/directory/`

  - `src/`

    - `main.usb`

  - `obj/`

  - `build/`

  - `<directoryname>.usbp.json`

  - `.gitignore`

#### Folder structure explained

- `src`:

  - Stores all your USB scripts.

  - The default usbp config will set `src/main.usb` as the default entrypoint for `usbasic build`

- `obj`:

  - Stores temporary files used during building (like `.o` , `.asm` files and more)

- `build`:
  - Stores the actual final binary
- `.usbp.json`
  - This is the config file for USB projects
  - This stores configuration settings, dependencies and more.
- `.gitignore`
  - Default gitignore ignoring the `obj` and `build` dirs
