use std::env;
use std::path::{Path, PathBuf};

use practice_tool_tasks::codegen::{self, aob_direct, aob_indirect, aob_indirect_twice};
use textwrap::dedent;

fn patches_paths() -> impl Iterator<Item = PathBuf> {
    let base_path = PathBuf::from(
        env::var("DSIII_PATCHES_PATH").unwrap_or_else(|_| panic!("{}", dedent(r"
            DSIII_PATCHES_PATH environment variable undefined.
            Check the documentation: https://github.com/veeenu/darksoulsiii-practice-tool/README.md#building
        "))),
    );
    base_path
        .read_dir()
        .expect("Couldn't scan patches directory")
        .map(Result::unwrap)
        .map(|dir| dir.path().join("Game").join("DarkSoulsIII.exe"))
}

fn base_addresses_rs_path() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
        .join("lib")
        .join("libds3")
        .join("src")
        .join("codegen")
        .join("base_addresses.rs")
}

pub fn get_base_addresses() {
    let aobs = &[
        aob_indirect_twice(
            "WorldChrMan",
            &["48 8B 1D ?? ?? ?? 04 48 8B F9 48 85 DB ?? ?? 8B 11 85 D2 ?? ?? 8D"],
            3,
            7,
        ),
        aob_indirect_twice(
            "WorldChrManDbg",
            &["48 8B 05 ?? ?? ?? ?? 66 0F 7F 44 24 40 48 85 C0"],
            3,
            7,
        ),
        aob_indirect_twice(
            "MenuMan",
            &["48 89 15 ?? ?? ?? ?? 44 8b 82 ?? ?? ?? ?? 44 8b 8a ?? ?? ?? ?? 48 8b c3"],
            3,
            7,
        ),
        aob_indirect_twice("BaseA", &["48 8B 05 ?? ?? ?? ?? 48 85 C0 ?? ?? 48 8b 40 ?? C3"], 3, 7),
        aob_indirect_twice("BaseD", &["48 8B 0D ?? ?? ?? ?? 48 85 C9 74 26 44 8B"], 3, 7),
        aob_indirect_twice("SprjDebugEvent", &["48 8B 05 ?? ?? ?? ?? 41 0F B6 D8 8B EA"], 3, 7),
        aob_indirect_twice(
            "Debug",
            &["C6 05 ?? ?? ?? ?? 01 48 8B 8C 24 ?? ?? ?? ?? 48 33 CC E8 ?? ?? ?? ?? 4C 8D 9C 24"],
            2,
            7,
        ),
        aob_indirect_twice(
            "Grend",
            &["C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? \
               00 4C 8B 05 ?? ?? ?? ?? 4C 89 44 24 58"],
            2,
            7,
        ),
        aob_indirect_twice(
            "BaseHBD",
            &["48 8B 0D ?? ?? ?? ?? 41 B0 01 E8 ?? ?? ?? ?? 48 8B D3 48 8B CF"],
            3,
            7,
        ),
        aob_indirect_twice(
            "MapItemMan",
            &["48 8B 0D ?? ?? ?? ?? 48 8B 89 ?? ?? ?? ?? E8 ?? ?? ?? ?? E9"],
            3,
            7,
        ),
        aob_indirect_twice(
            "SpawnItemFuncPtr",
            // "E8 ?? ?? ?? ?? C7 44 24 20 00 01 00 00 4C 8D 4C 24 40 41 B8 2C 00 00 00 48 8B D3",
            &["E8 ?? ?? ?? ?? C7 44 24 20 00 01 00 00 4C 8D 4C 24 40 41 B8"],
            1,
            5,
        ),
        aob_indirect_twice(
            "Param",
            &["48 8B 0D ?? ?? ?? ?? 48 85 C9 74 0B 4C 8B C0 48 8B D7"],
            3,
            7,
        ),
        aob_direct("FormatString", &[
            "3C 00 54 00 45 00 58 00 54 00 46 00 4F 00 52 00 4D 00 41 00 54 00"
        ]),
        aob_direct("NoLogo", &["E8 ?? ?? ?? FF 90 4D 8B C7 49 8B D4 48 8B C8 E8 ?? ?? ?? FF"]),
        aob_direct("CurrentTarget", &["48 8B 80 ?? ?? ?? ?? 48 8B 08 48 8B ?? 58"]),
        aob_direct("MenuTravel", &[
            "40 55 53 56 57 41 56 48 8D 6C 24 C9 48 81 EC 00 01 00 00 48 C7 45 97 FE FF FF FF"
        ]),
        aob_direct("MenuAttune", &["48 8D 45 0F 48 89 45 EF 48 8D 45 0F 48 89 45 F7 48 8D ?? ?? \
                                    ?? ?? ?? 48 89 45 0F 48 8D ?? ?? ?? ?? ?? 48 89 45 0F 48 8D \
                                    ?? ?? ?? ?? ?? 48 89 45 17"]),
        aob_indirect("XA", &["48 8B 83 ?? ?? ?? ?? 48 8B 10 48 85 D2 ?? ?? 8B"], 3),
    ];

    codegen::codegen_base_addresses(base_addresses_rs_path(), patches_paths(), aobs)
}
