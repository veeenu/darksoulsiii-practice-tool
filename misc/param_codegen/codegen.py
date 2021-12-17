import pandas as pd
import re
from glob import glob
from pathlib import Path
from itertools import zip_longest
from textwrap import dedent


SNAKECASE_RE = re.compile(r'(?!^)([A-Z]+)')
SNAKECASE_CLEAN_RE = re.compile(r'_+')
SLUG_RE = re.compile(r'([^a-zA-Z]+)')

MEME_VTABLE_TEMPLATE = '''
pub static RENDER_VTABLE: SyncLazy<HashMap<String, Box<dyn Fn(*const c_void, &imgui::Ui) + Send + Sync>>> = SyncLazy::new(|| {{
    [ {vtable_fields}
    ].into_iter().collect()
}});
'''

MEME_VTABLE_FIELD_TEMPLATE = '''
        ("{param_name}".to_string(), unsafe {{ get_render_lambda::<{param_name}>() }}),'''

STRUCT_TEMPLATE = '''
    #[derive(Debug)]
    #[repr(C)]
    #[allow(non_camel_case_types)]
    pub struct {param_name} {{
        {fields}
    }}
'''

IMPL_STRUCT_TEMPLATE = '''
    impl {param_name} {{ {methods}
    }}
'''

IMPL_RENDERABLE_STRUCT_TEMPLATE = '''
    impl RenderableParam for {param_name} {{
        fn render_imgui(&mut self, ui: &imgui::Ui) {{ {fields_imgui}
        }}
    }}
'''

IMPL_PARAMS_TEMPLATE = '''
    impl Params {{{impl_params}    }}
'''

IMPL_PARAM_TEMPLATE = '''
        #[allow(unused)]
        pub unsafe fn get_{name_snake_case}(&self) -> Option<impl Iterator<Item = Param<{param_name}>>> {{
            self.iter_param::<{param_name}>("{param_name}")
        }}
'''

FIELD_TEMPLATE = '''
        pub {field_name}: {field_type},
'''.strip()

BITFIELD_TEMPLATE = '''
        #[allow(unused)]
        pub fn set_{flag_name}(&mut self, state: bool) {{
            const FIELD_INDEX: {field_type} = 1 << {field_index};
            let val = self.{bitfield_name};
            self.{bitfield_name} = if state {{
                val | FIELD_INDEX
            }} else {{
                val & !FIELD_INDEX
            }};
        }}

        #[allow(unused)]
        pub fn {flag_name}(&mut self) -> bool {{
            const FIELD_INDEX: {field_type} = 1 << {field_index};
            (self.{bitfield_name} & FIELD_INDEX) != 0
        }}
'''

def to_snake_case(s):
    return SNAKECASE_CLEAN_RE.sub('_', SNAKECASE_RE.sub(r'_\1', s).lower())


def to_slug(s):
    return SLUG_RE.sub('', s).lower()


HERE = (Path(__file__) / '..').resolve()


def build_param_layouts():
    # Param layouts -- Credits: Soulsmodding community's Paramdex
    xml_files = dict(
        (to_slug(Path(i).stem.replace('_ST', '')), Path(i).resolve())
        for i in (HERE / 'Paramdex/DS3/Defs').glob('*.xml')
    )

    # Param names from the game's memory
    param_names = dict(
        (to_slug(i), i)
        for i in (HERE / 'param_names.txt').read_text().splitlines()
    )

    # Manual corrections
    xml_files['atkparampc'] = xml_files['atkparam']
    xml_files['atkparamnpc'] = xml_files['atkparam']
    xml_files['behaviorparampc'] = xml_files['behaviorparam']
    xml_files['bullet'] = xml_files['bulletparam']
    xml_files['ceremony'] = xml_files['ceremonyparam']
    xml_files['charainitparam'] = xml_files['characterinitparam']
    xml_files['calccorrectgraph'] = xml_files['caclcorrectgraph']
    xml_files['hpestusflaskrecoveryparam'] = xml_files['estusflaskrecoveryparam']
    xml_files['mpestusflaskrecoveryparam'] = xml_files['estusflaskrecoveryparam']
    xml_files['lodparam'] = xml_files['lodbank']
    xml_files['lodparamps'] = xml_files['lodbank']
    xml_files['lodparamxb'] = xml_files['lodbank']
    xml_files['magic'] = xml_files['magicparam']
    xml_files['menupropertylayoutparam'] = xml_files['menupropertylayout']
    xml_files['menupropertyspecparam'] = xml_files['menupropertyspec']
    xml_files['menuvaluetableparam'] = xml_files['menuvaluetablespec']
    xml_files['multihpestusflaskbonusparam'] = xml_files['multiestusflaskbonusparam']
    xml_files['multimpestusflaskbonusparam'] = xml_files['multiestusflaskbonusparam']
    xml_files['newmenucolortableparam'] = xml_files['menuparamcolortable']
    xml_files['throwparam'] = xml_files['throwinfobank']
    xml_files['wind'] = xml_files['windparam']

    del xml_files['atkparam']
    del xml_files['bulletparam']
    del xml_files['ceremonyparam']
    del xml_files['characterinitparam']
    del xml_files['caclcorrectgraph']
    del xml_files['estusflaskrecoveryparam']
    del xml_files['lodbank']
    del xml_files['magicparam']
    del xml_files['menupropertylayout']
    del xml_files['menupropertyspec']
    del xml_files['menuvaluetablespec']
    del xml_files['multiestusflaskbonusparam']
    del xml_files['menuparamcolortable']
    del xml_files['throwinfobank']
    del xml_files['windparam']

    assert(xml_files.keys() == param_names.keys())

    return [
        ParamLayout(name=param_names[i], layout=xml_files[i])
        for i in param_names.keys()
    ]


class ParamLayout:
    def __init__(self, name, layout):
        self.name = name
        self.name_snake_case = to_snake_case(name)
        self.fields = ParamLayout.dedup_fields(ParamLayout.group_bitfields([
            Field(i) for i in pd.read_xml(layout, xpath='./Fields/*')['Def']
        ]))

    def get_struct(self):
        fields = '\n        '.join(
            FIELD_TEMPLATE.format(field_name=ParamLayout.fix_name(to_snake_case(field.name)), field_type=field.type)
            for field in self.fields
        )
        return STRUCT_TEMPLATE.format(param_name=self.name, fields=fields)

    def get_impl_param(self):
        return IMPL_PARAM_TEMPLATE.format(param_name=self.name, name_snake_case=self.name_snake_case)

    def get_impls(self):
        impls = []
        for f in self.fields:
            for i in f.get_impls():
                impls.append(i)
        imgui_impls = self.get_imgui()

        if len(impls) > 0:
            return IMPL_STRUCT_TEMPLATE.format(
                param_name=self.name,
                methods=''.join(impls)
            ) + imgui_impls
        else:
            return imgui_impls

    def get_imgui(self):
        fields_imgui = ''.join(f.get_imgui() for f in self.fields)
        return IMPL_RENDERABLE_STRUCT_TEMPLATE.format(
            param_name=self.name,
            fields_imgui=fields_imgui, 
        )

    @staticmethod
    def fix_name(name: str):
        if name[0].isdigit():
            return 'field' + name

        if name == 'type':
            return 'r#type'

        return name

    @staticmethod
    def dedup_fields(fields):
        fieldset = set()
        idx = 0
        for f in fields:
            nsc = to_snake_case(f.name)
            if nsc in fieldset:
                f.rename(idx)
                idx += 1
            fieldset.add(nsc)
        return fields

    @staticmethod
    def group_bitfields(fields):
        grouped_fields = []
        bitfield = []
        bitfield_idx = 0
        for f in fields:
            if f.kind != 'bitfield':
                grouped_fields.append(f)
            else:
                bitfield.append(f)
                if len(bitfield) > 0 and len(bitfield) == bitfield[-1].size:
                    grouped_fields.append(Bitfield(bitfield_idx, bitfield[-1].type, bitfield))
                    bitfield = []
                    bitfield_idx += 1
        return grouped_fields


class Bitfield:
    def __init__(self, idx, dtype, fields):
        self.name = f'bitfield{idx}'
        self.type = dtype
        self.fields = list(enumerate(ParamLayout.dedup_fields(fields)))

    def get_impls(self):
        return [
            BITFIELD_TEMPLATE.format(
                bitfield_name=self.name,
                flag_name=ParamLayout.fix_name(to_snake_case(flag.name)),
                field_type=self.type,
                field_index=idx
            )
            for idx, flag in self.fields
        ]

    def get_imgui(self):
        return ''.join(
            '''
            let mut b: bool = self.{flag_name}();
            if ui.checkbox("{field_name}", &mut b) {{
                self.set_{flag_name}(b);
            }}
            '''.format(
                field_name=flag.name,
                flag_name=ParamLayout.fix_name(to_snake_case(flag.name))
            )
            for idx, flag in self.fields
        )

    def rename(self, idx):
        self.name = self.name + f'_{idx}'


class Field:
    def_array_re = re.compile(r'(\w+)\s+(\w+)\[(\d+)\]')
    def_bitfield_re = re.compile(r'(\w+)\s+(\w+):(\d+)')
    def_basic_re = re.compile(r'(\w+)\s+(\w+)')

    type_map = {
        's8': 'i8',
        'u8': 'u8',
        's16': 'i16',
        'u16': 'u16',
        's32': 'i32',
        'u32': 'u32',
        'f32': 'f32',
        'fixstr': 'u8',
        'fixstrW': 'u16',
        'dummy8': 'u8',
    }

    def __init__(self, definition):
        if matches := Field.def_array_re.match(definition):
            self.kind = 'array'
            self.name = matches.group(2)
            array_count = int(matches.group(3))
            dtype = Field.type_map.get(matches.group(1))
            self.type = f'[{dtype}; {array_count}]'
        elif matches := Field.def_bitfield_re.match(definition):
            self.kind = 'bitfield'
            self.name = matches.group(2)
            self.type = matches.group(1)
            if self.type == 'u8':
                self.size = 8
            elif self.type == 'u16':
                self.size = 16
            elif self.type == 'u32':
                self.size = 32
        elif matches := Field.def_basic_re.match(definition):
            self.kind = 'normal'
            self.name = matches.group(2)
            self.type = Field.type_map.get(matches.group(1))
        else:
            raise ValueError(f'Couldn\'t parse: {definition}')

    def get_imgui(self):
        if self.kind == 'normal':
            field_name = ParamLayout.fix_name(to_snake_case(self.name))
            if self.type in ('u8', 'u16', 'u32', 'i8', 'i16', 'i32'):
                return f'''
            let mut i: i32 = self.{field_name} as _;
            if ui.input_int("{self.name}", &mut i).build() {{
                self.{field_name} = i as _;
            }}
                '''
            elif self.type == 'f32':
                return f'''
            let mut i: f32 = self.{field_name};
            if ui.input_float("{self.name}", &mut i).build() {{
                self.{field_name} = i;
            }}
                '''
            else:
                return f'// unknown type for ui: {self.type}'
        else:
            return ''

    def get_impls(self):
        return []

    def rename(self, idx):
        self.name = self.name + f'_{idx}'
            

if __name__ == '__main__':
    layouts = build_param_layouts()
    # with open(HERE / '../../ccs-mod/src/params/param_data.rs', 'w') as fp:
    with open(HERE / '../../libds3/src/params/param_data.rs', 'w') as fp:
        fp.write('// **********************************\n')
        fp.write('// *** AUTOGENERATED, DO NOT EDIT ***\n')
        fp.write('// **********************************\n')
        fp.write('use super::*;\n')
        fp.write('use std::collections::HashMap;\n')
        fp.write('use std::lazy::SyncLazy;\n')

        fp.write(dedent(MEME_VTABLE_TEMPLATE.format(
            vtable_fields=''.join(MEME_VTABLE_FIELD_TEMPLATE.format(param_name=l.name) for l in layouts)
        )))

        for l in layouts:
            fp.write(dedent(l.get_struct()))
            fp.write(dedent(l.get_impls()))

        fp.write(dedent(
            IMPL_PARAMS_TEMPLATE.format(impl_params=''.join(l.get_impl_param() for l in layouts))
        ))
