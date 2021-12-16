import pandas as pd
import re
from glob import glob
from pathlib import Path
from itertools import zip_longest
from textwrap import dedent


SNAKECASE_RE = re.compile(r'(?!^)([A-Z]+)')
SNAKECASE_CLEAN_RE = re.compile(r'_+')
SLUG_RE = re.compile(r'([^a-zA-Z]+)')

STRUCT_TEMPLATE = '''
    #[derive(Debug)]
    #[repr(C)]
    pub(crate) struct {param_name} {{
        {fields}
    }}
'''

IMPL_STRUCT_TEMPLATE = '''
    impl {param_name} {{ {methods}
    }}
'''

IMPL_PARAMS_TEMPLATE = '''
    impl Params {{{impl_params}    }}
'''

IMPL_PARAM_TEMPLATE = '''
        #[allow(unused)]
        pub(crate) unsafe fn get_{name_snake_case}(&self) -> Option<impl Iterator<Item = Param<{param_name}>>> {{
            self.iter_param::<{param_name}>("{param_name}")
        }}
'''

FIELD_TEMPLATE = '''
        pub(crate) {field_name}: {field_type},
'''.strip()

BITFIELD_TEMPLATE = '''
        #[allow(unused)]
        pub(crate) fn set_{flag_name}(&mut self, state: bool) {{
            const FIELD_INDEX: {field_type} = 1 << {field_index};
            let val = self.{bitfield_name};
            self.{bitfield_name} = if state {{
                val | FIELD_INDEX
            }} else {{
                val & !FIELD_INDEX
            }};
        }}

        #[allow(unused)]
        pub(crate) fn {flag_name}(&mut self) -> bool {{
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
        if len(impls) > 0:
            return IMPL_STRUCT_TEMPLATE.format(
                param_name=self.name,
                methods=''.join(impls)
            )
        else:
            return ''

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
        self.fields = enumerate(ParamLayout.dedup_fields(fields))

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

    def get_impls(self):
        return []

    def rename(self, idx):
        self.name = self.name + f'_{idx}'
            

if __name__ == '__main__':
    with open(HERE / '../../ccs-mod/src/params/param_data.rs', 'w') as fp:
        fp.write('// **********************************\n')
        fp.write('// *** AUTOGENERATED, DO NOT EDIT ***\n')
        fp.write('// **********************************\n')
        fp.write('use super::*;\n')

        layouts = build_param_layouts()
        for l in layouts:
            fp.write(dedent(l.get_struct()))
            fp.write(dedent(l.get_impls()))

        fp.write(dedent(
            IMPL_PARAMS_TEMPLATE.format(impl_params=''.join(l.get_impl_param() for l in layouts))
        ))
