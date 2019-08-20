import os
import sys
import glob
import shutil
import subprocess

if __name__ == '__main__':
  if not os.path.exists('build'):
    os.mkdir('build')
  os.chdir('build')

  BUILD_CONFIG='RelWithDebInfo'

  print(sys.argv)
  if len(sys.argv) > 1:
    BUILD_CONFIG = sys.argv[1]

  cmake_args = ['-DCMAKE_BUILD_TYPE=' + BUILD_CONFIG, '../']
  subprocess.run(['cmake', *cmake_args])
  subprocess.run(['cmake', '--build', '.', '--config', BUILD_CONFIG])
  os.chdir('..')