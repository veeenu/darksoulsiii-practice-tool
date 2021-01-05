import os
import struct
import subprocess
import zipfile
from collections import namedtuple

try:
    import win32con
    import win32api
except Exception as _:
    print('This script needs pywin32 to run: https://github.com/mhammond/pywin32')


def update_icon(exe_file, icon_file):
    GroupHeader = namedtuple(
        'GroupHeader', 'reserved type count width height ccount reserved1 planes bcount bytes offset')

    with open(icon_file, 'rb') as fp:
        buf = fp.read()

    (width, height, count, offset) = struct.unpack('=BBxxxxxxII', buf[6:22])
    icon_data = buf[offset:offset + count]
    group_data = GroupHeader._make(struct.unpack('=HHHBBBBHHII', buf[:22]))
    group_data = group_data._replace(offset=1)
    group_data = struct.pack('=HHHBBBBHHII', *tuple(group_data))

    handle = win32api.BeginUpdateResource(exe_file, False)
    win32api.UpdateResource(handle, win32con.RT_ICON, 1, icon_data)
    win32api.UpdateResource(
        handle, win32con.RT_GROUP_ICON, 'IDI_ICON', group_data)
    win32api.EndUpdateResource(handle, False)


def main():
    ''' Package the build into a zip '''
    zip_name = 'jdsd_dsiii_practice_tool.zip'
    if os.path.exists(zip_name):
        os.remove(zip_name)

    subprocess.run(['cargo', 'build', '--release'])
    update_icon('target/release/jdsd_dsiii_practice_tool.exe',
                'src/sidherald.ico')

    def copy_into_zip(src, destn, zhandle):
        with zhandle.open(destn, mode='w') as dest, open(src, 'rb') as handle:
            dest.write(handle.read())

    with zipfile.ZipFile(zip_name, mode='w') as zhandle:
        copy_into_zip('target/release/jdsd_dsiii_practice_tool.exe',
                      'jdsd_dsiii_practice_tool.exe', zhandle)
        copy_into_zip('target/release/libjdsd_dsiii_practice_tool.dll',
                      'jdsd_dsiii_practice_tool.dll', zhandle)
        copy_into_zip('jdsd_dsiii_practice_tool.toml',
                      'jdsd_dsiii_practice_tool.toml', zhandle)


if __name__ == '__main__':
    main()
