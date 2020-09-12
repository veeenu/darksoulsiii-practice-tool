import os
import argparse
import subprocess
import zipfile


def main():
    ''' Package the build into a zip '''
    zip_name = 'jdsd_dsiii_practice_tool.zip'
    if os.path.exists(zip_name):
        os.remove(zip_name)

    subprocess.run(['cargo', 'build', '--release'])

    def copy_into_zip(src, destn, zhandle):
        with zhandle.open(destn, mode='w') as dest, open(src, 'rb') as handle:
            dest.write(handle.read())

    with zipfile.ZipFile(zip_name, mode='w') as zhandle:
        copy_into_zip('target/release/jdsd_dsiii_practice_tool.exe', 'jdsd_dsiii_practice_tool.exe', zhandle)
        copy_into_zip('target/release/libjdsd_dsiii_practice_tool.dll', 'jdsd_dsiii_practice_tool.dll', zhandle)
        copy_into_zip('jdsd_dsiii_practice_tool.toml', 'jdsd_dsiii_practice_tool.toml', zhandle)


if __name__ == '__main__':
    main()
