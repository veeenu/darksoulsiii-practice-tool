# darksoulsiii-practice-tool

A tool intended for practicing speedruns. It is compatible with all the major
speedrunning-related patches: 1.04 (Any%), 1.08 (Any% NTD), 1.12 (All Bosses)
and 1.15 (current patch).

The project is comprised of a `.dll` file, an `.exe` file and an optional
configuration file (see next section). To run it, just start the game and
double-click the`.exe` file. The tool window will automatically appear,
and it can be toggled by pressing `0`.

You can download the latest release [here](https://github.com/veeenu/darksoulsiii-practice-tool/releases).

If you are in need of help, please read the [FAQ](#troubleshooting--faq) section
for potential fixes or ways to get in touch.

## Settings

Settings are stored in a file named `jdsd_dsiii_practice_tool.toml` in the same
directory as the tool It is bundled with the release zip file. If the file is
not present, a [default configuration](jdsd_dsiii_practice_tool.toml) is loaded.


## Troubleshooting / FAQ

### How does it work?

In a nutshell:

- Start the game, then start the tool.
- **Turn the interface on/off** with `F1`. The tool keeps working even when hidden.
- **Move down/up** the list of commands with `J` / `K`.
- **Activate/deactivate** the highlighted command with `I`.

### Where are all the key bindings?

You can customize the default ones or add your own by editing
`jdsd_dsiii_practice_tool.toml` with an editor such as Notepad, or using the
bundled `jdsd_dsiii_config_editor.exe` (the preferred way).

The bundled file contains all possible settings with predefined hotkeys. You can
either edit the hotkeys via the provided configuration tool, or manually edit
the `jdsd_dsiii_practice_tool.toml` file.

Most commands have a similar structure, but a few commands have extra configurations.
Refer to the default configuration file for the complete list of configurations
and commands. Here's a few examples:

```toml
[[command]]
cmd = "toggle"
flag = "gravity"
hotkey = "VK_F9"

[[command]]
cmd = "position"
hotkey_save = "J"
hotkey_load = "L"

[[command]]
cmd = "cycle_speed"
values = [1, 3]
hotkey = "8"

[[command]]
cmd = "souls"
quantity = 10000
hotkey = "9"
```

You can find a list of hotkey codes [here](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes).

### I found a bug. What do I do?

- Set the `log_level = "DEBUG"` option in `jdsd_dsiii_practice_tool.toml`.
- Reproduce the steps that cause your bug.
- Go
  [here](https://github.com/veeenu/darksoulsiii-practice-tool/issues/new)
  and submit a new issue, explaining the problem and attaching the
  `jdsd_dsiii_practice_tool.log` file.

I'll do my best to get back to you and fix the bug. You can also contact
me on Discord (`johndisandonato#4484`) but I'd prefer an issue to be filed.

## To-do list and known issues

- Investigate special characters in the DLL path preventing injection.
- Implement good UX for the item spawner.
- Implement a non-circumventable activity indicator (i.e. ingame font color).
- Implement custom keybindings for the savefile manager.

## Building

Production (artifacts in `target/release`):

```
$ cargo build --release
```

Package (creates a .zip file in the repo directory):

```
$ python package.py
```

## Writeup

I wrote a technical post about some inner workings of the mod.
You can read it [here](https://veeenu.github.io/blog/sekiro-practice-tool-architecture/).
