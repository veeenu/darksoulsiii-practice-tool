use std::cmp::PartialOrd;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use heck::AsSnakeCase;
use once_cell::sync::Lazy;
use pelite::{FileMap, PeFile};
use rayon::prelude::*;
use textwrap::dedent;

const AOBS: &[(&str, &str, usize, usize)] = &[
    ("WorldChrMan", "48 8B 1D ?? ?? ?? 04 48 8B F9 48 85 DB ?? ?? 8B 11 85 D2 ?? ?? 8D", 3, 7),
    ("WorldChrManDbg", "48 8B 05 ?? ?? ?? ?? 66 0F 7F 44 24 40 48 85 C0", 3, 7),
    ("MenuMan", "48 89 15 ?? ?? ?? ?? 44 8b 82 ?? ?? ?? ?? 44 8b 8a ?? ?? ?? ?? 48 8b c3", 3, 7),
    ("BaseA", "48 8B 05 ?? ?? ?? ?? 48 85 C0 ?? ?? 48 8b 40 ?? C3", 3, 7),
    ("BaseD", "48 8B 0D ?? ?? ?? ?? 48 85 C9 74 26 44 8B", 3, 7),
    ("SprjDebugEvent", "48 8B 05 ?? ?? ?? ?? 41 0F B6 D8 8B EA", 3, 7),
    (
        "Debug",
        "C6 05 ?? ?? ?? ?? 01 48 8B 8C 24 ?? ?? ?? ?? 48 33 CC E8 ?? ?? ?? ?? 4C 8D 9C 24",
        2,
        7,
    ),
    (
        "Grend",
        "C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? 00 4C \
         8B 05 ?? ?? ?? ?? 4C 89 44 24 58",
        2,
        7,
    ),
    ("BaseHBD", "48 8B 0D ?? ?? ?? ?? 41 B0 01 E8 ?? ?? ?? ?? 48 8B D3 48 8B CF", 3, 7),
    ("MapItemMan", "48 8B 0D ?? ?? ?? ?? 48 8B 89 ?? ?? ?? ?? E8 ?? ?? ?? ?? E9", 3, 7),
    (
        "SpawnItemFuncPtr",
        // "E8 ?? ?? ?? ?? C7 44 24 20 00 01 00 00 4C 8D 4C 24 40 41 B8 2C 00 00 00 48 8B D3",
        "E8 ?? ?? ?? ?? C7 44 24 20 00 01 00 00 4C 8D 4C 24 40 41 B8",
        1,
        5,
    ),
    ("Param", "48 8B 0D ?? ?? ?? ?? 48 85 C9 74 0B 4C 8B C0 48 8B D7", 3, 7),
];

static AOBS_README: Lazy<Vec<(&str, Vec<&str>, usize)>> =
    Lazy::new(|| vec![("XA", vec!["48 8B 83 ?? ?? ?? ?? 48 8B 10 48 85 D2 ?? ?? 8B"], 3)]);

static AOBS_DIRECT: Lazy<Vec<(&str, Vec<&str>)>> = Lazy::new(|| {
    vec![
        ("FormatString", vec!["3C 00 54 00 45 00 58 00 54 00 46 00 4F 00 52 00 4D 00 41 00 54 00"]),
        ("NoLogo", vec!["E8 ?? ?? ?? FF 90 4D 8B C7 49 8B D4 48 8B C8 E8 ?? ?? ?? FF"]),
        ("CurrentTarget", vec!["48 8B 80 ?? ?? ?? ?? 48 8B 08 48 8B ?? 58"]),
        (
            "MenuTravel",
            vec![
                "40 55 53 56 57 41 56 48 8D 6C 24 C9 48 81 EC 00 01 00 00 48 C7 45 97 FE FF FF FF",
            ],
        ),
        (
            "MenuAttune",
            vec![
            "48 8D 45 0F 48 89 45 EF 48 8D 45 0F 48 89 45 F7 48 8D ?? ?? ?? ?? ?? 48 89 45 0F 48 \
             8D ?? ?? ?? ?? ?? 48 89 45 0F 48 8D ?? ?? ?? ?? ?? 48 89 45 17",
            ],
        ),
    ]
});

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Version(u32, u32, u32);

impl Version {
    fn to_fromsoft_string(self) -> String {
        format!("{}.{:02}.{}", self.0, self.1, self.2)
    }
}

struct VersionData {
    version: Version,
    aobs: Vec<(&'static str, usize)>,
}

fn into_needle(pattern: &str) -> Vec<Option<u8>> {
    pattern
        .split(' ')
        .map(|byte| match byte {
            "?" | "??" => None,
            x => u8::from_str_radix(x, 16).ok(),
        })
        .collect::<Vec<_>>()
}

fn naive_search(bytes: &[u8], pattern: &[Option<u8>]) -> Option<usize> {
    bytes.windows(pattern.len()).position(|wnd| {
        wnd.iter().zip(pattern.iter()).all(|(byte, pattern)| match pattern {
            Some(x) => byte == x,
            None => true,
        })
    })
}

fn find_aob_normal(
    name: &'static str,
    aob: &str,
    offset_read: usize,
    offset_final: usize,
    pe_file: &PeFile,
) -> Option<(&'static str, usize)> {
    let needle = into_needle(aob);

    pe_file
        .section_headers()
        .into_iter()
        .filter_map(|sh| Some((sh.VirtualAddress as usize, pe_file.get_section_bytes(sh).ok()?)))
        .find_map(|(base, bytes)| {
            let offset = naive_search(bytes, &needle)?;
            let val = u32::from_le_bytes(
                bytes[offset + offset_read..offset + offset_read + 4].try_into().unwrap(),
            ) as usize;
            let addr = val + offset_final + offset + base;

            Some((name, addr))
        })
}

fn find_aob_direct(
    name: &'static str,
    aob: &str,
    pe_file: &PeFile,
) -> Option<(&'static str, usize)> {
    let needle = into_needle(aob);

    pe_file
        .section_headers()
        .into_iter()
        .filter_map(|sh| Some((sh.VirtualAddress as usize, pe_file.get_section_bytes(sh).ok()?)))
        .find_map(|(base, bytes)| naive_search(bytes, &needle).map(|r| (name, r + base)))
}

fn find_aob_readme(
    name: &'static str,
    aob: &str,
    offset: usize,
    pe_file: &PeFile,
) -> Option<(&'static str, usize)> {
    let needle = into_needle(aob);

    pe_file
        .section_headers()
        .into_iter()
        .filter_map(|sh| pe_file.get_section_bytes(sh).ok())
        .find_map(|bytes| {
            naive_search(bytes, &needle).map(|r| {
                let r =
                    u32::from_le_bytes((&bytes[r + offset..r + offset + 4]).try_into().unwrap());
                (name, r as usize)
            })
        })
}

fn find_aobs(pe_file: &PeFile) -> Vec<(&'static str, usize)> {
    let mut aob_offsets = AOBS
        .into_par_iter()
        .filter_map(|(name, aob, offset_read, offset_final)| {
            find_aob_normal(name, aob, *offset_read, *offset_final, pe_file)
        })
        .collect::<Vec<_>>();

    aob_offsets.sort_by(|a, b| a.0.cmp(b.0));

    //
    let mut aob_offsets_direct = AOBS_DIRECT
        .iter()
        .filter_map(|(name, aob)| aob.iter().find_map(|aob| find_aob_direct(name, aob, pe_file)))
        .collect::<Vec<_>>();

    aob_offsets_direct.sort_by(|a, b| a.0.cmp(b.0));

    aob_offsets.extend(aob_offsets_direct);

    //
    let mut aob_offsets_readme = AOBS_README
        .iter()
        .filter_map(|(name, aob, offset)| {
            aob.iter().find_map(|aob| find_aob_readme(name, aob, *offset, pe_file))
        })
        .collect::<Vec<_>>();

    aob_offsets_readme.sort_by(|a, b| a.0.cmp(b.0));

    aob_offsets.extend(aob_offsets_readme);

    aob_offsets
}

// Codegen routine
//

/// Generate the `BaseAddresses` struct.
fn codegen_base_addresses_struct() -> String {
    let mut generated = String::new();

    generated.push_str("// **********************************\n");
    generated.push_str("// *** AUTOGENERATED, DO NOT EDIT ***\n");
    generated.push_str("// **********************************\n");

    generated.push_str("#[derive(Debug)]\n");
    generated.push_str("pub struct BaseAddresses {\n");
    generated.push_str(
        &AOBS
            .iter()
            .map(|(name, ..)| format!("    pub {}: usize,\n", AsSnakeCase(name)))
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str({
        &AOBS_DIRECT
            .iter()
            .map(|(name, _)| format!("    pub {}: usize,\n", AsSnakeCase(name)))
            .collect::<Vec<_>>()
            .join("")
    });
    generated.push_str({
        &AOBS_README
            .iter()
            .map(|(name, ..)| format!("    pub {}: usize,\n", AsSnakeCase(name)))
            .collect::<Vec<_>>()
            .join("")
    });
    generated.push_str("}\n\n");
    generated.push_str("impl BaseAddresses {\n");
    generated.push_str("    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {\n");
    generated.push_str("        BaseAddresses {\n");
    generated.push_str(
        &AOBS
            .iter()
            .map(|(name, ..)| {
                format!("            {}: self.{} + base,\n", AsSnakeCase(name), AsSnakeCase(name))
            })
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str(
        &AOBS_DIRECT
            .iter()
            .map(|(name, _)| {
                format!("            {}: self.{} + base,\n", AsSnakeCase(name), AsSnakeCase(name))
            })
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str(
        &AOBS_README
            .iter()
            .map(|(name, ..)| {
                format!("            {}: self.{},\n", AsSnakeCase(name), AsSnakeCase(name))
            })
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str("        }\n    }\n}\n\n");
    generated
}

/// Generate `BaseAddresses` instances.
fn codegen_base_addresses_instances(ver: &Version, aobs: &[(&str, usize)]) -> String {
    use std::fmt::Write;
    let mut string = aobs.iter().fold(
        format!(
            "pub const BASE_ADDRESSES_{}_{:02}_{}: BaseAddresses = BaseAddresses {{\n",
            ver.0, ver.1, ver.2
        ),
        |mut o, (name, offset)| {
            writeln!(o, "    {}: 0x{:x},", AsSnakeCase(name), offset).unwrap();
            o
        },
    );
    string.push_str("};\n\n");
    string
}

/// Generate the `Version` enum and `From<Version> for BaseAddresses`.
fn codegen_version_enum(ver: &[VersionData]) -> String {
    use std::fmt::Write;
    let mut string = String::new();

    // pub enum Version

    string.push_str("#[derive(Clone, Copy)]\n");
    string.push_str("pub enum Version {\n");

    for v in ver {
        writeln!(string, "    V{}_{:02}_{},", v.version.0, v.version.1, v.version.2).unwrap();
    }

    string.push_str("}\n\n");

    // impl From<(u32, u32, u32)> for Version

    string.push_str("impl From<(u32, u32, u32)> for Version {\n");
    string.push_str("    fn from(v: (u32, u32, u32)) -> Self {\n");
    string.push_str("        match v {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        writeln!(
            string,
            "            ({maj}, {min}, {patch}) => Version::V{maj}_{min:02}_{patch},"
        )
        .unwrap();
    }

    string.push_str("            (maj, min, patch) => {\n");
    string.push_str(
        "                log::error!(\"Unrecognized version {maj}.{min:02}.{patch}\");\n",
    );
    string.push_str("                panic!()\n");
    string.push_str("            }\n");
    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    // impl From<Version> for (u32, u32, u32)

    string.push_str("impl From<Version> for (u32, u32, u32) {\n");
    string.push_str("    fn from(v: Version) -> Self {\n");
    string.push_str("        match v {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        writeln!(
            string,
            "            Version::V{maj}_{min:02}_{patch} => ({maj}, {min}, {patch}),"
        )
        .unwrap();
    }

    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    // impl From<Version> for BaseAddresses

    string.push_str("impl From<Version> for BaseAddresses {\n");
    string.push_str("    fn from(v: Version) -> Self {\n");
    string.push_str("        match v {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        let stem = format!("{maj}_{min:02}_{patch}");
        writeln!(string, "            Version::V{stem} => BASE_ADDRESSES_{stem},").unwrap();
    }

    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    string
}

fn patches_paths() -> impl Iterator<Item = PathBuf> {
    let base_path = PathBuf::from(
        env::var("DSIIIPT_PATCHES_PATH").unwrap_or_else(|_| panic!("{}", dedent(r"
            DSIIIPT_PATCHES_PATH environment variable undefined.
            Check the documentation: https://github.com/veeenu/darksoulsiii-practice-tool/README.md#building
        "))),
    );
    base_path
        .read_dir()
        .expect("Couldn't scan patches directory")
        .map(Result::unwrap)
        .map(|dir| dir.path().join("Game").join("DarkSoulsIII.exe"))
}

fn codegen_base_addresses_path() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf()
        .join("lib")
        .join("libds3")
        .join("src")
        .join("codegen")
        .join("base_addresses.rs")
}

pub fn codegen_base_addresses() {
    let mut processed_versions: HashSet<Version> = HashSet::new();

    let mut version_data = patches_paths()
        .filter(|p| p.exists())
        .filter_map(|exe| {
            let file_map = FileMap::open(&exe).unwrap();
            let pe_file = PeFile::from_bytes(&file_map).unwrap();

            let version = pe_file
                .resources()
                .unwrap()
                .version_info()
                .unwrap()
                .fixed()
                .unwrap()
                .dwProductVersion;
            let version = Version(version.Major as u32, version.Minor as u32, version.Patch as u32);

            if processed_versions.contains(&version) {
                None
            } else {
                let exe = exe.canonicalize().unwrap();
                println!("\nVERSION {}: {:?}", version.to_fromsoft_string(), exe);

                let aobs = find_aobs(&pe_file);
                processed_versions.insert(version);
                Some(VersionData { version, aobs })
            }
        })
        .collect::<Vec<_>>();

    version_data.sort_by_key(|vd| vd.version);

    let mut codegen = codegen_base_addresses_struct();
    codegen.push_str(&codegen_version_enum(&version_data));

    let codegen = version_data.iter().fold(codegen, |mut o, i| {
        o.push_str(&codegen_base_addresses_instances(&i.version, &i.aobs));
        o
    });

    File::create(codegen_base_addresses_path()).unwrap().write_all(codegen.as_bytes()).unwrap();
}
