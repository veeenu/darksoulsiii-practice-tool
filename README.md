# darksoulsiii-practice-tool

A tool intended for practicing speedruns. It is compatible with all the major
speedrunning-related patches: 1.04 (Any%), 1.08 (Any% NTD), 1.12 (All Bosses)
and 1.15 (current patch).

The project is comprised of a `.dll` file, an `.exe` file and an optional
configuration file (see next section). To run it, just start the game and
double-click the`.exe` file. The tool window will automatically appear,
and it can be toggled by pressing `F1`.

You can download the latest release [here](https://github.com/veeenu/darksoulsiii-practice-tool/releases).

If you are in need of help, please read the [FAQ](#troubleshooting--faq) section
for potential fixes or ways to get in touch.

## Settings

Settings are stored in a file named `jdsd_dsiii_practice_tool.toml` in the
same directory as the tool. If the file is not present, a default configuration
(see the code block below) is loaded.

Starting from this release, hotkeys are optional and there is a navigable
interface.

```toml
[mappings]
# Pick key values from https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
# These four are set by default:

interact = "I"     # Toggle current command
next = "J"         # Go to next command
prev = "K"         # Go to prev command
display = "VK_F1"  # Show/hide tool window

# List of available commands to map:

# all_no_damage
# no_death
# one_shot
# inf_stamina
# inf_focus
# inf_consumables
# save_position
# load_position
# souls
# quitout
# deathcam
# evt_draw
# evt_disable
# ai_disable
# rend_chr
# rend_map
# rend_obj
# gravity

[settings]
# Possible values from most to least verbose:
# trace, debug, info, warn, error
# trace and debug will also log to a console window.
log_level = "info"
```

## Troubleshooting / FAQ

### How does it work?

In a nutshell:

- Start the game, then start the tool.
- **Turn the interface on/off** with `F1`. The tool keeps working even when hidden.
- **Move down/up** the list of commands with `J` / `K`.
- **Activate/deactivate** the highlighted command with `I`.

### Where are all the key bindings?

You can customize the default ones or add your own by editing
`jdsd_dsiii_practice_tool.toml` with an editor such as Notepad. Under the
`[mappings]` table, you can define a keybinding this way:

```toml
quitout = "P"
save_position = "VK_F7"
load_position = "VK_F2"
```

### I want the old keybindings!

Here's a configuration file with a default keybinding setup to get you up and running.

Copy and paste this code block in your `jdsd_dsiii_practice_tool.toml` file.

```toml
[mappings]
# Pick key values from https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
# These four are set by default:

interact = "I"     # Toggle current command
next = "J"         # Go to next command
prev = "K"         # Go to prev command
display = "VK_F11"  # Show/hide tool window

# List of available commands to map:

all_no_damage = "VK_F5"
no_death = "VK_F6"
one_shot = "VK_F9"
inf_stamina = "VK_F2"
inf_focus = "VK_F3"
inf_consumables = "VK_F4"
save_position = "VK_F7"
load_position = "VK_F1"
souls = "O"
quitout = "P"
deathcam = "VK_F8"
evt_draw = "5"
evt_disable = "6"
ai_disable = "7"
rend_chr = "8"
rend_map = "9"
rend_obj = "0"
gravity = "VK_F10"

[settings]
# Possible values from most to least verbose:
# trace, debug, info, warn, error
# trace and debug will also log to a console window.
log_level = "info"
```

### Q. I found a bug. What do I do?

- Set the `log_level = "debug"` option in `jdsd_dsiii_practice_tool.toml`.
- Reproduce the steps that cause your bug.
- Go
  [here](https://github.com/veeenu/darksoulsiii-practice-tool/issues/new)
  and submit a new issue, explaining the problem and attaching the
  `jdsd_dsiii_practice_tool.log` file.

I'll do my best to get back to you and fix the bug. You can also contact
me on Discord (`johndisandonato#4484`) but I'd prefer an issue to be filed.

## To-do list and known issues

- Update notifier on startup

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
You can read it [here](http://veeenu.github.io/2019/08/18/sekiro-practice-tool-architecture.html).
