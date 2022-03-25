use std::path::PathBuf;

use rayon::prelude::*;

mod bmh;
mod read_module;

fn print_offset(bytes: &[u8], start: usize, len: usize) {
    for i in 0..len {
        print!("{:02x} ", bytes[start + i]);
    }
    println!("");
}

fn naive_search(bytes: &[u8], pattern: &[Option<u8>]) -> Option<usize> {
    bytes.windows(pattern.len()).position(|wnd| {
        wnd.iter()
            .zip(pattern.iter())
            .all(|(byte, pattern)| match pattern {
                Some(x) => byte == x,
                None => true,
            })
    })
}

fn find_aobs(bytes: Vec<u8>) -> Vec<(&'static &'static str, usize)> {
    let aob: &[(&str, &str)] = &[
        ("WorldChrMan", "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 0F 48 39 88 ?? ?? ?? ?? 75 06 89 B1 5C 03 00 00 0F 28 05 ?? ?? ?? ?? 4C 8D 45 E7"),
        ("MapItemMan", "48 8B 0D ?? ?? ?? ?? C7 44 24 50 FF FF FF FF C7 45 A0 FF FF FF FF 48 85 C9 75 2E"),
        ("CSFlipper", "48 8B 0D ?? ?? ?? ?? 80 BB D7 00 00 00 00 0F 84 CE 00 00 00 48 85 C9 75 2E"),
        ("CSMenuMan", "E8 ?? ?? ?? ?? 4C 8B F8 48 85 C0 0F 84 ?? ?? ?? ?? 48 8B 0D"),
        ("CSLuaEventManager", "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 ?? 41 BE 01 00 00 00 44 89 74"),
        ("MsgRepository", "48 8B 3D ?? ?? ?? ?? 44 0F B6 30 48 85 FF 75 26"),
        ("FieldArea", "48 8B 3D ?? ?? ?? ?? 48 85 FF 0F 84 ?? ?? ?? ?? 45 38 66 34"),
        ("CSNetMan", "48 8B 0D ?? ?? ?? ?? 48 85 C9 74 5E 48 8B 89 ?? ?? ?? ?? B2 01"),
        ("WorldChrManDbg", "48 8B 0D ?? ?? ?? ?? 89 5C 24 20 48 85 C9 74 12 B8 ?? ?? ?? ?? 8B D8"),
        ("SoloParamRepository", "48 8B 0D ?? ?? ?? ?? 48 85 C9 0F 84 ?? ?? ?? ?? 45 33 C0 BA 8D 00 00 00 E8"),
        ("CSSessionManager", "48 8B 05 ?? ?? ?? ?? 48 89 9C 24 E8 00 00 00 48 89 B4 24 B0 00 00 00 4C 89 A4 24 A8 00 00 00 4C 89 AC 24 A0 00 00 00 48 85 C0"),
        ("DamageCtrl", "48 8B 05 ?? ?? ?? ?? 49 8B D9 49 8B F8 48 8B F2 48 85 C0 75 2E"),
        ("CSRegulationManager", "48 8B 0D ?? ?? ?? ?? 48 85 C9 74 0B 4C 8B C0 48 8B D7"),
        ("CSFD4VirtualMemoryFlag", "48 8B 3D ?? ?? ?? ?? 48 85 FF 74 ?? 48 8B 49"),
        ("GameMan", "48 8B 15 ?? ?? ?? ?? 41 B0 01 48 8B 0D ?? ?? ?? ?? 48 81 C2 10 0E 00 00"),
        ("GameDataMan", "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 05 48 8B 40 58 C3 C3"),
        ("WorldChrManImp", "48 8b 05 ?? ?? ?? ?? 48 89 98 70 84 01 00 4c 89 ab 74 06 00 00 4c 89 ab 7c 06 00 00 44 88 ab 84 06 00 00 41 83 7f 4c 00"),
        ("MenuManIns", "48 8b 0d ?? ?? ?? ?? 48 8b 53 08 48 8b 92 d8 00 00 00 48 83 c4 20 5b"),
        ("CHR_DBG_FLAGS", "?? 80 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? 32 C0 48"),
    ];

    let mut aob_offsets = aob
        .into_par_iter()
        .filter_map(|(name, aob)| {
            if let Some(r) = naive_search(&bytes, &bmh::into_needle(aob)) {
                Some((name, r))
            } else {
                println!("{name:24} not found");
                None
            }
        })
        .map(|offset| {
            (
                offset.0,
                offset.1,
                u32::from_le_bytes(bytes[offset.1 + 3..offset.1 + 7].try_into().unwrap()),
            )
        })
        .map(|offset| (offset.0, (offset.2 + 7) as usize + offset.1))
        .collect::<Vec<_>>();

    aob_offsets.sort_by(|a, b| a.0.cmp(b.0));

    for (name, addr) in &aob_offsets {
        println!("{name:24} {:x}", addr);
    }
    aob_offsets
}
fn main() {
    let exes = [
        "exe\\eldenring-1.02.1.exe",
        "exe\\eldenring-1.02.2.exe",
        "exe\\eldenring-1.02.3.exe",
        "exe\\eldenring-1.03.0.exe",
        "exe\\eldenring-1.03.1.exe",
        "exe\\eldenring-1.03.2.exe",
    ];

    for exe in exes {
        println!("\n{}\n", exe);
        let exe = PathBuf::from(exe).canonicalize().unwrap();

        if let Some((base_addr, bytes)) = read_module::run_exe_and_get_base_module_bytes(&exe) {
            println!("Base addr {:x}", base_addr);
            let mem_aobs = find_aobs(bytes);
        }
    }
}

/*
 * IGT              GameDataMan, A0
 * Debug Sphere     DamageCtrl, A0
 *                  DamageCtrl, A1
 * Damipoli         DamageCtrl, A4
 * PlayerIns is at offset 0x18468 in WorldChrManImp
 * Position         WorldChrManImp, 18468, F68 (old patch)
 */

/*
CSFD4VirtualMemoryFlag   3c672a8
CSFlipper                4487908
CSLuaEventManager        3c66cb8
CSMenuMan                8ba63d14
CSNetMan                 3c59d20
CSRegulationManager      3c84e38
CSSessionManager         3c78900
DamageCtrl               3c65228
FieldArea                3c68040
GameDataMan              3c5cd78
GameMan                  3c68758
MapItemMan               3c668c0
MenuManIns               3c6a700
MsgRepository            3c7b928
SoloParamRepository      3c80158
WorldChrMan              3c64e38
WorldChrManDbg           3c65048
WorldChrManImp           3c64e38
*/

/* Flags
 
 CHR_DBG_FLAGS
   Player Exterminate:      3
   All No Goods Consume:    4
   All No Stam  Consume:    5
   All No FP    Consume:    6
   All No Arrow Consume:    7
   All No Dead:             B
   All No Damage:           C
   All No Hit:              D
   All No Attack:           E
   All No Move:             F
   All No Update AI:       10 
   All no AOW FP Consume:  12
 
 */
