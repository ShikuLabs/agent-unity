#!.venv/bin/python3

# How to use
#
#   Sub CLI: Native
#       ./publish [{ (--no-release) | --release }] [--input { (native) | cross | all }] [--version: string] [--compress： { (none) | zip }] [--output: string]
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
# Build + Pack

import os
import click

@click.command()
@click.option('--release/--no-release', default=False)
@click.option('--input', type=click.Choice(['native', 'cross', 'all'], case_sensitive=False), default='native')
@click.option('--compress', type=click.Choice(['none', 'zip'], case_sensitive=False), default='none')
@click.option('--version', required=True, type=str, default='none')
@click.option('--output', required=True, type=str, default='./')
@click.pass_context
def publish(ctx, release, input, compress, version, output):
    mode = '--release' if release else ''
    
    script_dir = os.path.dirname(os.path.realpath(__file__))
    project_dir = os.path.abspath(os.path.join(script_dir, os.pardir))

    build_script_path = f'{script_dir}/build.py'
    pack_script_path = f'{script_dir}/pack.py'

    def do_native():
        # build
        cmd = f'{build_script_path} {mode} native'
        stats = os.system(cmd)
        code = os.WEXITSTATUS(stats)
        
        if code != 0:
            click.echo(click.style("ERROR", fg="red") + f": Failed to run build {mode} native!", err=True)
            raise click.Abort()

    def do_cross():
        # build
        valid_targets = [
            {"arch": "x86_64", "os": "win"},
            {"arch": "x86_64", "os": "nix"},
            {"arch": "x86_64", "os": "osx"},
            {"arch": "aarch64", "os": "osx"}
        ]

        for target in valid_targets:
            arch = target['arch']
            os_ = target['os']
            cmd = f'{build_script_path} {mode} cross --arch={arch} --os={os_}'
            stats = os.system(cmd)
            code = os.WEXITSTATUS(stats)
        
            if code != 0:
                click.echo(click.style("ERROR", fg="red") + f": Failed to run build {mode} native!", err=True)
                raise click.Abort()

    if input == 'native': do_native()
    elif input == 'cross': do_cross()
    elif input == 'all':
        do_native()
        do_cross()

    # pack
    cmd = f'{pack_script_path} {mode} --input {input} --compress {compress} --version {version} --output {output}'
    stats = os.system(cmd)
    code = os.WEXITSTATUS(stats)
    
    if code != 0:
        click.echo(click.style("ERROR", fg="red") + f": Failed to pack {mode} native!", err=True)
        raise click.Abort()
    
if __name__ == '__main__':
    publish()