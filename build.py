#!/usr/bin/env python

import os
import sys
import subprocess
import shutil
import platform
import traceback

def trim_nones(the_list):
  new_l = []
  for n in the_list:
    if n is None:
      continue
    new_l.append(n)
  return new_l

def check_bins(*bin_array):
  num_errs = 0
  for bin_name, description_msg in trim_nones(list(bin_array)):
    if isinstance(bin_name, list):
      at_least_one_exists = False
      for b in bin_name:
        if shutil.which(b):
          at_least_one_exists = True;
          break
      if not at_least_one_exists:
        num_errs += 1
        print("[ error ] required tool not found; need at least one of {}. {}".format(
          ', or '.join(["'{}'".format(x) for x in bin_name]), description_msg
        ))

    else:
      if not shutil.which(bin_name):
        num_errs += 1
        print("[ error ] required tool '{}' not found. {}".format(bin_name, description_msg))

  return num_errs

def host_is_linux():
  return 'linux' in platform.system().lower()

def host_is_windows():
  return 'windows' in platform.system().lower()

def host_is_macos():
  return 'darwin' in platform.system().lower()

def cmd(bin_name, *args):
  cmd = [shutil.which(bin_name)]
  cmd += list(args)

  print('[ info ] running command: {}'.format( ' '.join(cmd) ))
  subprocess.run(cmd, check=True)

def stdout_of(bin_name, *args):
  cmd = [shutil.which(bin_name)]
  cmd += list(args)
  try:
    return subprocess.check_output(cmd).decode('utf-8')
  except:
    traceback.print_exc()
    return ''

def errif_ncontain(haystack, needle, err_msg):
  if not needle in haystack:
    print('[ error ] {}'.format(err_msg))
    return 1
  return 0

if __name__ == '__main__':
  build_hostonly = not 'all' in sys.argv

  # cd to directory containing build.py
  os.chdir(
    os.path.dirname( os.path.abspath(__file__) )
  )

  # Add our 'tool_shims' to the OS path so we can refer to them by name
  os.environ['PATH'] = os.path.abspath('tool_shims')+os.pathsep+os.environ['PATH']

  num_errs = 0
  num_errs += check_bins(
    ('rustup', 'The build script uses rustup to detect currently installed rust targets.'),
    ('cargo', 'Cargo is required to compile rust and perform low-level build logic.'),
    (['gcc', 'clang'], 'At least one good C compiler is required for the host target.'),
    ('zig', 'We use zig to cross-compile everything for foreign targets.'),
  )
  ito = stdout_of('rustup', 'target', 'list', '--installed')
  num_errs += errif_ncontain(ito, 'x86_64-unknown-linux-gnu', 'You must install the x86_64-unknown-linux-gnu toolchain for rust: rustup target add x86_64-unknown-linux-gnu')
  num_errs += errif_ncontain(ito, 'x86_64-pc-windows-gnu', 'You must install the x86_64-pc-windows-gnu toolchain for rust: rustup target add x86_64-pc-windows-gnu')
  num_errs += errif_ncontain(ito, 'x86_64-apple-darwin', 'You must install the x86_64-apple-darwin toolchain for rust: rustup target add x86_64-apple-darwin')
  #num_errs += errif_ncontain(ito, 'aarch64-apple-darwin', 'You must install the aarch64-apple-darwin toolchain for rust: rustup target add aarch64-apple-darwin')
  
  if num_errs > 0:
    print('Exiting because of {} development environment errors detected above.'.format(num_errs))
    sys.exit(1)

  # Now begin compile for all targets

  cmd('cargo', 'build', '--release', '--target=x86_64-unknown-linux-gnu')
  cmd('cargo', 'build', '--release', '--target=x86_64-pc-windows-gnu')
  cmd('cargo', 'build', '--release', '--target=x86_64-apple-darwin')
  #cmd('cargo', 'build', '--release', '--target=aarch64-apple-darwin')

  print('='*18, 'build complete', '='*18)
  expected_binaries = [
    os.path.join('target', 'x86_64-unknown-linux-gnu', 'release', 'jskeu'),
    os.path.join('target', 'x86_64-pc-windows-gnu', 'release', 'jskeu.exe'),
    os.path.join('target', 'x86_64-apple-darwin', 'release', 'jskeu'),
  ]
  for b in expected_binaries:
    print('Built {}'.format(b))




