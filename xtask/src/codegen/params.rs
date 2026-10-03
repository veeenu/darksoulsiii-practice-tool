use std::collections::HashMap;
use std::fmt::Write;
use std::fs;

use anyhow::Context;
use practice_tool_tasks::params::{checkout_paramdex, codegen_param_data, codegen_param_names};

use crate::{project_root, Result};

/// The game's param names, in the order they're generated.
const PARAM_NAMES: &str = include_str!("param_names.txt");

/// Params whose layout isn't named after them, and the layout they use.
const LAYOUT_ALIASES: &[(&str, &str)] = &[
    ("AtkParam_Npc", "ATK_PARAM"),
    ("AtkParam_Pc", "ATK_PARAM"),
    ("BehaviorParam_PC", "BEHAVIOR_PARAM"),
    ("Bullet", "BULLET_PARAM"),
    ("CalcCorrectGraph", "CACL_CORRECT_GRAPH"),
    ("Ceremony", "CEREMONY_PARAM"),
    ("CharaInitParam", "CHARACTER_INIT_PARAM"),
    ("HPEstusFlaskRecoveryParam", "ESTUS_FLASK_RECOVERY_PARAM"),
    ("LodParam", "LOD_BANK"),
    ("LodParam_ps4", "LOD_BANK"),
    ("LodParam_xb1", "LOD_BANK"),
    ("Magic", "MAGIC_PARAM"),
    ("MenuPropertyLayoutParam", "MENUPROPERTY_LAYOUT"),
    ("MenuPropertySpecParam", "MENUPROPERTY_SPEC"),
    ("MenuValueTableParam", "MENU_VALUE_TABLE_SPEC"),
    ("MPEstusFlaskRecoveryParam", "ESTUS_FLASK_RECOVERY_PARAM"),
    ("MultiHPEstusFlaskBonusParam", "MULTI_ESTUS_FLASK_BONUS_PARAM"),
    ("MultiMPEstusFlaskBonusParam", "MULTI_ESTUS_FLASK_BONUS_PARAM"),
    ("NewMenuColorTableParam", "MENU_PARAM_COLOR_TABLE"),
    ("ThrowParam", "THROW_INFO_BANK"),
    ("Wind", "WIND_PARAM"),
];

const VTABLE_START: &str = "    [\n";
const VTABLE_END: &str = "    ].into_iter().collect()\n});";
const STRUCT_START: &str = "\n#[derive(ParamStruct, Debug)]\n#[repr(C)]\npub struct ";

pub(crate) fn codegen() -> Result<()> {
    checkout_paramdex()?;

    let param_data = codegen_param_data(&project_root().join("target/Paramdex"), "DS3")?;
    fs::write(
        project_root().join("lib/libds3/src/params/param_data.rs"),
        name_after_params(&param_data)?,
    )?;

    codegen_param_names("target/Paramdex/DS3/Names", "lib/libds3/src/params/param_names.json")?;

    Ok(())
}

/// Names the generated structs after the game's params, which the tool looks
/// them up by, instead of after their layouts. A layout used by several params
/// is emitted once per param.
fn name_after_params(param_data: &str) -> Result<String> {
    let (header, rest) = param_data.split_once(VTABLE_START).context("No PARAM_VTABLE")?;
    let (_, structs) = rest.split_once(VTABLE_END).context("No PARAM_VTABLE end")?;

    // Struct bodies, from the opening brace, by layout.
    let bodies = structs
        .split(STRUCT_START)
        .skip(1)
        .map(|s| s.split_once(' ').map(|(layout, body)| (slug(layout), body)))
        .collect::<Option<HashMap<_, _>>>()
        .context("Malformed struct")?;

    let params = PARAM_NAMES
        .lines()
        .map(|param| {
            let layout = LAYOUT_ALIASES.iter().find(|(p, _)| *p == param).map_or(param, |(_, l)| l);
            let body =
                bodies.get(&slug(layout)).with_context(|| format!("No layout for {param}"))?;
            Ok((param, body))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut source = format!("{header}{VTABLE_START}");
    for (param, _) in &params {
        writeln!(
            source,
            "        (\"{param}\".to_string(), unsafe {{ get_lambda::<{param}>() }}),"
        )?;
    }
    source.push_str(VTABLE_END);
    for (param, body) in &params {
        write!(source, "{STRUCT_START}{param} {body}")?;
    }

    Ok(source)
}

fn slug(name: &str) -> String {
    name.chars().filter(char::is_ascii_alphabetic).collect::<String>().to_ascii_lowercase()
}
