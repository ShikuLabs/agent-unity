#!.venv/bin/python3

# How to use
#
#   CLI
#       ./pack [{ (--no-release) | --release }] [--input { (native) | cross | all }] [--version: string] [--compress： { (none) | zip }] [--output: string]
#
# __NOTE__
#
# The Choice Rule: Which libraries will be packed?
#
#   --input: The place where the libraries come from;
#       --input=native: the libraries from ./target/native_*/ folder only;
#       
#       --input=cross: the libraries from ./target/cross_*/ folder only;
#
#       --input=all: the libraries from ./target/native_*/ & ./target/cross_*, when meet conflict, take `cross` first;
#
# What is that
#
# Pack the compiled dynamic libraries with package-template to output path;

import os
import re
import json
import click
import shutil
import pathlib

@click.command()
@click.option('--release/--no-release', default=False)
@click.option('--input', type=click.Choice(['native', 'cross', 'all'], case_sensitive=False), default='native')
@click.option('--compress', type=click.Choice(['none', 'zip'], case_sensitive=False), default='none')
@click.option('--version', required=True, type=str, default='none')
@click.option('--output', required=True, type=str, default='./')
def pack(release, input, compress, version, output):
    if not os.path.isdir(output):
        click.echo(click.style("ERROR", fg="red") + f": Wrong option --output={output}, should be directory!", err=True)
        raise click.Abort()

    script_dir = os.path.dirname(os.path.realpath(__file__))
    project_dir = os.path.abspath(os.path.join(script_dir, os.pardir))
    target_dir = f'{project_dir}/target'
    package_dir = f'{project_dir}/package-template'

    mode = 'release' if release else 'debug'

    # 1. get dynamic library file paths
    dy_paths = []
    exts = {'.dylib', '.dll', '.so'}
    for path in pathlib.Path(target_dir).glob(f'**/{mode}/*'):
        if path.suffix in exts:
            dy_paths.append(str(path))

    # 2. capture the meta info: { native | cross }-{arch}-*-{os}
    dy_dicts = {}
    for dy_path in dy_paths:
        rgx = '/(?P<input>native|cross)-(?P<arch>x86_64|aarch64)-(.*)-(?P<os>darwin|windows|linux)'
        mat = re.search(rgx, dy_path)

        input_ = mat.group('input')

        # filter by `--input`
        if input != 'all' and input != input_: continue

        dy_dicts[dy_path] = {
            "input": mat.group('input'),
            "arch": mat.group('arch'),
            "os": mat.group('os')
        }

    # break point
    if len(dy_dicts) == 0: 
        click.echo(click.style("ATTENSION", fg="yellow") + f": Failed to find dynamic libraries that matched, EXIT!")
        return

    # 3. copy `./package-template` to `./pack-temp`
    pack_temp_dir = f'{project_dir}/pack-temp'
    if os.path.isdir(pack_temp_dir):
        shutil.rmtree(pack_temp_dir)
    cmd = f'cp -r {package_dir} {pack_temp_dir}'
    stats = os.system(cmd)
    code = os.WEXITSTATUS(stats)

    if code != 0:
        click.echo(click.style("ERROR", fg="red") + f": Failed to create ./pack-temp, EXIT!", err=True)
        raise click.Abort()

    is_failed = False

    # 4. copy the dynamic library to matched folder in `./pack-temp` (cross first)
    for path, meta in dy_dicts.items():
        input_ = meta['input']
        arch = meta['arch']
        os_ = map_os(meta['os'])

        ext = pathlib.Path(path).suffix.replace('.', '')
        dst_path = f'{pack_temp_dir}/Plugins/{arch}/{os_}/ic-agent.{ext}'

        # if there has had a library already, jump over if the current library is `native`
        if os.path.isfile(dst_path) and input_ == 'native': continue

        cmd = f'cp -r {path} {dst_path}'
        stats = os.system(cmd)
        code = os.WEXITSTATUS(stats)

        if code != 0:
            is_failed = True
            
            click.echo(click.style("ERROR", fg="red") + f": Failed to copy {path} to destination path, EXIT!", err=True)
            raise click.Abort()

    # 5. change the version in package.json
    if not is_failed:
        with open(f'{pack_temp_dir}/package.json', 'r') as f:
            pkg = json.load(f)

        name = pkg['name']

        version = version if version != 'none' else pkg['version']
        pkg['version'] = version

        with open(f'{pack_temp_dir}/package.json', 'w') as f:
            json.dump(pkg, f)

    # 6. compress `./pack-temp`
    new_name = f'{name}-{map_version(version)}'
    package_dst_dir = f'{output}/{new_name}'

    if compress == 'none':
        shutil.copytree(pack_temp_dir, package_dst_dir)
    elif compress == 'zip':
        cmd = f'cd {pack_temp_dir} && zip -r ../{new_name}.zip *'
        stats = os.system(cmd)
        code = os.WEXITSTATUS(stats)

        if code != 0:
            is_failed = True
            click.echo(click.style("ERROR", fg="red") + f": Failed to zip {pack_temp_dir} to {package_dst_dir}.zip, EXIT!", err=True)
            raise click.Abort()
        
        shutil.move(f'./{new_name}.zip', f'{package_dst_dir}.zip')

    # 7. delete `./pack-temp`
    cmd = f'rm -rf {pack_temp_dir}'
    stats = os.system(cmd)
    code = os.WEXITSTATUS(stats)

    if code != 0:
        click.echo(click.style("ERROR", fg="red") + f": Failed to delete temporary ./pack-temp, EXIT!", err=True)
        raise click.Abort()

def map_os(os_):
    if os_ == 'darwin':
        return 'osx'
    elif os_ == 'linux':
        return 'nix'
    elif os_ == 'windows':
        return 'win'

def map_version(version: str):
    return version.replace('.', '_')

if __name__ == '__main__':
    pack()