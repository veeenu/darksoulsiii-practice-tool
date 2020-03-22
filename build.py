import os, sys, glob, shutil, subprocess, argparse, zipfile

def build(ns):
  if not os.path.exists('build'):
    os.mkdir('build')
  os.chdir('build')

  cmake_args = ['-DCMAKE_BUILD_TYPE=' + ns.config, '../']
  subprocess.run(['cmake', *cmake_args])
  subprocess.run(['cmake', '--build', '.', '--config', ns.config])
  os.chdir('..')

def package(ns):
  zip_name = 'jdsd_dsiii_practice_tool.zip'
  if os.path.exists(zip_name):
    os.remove(zip_name)

  def copy_into_zip(src, zf):
    basename = os.path.basename(src)
    with zf.open(basename, mode='w') as dest, open(src, 'rb') as fp:
      dest.write(fp.read())

  with zipfile.ZipFile(zip_name, mode='w') as zf:
    copy_into_zip('build/Release/jdsd_dsiii_practice_tool.exe', zf)
    copy_into_zip('build/Release/jdsd_dsiii_practice_tool.dll', zf)
    copy_into_zip('jdsd_dsiii_practice_tool.toml', zf)

if __name__ == '__main__':
  parser = argparse.ArgumentParser(description='Build and package the tool')
  subparsers = parser.add_subparsers(help='sub-commands')

  cmd_build = subparsers.add_parser('build')
  cmd_build.add_argument('--config', choices=['Release', 'Debug', 'RelWithDebInfo'], default='RelWithDebInfo', help='build configuration')
  cmd_build.set_defaults(func=build)

  cmd_package = subparsers.add_parser('package')
  cmd_package.set_defaults(func=package)

  args = parser.parse_args()
  if hasattr(args, 'func'):
    args.func(args)