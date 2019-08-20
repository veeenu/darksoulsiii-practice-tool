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

```toml
[mappings]
  # pick values from https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
  show = "VK_F11"         # show/hide tool window
  quitout = "P"           # instant quitout
  save_pos = "VK_F7"      # save position
  load_pos = "VK_F1"      # load position
  # on/off toggles
  inf_stamina = "VK_F2"
  inf_focus = "VK_F3"
  inf_consum = "VK_F4"
  no_damage = "VK_F5"
  no_death = "VK_F6"
  deathcam = "VK_F8"
  one_shot = "VK_F9"
  no_gravity = "VK_F10"
  cycle_speed = "4"
  event_draw = "5"
  event_disable = "6"
  ai_disable = "7"
  rend_chr = "8"
  rend_map = "9"
  rend_obj = "0"
[settings]
  # you can ignore this section
  enabled = "true"
  debug = "false"
```

## To-do list and known issues

- Validity-check pointers: right now, flipping a value when the pointer is
  not well defined will lead to a crash.
- Live-syncing game memory and UI status

## Building

Development (artifacts in `build/RelWithDebInfo`):

```
$ python build.py
```

Production (artifacts in `build/Release`):

```
$ python build.py Release
```

## Writeup

I wrote a technical post about some inner workings of the mod.
You can read it [here](http://veeenu.github.io/2019/08/18/sekiro-practice-tool-architecture.html).