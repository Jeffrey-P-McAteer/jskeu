
import json
import os
import sys
import subprocess
import shutil

def stdout_of(bin_name, *args):
  cmd = [shutil.which(bin_name)]
  cmd += list(args)
  try:
    return subprocess.check_output(cmd).decode('utf-8')
  except:
    traceback.print_exc()
    return ''

def main(repo_root='.'):
  if not os.path.exists('.secrets.json'):
    raise RuntimeError('Please create .secrets.json first!')

  secrets = {
    "GH_TOKEN": ""
  }
  with open('.secrets.json', 'r') as fd:
    secrets = json.loads(fd.read())

  version_s = '0.0'
  with open('version.txt', 'r') as fd:
    version_s = fd.read().strip()

  commits_since_version_txt_modified = stdout_of(
    'git', 'rev-list', '--count', stdout_of('git', 'log', '--follow', '-1', '--pretty=%H', 'version.txt').strip()+'..HEAD'
  ).strip()
  if not commits_since_version_txt_modified or len(commits_since_version_txt_modified) < 1:
    commits_since_version_txt_modified = '0'

  version = '{}.{}'.format(version_s, commits_since_version_txt_modified)

  print(f'Creating version {version}')



