import argparse
import os
import struct
import shutil
import subprocess
import time
import zipfile
from collections import namedtuple

try:
    import win32con
    import win32api
except:  # noqa: E722
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


def package():
    ''' Package the build into a zip '''
    zip_name = 'jdsd_dsiii_practice_tool.zip'
    if os.path.exists(zip_name):
        os.remove(zip_name)

    env = os.environ.copy()
    env['RUSTFLAGS'] = "-C link-args=-Wl,--subsystem,windows"

    subprocess.run(['cargo', 'build', '--release'], env=env)
    update_icon('target/release/jdsd_dsiii_practice_tool.exe',
                'practice-tool/src/sidherald.ico')
    # update_icon('target/release/jdsd_dsiii_savefile_manager.exe',
    #             'src/sidherald.ico')
    # update_icon('target/release/jdsd_dsiii_config_editor.exe',
    #             'src/sidherald.ico')

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
        # copy_into_zip('target/release/jdsd_dsiii_savefile_manager.exe',
        #               'jdsd_dsiii_savefile_manager.exe', zhandle)
        # copy_into_zip('target/release/jdsd_dsiii_config_editor.exe',
        #               'jdsd_dsiii_config_editor.exe', zhandle)


def run_release():
    ''' Run the tool '''
    subprocess.run(['cargo', 'build', '--release', '--lib'])
    shutil.copyfile(
        'target/release/libjdsd_dsiii_practice_tool.dll',
        'target/release/jdsd_dsiii_practice_tool.dll',
    )
    # os.startfile('steam://rungameid/374320')
    # time.sleep(10)
    subprocess.run('target/release/jdsd_dsiii_practice_tool.exe',
                   cwd='target/release')


def run():
    ''' Run the tool '''
    subprocess.run(['cargo', 'build', '--lib'])
    shutil.copyfile(
        'target/debug/libjdsd_dsiii_practice_tool.dll',
        'target/debug/jdsd_dsiii_practice_tool.dll',
    )
    # os.startfile('steam://rungameid/374320')
    # time.sleep(10)
    subprocess.run('target/debug/jdsd_dsiii_practice_tool.exe',
                   cwd='target/debug')


def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers()
    cmd_package = subparsers.add_parser(
        'package', help='Build and package the tool')
    cmd_package.set_defaults(func=package)
    cmd_run = subparsers.add_parser('run', help='Run the tool')
    cmd_run.set_defaults(func=run)
    cmd_run_release = subparsers.add_parser(
        'run_release', help='Run the tool (release config)')
    cmd_run_release.set_defaults(func=run_release)
    parser.parse_args().func()


if __name__ == '__main__':
    main()
