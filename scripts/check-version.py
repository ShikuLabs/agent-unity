#!.venv/bin/python3

import os
import re
import sys
import json
import toml
import click

SCRIPT_DIR = os.path.dirname(os.path.realpath(__file__))
PROJ_DIR = os.path.abspath(os.path.join(SCRIPT_DIR, os.pardir))
BEND_DIR = f'{PROJ_DIR}/ic-agent-ffi'
FEND_DIR = f'{PROJ_DIR}/package-template'

cargo_path = f'{BEND_DIR}/Cargo.toml'
package_path = f'{FEND_DIR}/package.json'

with open(cargo_path, 'r') as f:
    cargo = toml.load(f)
cargo_version = cargo['package']['version']

with open(package_path, 'r') as f:
    package = json.load(f)
package_version = package['version']

flag_a = cargo_version == package_version

if not flag_a:
    click.echo(click.style("ERROR ", fg="red") + f'package({package_version}) != cargo({cargo_version})', file=sys.stderr)
    raise click.Abort()

branch_name = os.popen('git branch --show-current').read().replace('\n', '')
is_version = re.match("[0-9]*\.[0-9]*\..*", branch_name) != None

if not is_version:
    click.echo(click.style("ATTENSION ", fg="yellow") + f'branch({branch_name}) is not version format.')
    exit(0)

flag_b = branch_name == cargo_version and branch_name == package_version

if not flag_a:
    click.echo(click.style("ERROR ", fg="red") + f'branch({branch_name}) != {package_version}', file=sys.stderr)
    raise click.Abort()