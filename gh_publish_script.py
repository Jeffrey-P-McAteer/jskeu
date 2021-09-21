
import json
import os
import sys
import subprocess
import shutil
import urllib.request

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


def post(url, data={}):
  with urllib.request.urlopen(url,data=json.dumps(data).encode('utf-8')) as f:
    return f.read().decode('utf-8')


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

  all_tags = stdout_of('git', 'tag')
  if version in all_tags:
    raise RuntimeError('Version {} already exists, commit at least one new change first!'.format(version))

  first_lines_cargo_toml = ''
  with open('Cargo.toml', 'r') as fd:
    for i, line in zip(range(0, 5), fd.readlines()):
      first_lines_cargo_toml += line
      first_lines_cargo_toml += os.linesep

  if not version_s in first_lines_cargo_toml:
    raise RuntimeError('Please update Cargo.toml to list the version as at least {}'.format(version_s))

  git_status = stdout_of('git', 'status', '-s').strip()
  if len(git_status) > 1:
    raise RuntimeError('Please commit all changes, refusing to release a dirty working tree!')

  git_tag = 'v{}'.format(version)

  print(f'Creating {git_tag}')

  cmd('git', 'tag', '-a', git_tag, '-m', git_tag)

  release_msg = '''

  '''.format(
    
  ).strip()

  post(
    'https://api.github.com/repos/Jeffrey-P-McAteer/jskeu/releases?access_token=%s'.format(secrets['GH_TOKEN'])
    data={
      'tag_name': "v%s" % (new_version),
      'target_commitish': args.branch,
      'name': "v%s" % (new_version),
      'body': release_msg,
      'draft': False,
      'prerelease': False
    }
  )


if __name__ == '__main__':
  main()

