# darksoulsiii-practice-tool

A tool for practicing speedruns. It is compatible with the major speedrunning-related patches.

To run the tool, extract all files from the zip archive and double-click the `.exe` file.
The tool will automatically appear over the game, and it can be toggled by pressing `0`.

You can download the latest release [here](https://github.com/veeenu/darksoulsiii-practice-tool/releases).

If you need help, please read the [FAQ](#troubleshooting--faq) section for
solutions or ways to get in touch.

## Troubleshooting / FAQ

### Where are all the key bindings?

You can customize the default ones or add your own by editing
`jdsd_dsiii_practice_tool.toml` with your favorite text editor.

The bundled file contains all possible settings with predefined hotkeys and is mostly
self-explanatory.

You can find a list of supported hotkey codes [here](https://github.com/veeenu/darksoulsiii-practice-tool/blob/7aa6ac33c6f155d35d0fa99ab100c8caa13913f9/practice-tool/src/util/vk.rs#L15-L186).

### What versions of the game are supported?

| Version | Category |
| --- | --- |
| 1.04 | Any% |
| 1.08 | Any% NTD |
| 1.12 | All Bosses | 
| 1.15 | Current patch |

### I found a bug. What do I do?

- Set the `log_level = "DEBUG"` option in `jdsd_dsiii_practice_tool.toml`.
- Reproduce the steps that cause your bug.
- Go
  [here](https://github.com/veeenu/darksoulsiii-practice-tool/issues/new)
  and submit a new issue, explaining the problem and attaching the
  `jdsd_dsiii_practice_tool.log` file.

I'll do my best to get back to you and fix the bug.

### I want to talk to you!

You can contact me on Discord at `johndisandonato#4484` or on [Twitter](https://twitter.com/johndisandonato).

## Credits

- The Cheat Engine table maintained by [The Grand Archives](https://github.com/inunorii/Dark-Souls-III-CT-TGA)
  provided the research base for many of the pointers used in the tool.
- NamelessHoodie[2] and Amir's work on [HoodieScript](https://github.com/NamelessHoodie/HoodieScript)
  for insights about the game's inner workings.
- The Soulsmodding community for the [Param definitions](https://github.com/soulsmods/Paramdex).

## Development

All you need for building the project is a nightly version of Rust.

```
cargo +nightly build --release
```

In order to create a release package, run the bundled Python build script.
This will create a `jdsd_dsiii_practice_tool.zip` file in the project root.

```
python build.py package
```

## Writeup

I wrote a technical post about some inner workings of the mod.
You can read it [here](https://veeenu.github.io/blog/sekiro-practice-tool-architecture/).
