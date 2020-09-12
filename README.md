# darksoulsiii-practice-tool

A tool intended for practicing speedruns. It is compatible with all the major
speedrunning-related patches: 1.04 (Any%), 1.08 (Any% NTD), 1.12 (All Bosses)
and 1.15 (current patch).

The project is comprised of a `.dll` file, an `.exe` file and an optional
configuration file (see next section). To run it, just start the game and
double-click the`.exe` file. The tool window will automatically appear,
and it can be toggled by pressing `F11`.

You can download the latest release [here](https://github.com/veeenu/darksoulsiii-practice-tool/releases).

If you are in need of help, please feel free to contact
`johndisandonato#4484` on Discord or submit an issue here on GitHub.

## Settings

Settings are stored in a file named `jdsd_dsiii_practice_tool.toml` in the
same directory as the tool. The file, if not present, is automatically filled
with default values. Any errors coming from wrong syntax or undefined fields
may be fixed by simply removing the file.

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

## To-do list and known issues

- "Increase souls" pointer for every version but 1.08

## Building

Production (artifacts in `target/release`):

```
$ cargo build --release
```

## Writeup

I wrote a technical post about some inner workings of the mod.
You can read it [here](http://veeenu.github.io/2019/08/18/sekiro-practice-tool-architecture.html).
