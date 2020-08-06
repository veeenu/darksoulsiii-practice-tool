import os
import sys
import glob
import shutil
import subprocess
import argparse
import zipfile


def build(namespace):
    ''' Build the executable '''
    if not os.path.exists('build'):
        os.mkdir('build')
    os.chdir('build')

    cmake_args = ['-DCMAKE_BUILD_TYPE=' + namespace.config, '../']
    subprocess.run(['cmake', *cmake_args])
    subprocess.run(['cmake', '--build', '.', '--config', namespace.config])
    os.chdir('..')


def package(namespace):
    ''' Package the build into a zip '''
    zip_name = 'jdsd_dsiii_practice_tool.zip'
    if os.path.exists(zip_name):
        os.remove(zip_name)

    def copy_into_zip(src, zhandle):
        basename = os.path.basename(src)
        with zhandle.open(basename, mode='w') as dest, \
                open(src, 'rb') as handle:
            dest.write(handle.read())

    with zipfile.ZipFile(zip_name, mode='w') as zhandle:
        copy_into_zip('build/Release/jdsd_dsiii_practice_tool.exe', zhandle)
        copy_into_zip('build/Release/jdsd_dsiii_practice_tool.dll', zhandle)
        copy_into_zip('jdsd_dsiii_practice_tool.toml', zhandle)


def main():
    ''' Main '''
    parser = argparse.ArgumentParser(description='Build and package the tool')
    subparsers = parser.add_subparsers(help='sub-commands')

    cmd_build = subparsers.add_parser('build')
    cmd_build.add_argument(
        '--config',
        choices=['Release', 'Debug', 'RelWithDebInfo'],
        default='RelWithDebInfo',
        help='build configuration'
    )
    cmd_build.set_defaults(func=build)

    cmd_package = subparsers.add_parser('package')
    cmd_package.set_defaults(func=package)

    args = parser.parse_args()
    if hasattr(args, 'func'):
        args.func(args)


if __name__ == '__main__':
    main()

