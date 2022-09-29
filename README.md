# Dark Souls III Practice Tool

[![build](https://github.com/veeenu/darksoulsiii-practice-tool/actions/workflows/build.yml/badge.svg)](https://github.com/veeenu/darksoulsiii-practice-tool/actions)
[![GitHub all releases](https://img.shields.io/github/downloads/veeenu/darksoulsiii-practice-tool/total)](https://github.com/veeenu/darksoulsiii-practice-tool/releases/latest)
[![GitHub](https://img.shields.io/github/license/veeenu/darksoulsiii-practice-tool)](https://github.com/veeenu/darksoulsiii-practice-tool/blob/main/LICENSE) 
[![Discord](https://img.shields.io/discord/267623298647457802)](https://discord.gg/CVHbN7eF)
[![Twitch](https://img.shields.io/twitch/status/johndisandonato?style=social)](https://twitch.tv/johndisandonato)

A tool for practicing speedruns. It is compatible with all Dark Souls III patches.

Made with ❤️ by [johndisandonato](https://twitch.tv/johndisandonato).

To run the tool, extract all files from the zip archive and double-click the
`.exe` file he tool will automatically appear over the game, and it can be
toggled by pressing `0`.

You can download the **latest stable release** [here](https://github.com/veeenu/eldenring-practice-tool/releases/latest).

If you need help, **please first read** the [Known Issues](#known-issues) and [FAQ](#troubleshooting--faq) sections for
solutions, or ways to get in touch.


## Troubleshooting / FAQ

### Where are all the key bindings?

You can customize the default ones or add your own by editing
`jdsd_dsiii_practice_tool.toml` with your favorite text editor.

The bundled file contains all possible settings with predefined hotkeys and is mostly
self-explanatory.

You can find a list of supported hotkey codes [here](https://github.com/veeenu/darksoulsiii-practice-tool/blob/7aa6ac33c6f155d35d0fa99ab100c8caa13913f9/practice-tool/src/util/vk.rs#L15-L186).

## What versions of the game are supported?

All of them! When new patches come out, a new release with compatibility will be drafted as soon as possible.

## Will I get banned if I use this online?

Use at your own risk. Bans are unlikely, but in doubt, make backups of your savefiles and only use the tool offline.
By using the tool, you agree that I will not be held liable for any bans or unintended side effects resulting from the usage of the tool.

### I found a bug. What do I do?

- Set the `log_level = "DEBUG"` option in `jdsd_dsiii_practice_tool.toml`.
- Reproduce the steps that cause your bug.
- Go
  [here](https://github.com/veeenu/darksoulsiii-practice-tool/issues/new)
  and submit a new issue, explaining the problem and attaching the
  `jdsd_dsiii_practice_tool.log` file.

I'll do my best to get back to you and fix the bug.

### I want to talk to you!

You can contact me on [my Discord server](https://discord.gg/jCVjxjHZ).
Please use the [Practice Tool help channel](https://discord.com/channels/267623298647457802/996101875214585867)
if you have questions about the Practice Tool.

## Credits

- The Cheat Engine table maintained by [The Grand Archives](https://github.com/inunorii/Dark-Souls-III-CT-TGA)
  provided the research base for many of the pointers used in the tool.
- NamelessHoodie[2] and Amir's work on [HoodieScript](https://github.com/NamelessHoodie/HoodieScript)
  for insights about the game's inner workings.
- The Soulsmodding community for the [Param definitions](https://github.com/soulsmods/Paramdex).
- r3sus for the help with anti-cheat ideas, for all the interesting study material / code and
  all the general tips.

## Development

You will need:

- A recent [Rust nightly](https://rustup.rs/)
- The [MSVC toolchain](https://visualstudio.microsoft.com/vs/features/cplusplus/)
- Python >=3.9 with `pandas`

Most building functions are exposed by the [xtasks](https://github.com/matklad/cargo-xtask).

## Run the tool

```
cargo xtask run
```

This task will compile and run the practice tool from the repo.

## Distribution artifacts

```
cargo xtask dist
```

This task will create release artifacts in `target/dist/jdsd_dsiii_practice_tool.zip`.

## Code generation

```
cargo xtask codegen
```

This task is responsible for generating Rust code from various external sources.
Examples: params from [Paramdex](https://github.com/soulsmods/Paramdex), base pointers for
array-of-byte scans from the Dark Souls III executables.

## Writeup

I wrote a technical post about some inner workings of the mod.
You can read it [here](https://veeenu.github.io/blog/sekiro-practice-tool-architecture/).
