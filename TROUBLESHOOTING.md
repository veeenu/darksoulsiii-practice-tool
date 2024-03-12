# Troubleshooting / FAQ

## Is this safe to run?

The tool employs some techniques that are usually typical of malware, such as process manipulation,
DLL injection and detouring. It needs to do so to interact with the Elden Ring process.

As a result, antiviruses and scans will raise all sorts of red flags.

Don't worry, the tool is completely safe! The source code is fully available and auditable in this repository.

Getting a code signing certificate could mitigate this, but it is very expensive. Please
consider [supporting the project](https://www.patreon.com/johndisandonato).

If you don't want to trust the published binaries, you can [compile the tool from sources](CONTRIBUTING.md)
yourself.

## I found a bug. What do I do?

- Set the `log_level = "DEBUG"` option in `jdsd_dsiii_practice_tool.toml`.
- Reproduce the steps that cause your bug.
- Go
  [here](https://github.com/veeenu/darksoulsiii-practice-tool/issues/new)
  and submit a new issue, explaining the problem and attaching the
  `jdsd_dsiii_practice_tool.log` file.

I'll do my best to get back to you and fix the bug.

## How can I change the key bindings?

You can customize the default ones or add your own by editing
`jdsd_dsiii_practice_tool.toml` with your favorite text editor.

The bundled file contains all possible settings with predefined hotkeys and is mostly
self-explanatory.

You can find a list of supported hotkey codes [here](https://github.com/veeenu/practice-tool-core/blob/2960d851005ca0edaf030472cdddd3c992f077f9/src/key.rs#L7-L151).

Valid combinations are:
- Individual keys: `"tab"`, `"left"`
- Keys with up to 3 modifiers, separated by `+`: `"ctrl+x"`, `"alt+1"`, `"ctrl+rshift+alt+q"`.

  Valid modifiers are:
  - `ctrl`, `shift`, `alt`, `super` (bilateral)
  - `lctrl`, `lshift`, `lalt`, `lsuper` (left variant)
  - `rctrl`, `rshift`, `ralt`, `rsuper` (right variant)

## What versions of the game are supported?

All of them! When new patches come out, a new release with compatibility will be drafted as soon as possible.

## Will I get banned if I use this online?

Use at your own risk. Bans are unlikely, but in doubt, make backups of your savefiles and only use the tool offline.
By using the tool, you agree that I will not be held liable for any bans or unintended side effects resulting from the usage of the tool.

## I want to talk to you!

You can contact me on [my Discord server](https://discord.gg/jCVjxjHZ).
Please use the [Practice Tool help channel](https://discord.com/channels/267623298647457802/996101875214585867)
if you have questions about the Practice Tool.

## I want to watch your speedruns!

Sure! See you over here 👉 [https://twitch.tv/johndisandonato](https://twitch.tv/johndisandonato)!

