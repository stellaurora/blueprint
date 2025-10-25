<h1 align="center">Blueprint</h1>

<div align="center">
🏗️📝📐📏🔨
</div>
<div align="center">
  <strong>Declarative Package Manager for Linux</strong>
</div>
<div align="center">
  A <code>TOML</code> file-based declarative package manager.
</div>


## 📜 Table of Contents
- [<code>📦 Getting Started</code>](#getting-started)
- [<code>🛠️ Features</code>](#features)
- [<code>📝 Usage</code>](#usage)
- [<code>⚙️ Config Examples</code>](#config-examples)
- [<code>📖 Configuration</code>](#configuration)
- [<code>🧾 License</code>](#license)
- [<code>🎓 Acknowledgments</code>](#acknowledgments)

<a name="getting-started"></a>
## 📦 Getting Started 

### For Linux x86_64

You can download prebuilt binaries off the latest runner build at **https://github.com/stellaurora/blueprint/releases/latest**. Then verify the checksum and simply provide execution permissions to the binary and use. The tiny version is compiled for binary size and uses the ``UPX`` on the binary for even more compression.

The entirety of blueprint can be used through this one binary, run it to view the initial ``help`` command to learn more or ``init`` to generate a basic template configuration file.

Currently blueprint only supports the ``paru`` package manager for ``Arch Linux``.

### For ???

Blueprint is for ``Linux`` and currently for github releases is only compiled under the CI for ``x86_64`` if you are running on another platform, you will need to manually clone and build blueprint yourself (unknown if it will work on other platforms!).

<a name="features"></a>
## 🛠️ Features

- Entirely ``TOML`` file-based configuration 
  - Supports "linking" (including) multiple ``TOML`` files together for a more modular configuration.

- Per-Package source
  - Supports having multiple sources for packages in one configuration file, so each package can come from where you want it to.

- Entirely Configurable
  - All of the above features can be totally/partially customised in functionality, If you don't want to check checksums then you don't need to!

<a name="usage"></a>
## 📝 Usage

The primary/entire use of blueprint is through the command:

```
blueprint apply --file <ROOT_CONFIG>
```

This apply command will first ensure all packages as specified by the configuration files managed under blueprint are installed on the system using their associated sources.

Then after all of that is ran, then all of the unecessary packages (orphans e.g) are removed from the system.

```
blueprint init --file <FILE_PATH>
```

The file argument is optional, and will simply default to ``blueprint.toml`` if not provided, The general flow of blueprint is to then edit this file (and associated ones) and use it with the ``apply`` command.

For any more information about the blueprint commands, the command:

```
blueprint help
```

can be used, though this does not cover the configuration options.


<a name="config-examples"></a>
## ⚙️ Config Examples

### Example/Default Configuration

This is a configuration that demonstrates the functionality of ``blueprint``, Read the comments in the file to learn more and read more about the various configuration options below..

```toml
# My first blueprint config

# [[link]]
# Links to another blueprint configuration file (in same format)
# and essentially "includes/runs" it as part of this configuration.
[[link]]

# File paths are relative to this configuration file.
file="blueprint_other.toml"

# [[package]]
# An individual package to
# install in the system
[[package]]

# Name of the package
name="linux-zen"

# [[multi_packages]]
# A shorthand for many different
# packages at once to be installed from the same source
[[multi_packages]]
names = [
    "linux-lts",
    "linux"
]
```

<a name="configuration"></a>
## 📖 Configuration

These are the various configuration options available in blueprint configs.

### Global Configuration

These configuration options will only be taken in from the root configuration file referenced by the ``apply`` command, any global config referenced in ``link`` will be ignored in order to reduce confusion about configuration.

This is supplied under the ``[config]`` table of the file.

------------------

#### ``shell``

Which shell to use for running package manager related commands?

type: ``string``

```toml 
[conifg]
shell="bash"
```

------------------

#### ``shell_exec_arg``

Argument to pass to shell for it to be capable of running commands. 

This will be the first argument, and commands needed to be executed as the second argument

type: ``string``

```toml 
[conifg]
shell_exec_arg="-c"
```

------------------

#### ``prompt_apply_continue``

Confirm inside of blueprint whether or not to continue the apply operation before running anything?

type: ``bool``

```toml 
[conifg]
prompt_apply_continue=true
```

------------------

#### ``prompt_install_per_source``

Confirm for each installation source whether to proceed with installing

type: ``bool``

```toml 
[conifg]
prompt_install_per_source=true
```

------------------

#### ``prompt_removal_per_source``

Confirm for each installation source whether to proceed with removing uneeded packages

type: ``bool``

```toml 
[conifg]
prompt_removal_per_source=true
```

------------------

#### ``remove_unrequired_software``

Global toggle for removing software determined as "unrequired" which is just software that is not
in the package list or a dependency of package list
software as determined by the package source.

type: ``bool``

```toml 
[conifg]
remove_unrequired_software=true
```

### Links

This is an array of files specified each individually under the array table ``[[link]]``, each link is like including the file and will execute its contents as part of the blueprint system (excluding ``config`` for non-root configs).

---------------

#### ``file``

Links to another blueprint configuration file under the path supplied, this is relative to the current blueprint configuration file.

Can be used in any blueprint configuration file to "include" it into the overall configuration in order to have better modularity/cleaner file structure for the system configuration.

type: ``string``

```toml 
[[link]]
file="other_dir/other_blueprint_config.toml"
```

### Single Packages

There two main ways to declare packages through the config the first is through declaring single packages under the ``[[package]]`` table.


---------------

#### ``name``

Name of the package to install from this source.

type: ``string``

```toml
[[package]]
name="linux"
```

------------------

#### ``source``

What package manager/source to use to download/install this package?

type: ``string``

**Valid Options**

``archlinux-paru``: Use the Paru AUR helper for arch linux as the command that installs the package. 

```toml
[[package]]
source="archlinux-paru"
```

### Multiple Packages

A shorthand exists to install many packages at once under the ``[multi_packages]`` table, this takes the same arguments as ``[[package]]`` but ``name`` is replaced by a ``names`` list, which takes in a list of strings as the packages instead.

<a name="license"></a>
## 🧾 License

This repository/``blueprint`` is under the MIT license. view it in the ``LICENSE`` file in the root directory of the repository.

<a name="acknowledgements"></a>
## 🎓 Acknowledgments

- Thanks to all of the crates used by ``blueprint``.
- Thank you for reading this README/Learning about blueprint! ❤️

<br>

-------------

[Blueprint Authored/Created by Stellaurora](https://github.com/stellaurora/blueprintf)

Love for everyone 💛 
