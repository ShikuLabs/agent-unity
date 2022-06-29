#!.venv/bin/python3

# How to use
#
#   Sub CLI: Native
#   python ./script/build.py (native) [-mode { (debug) | release }]
#
#   Sub CLI: Cross
#   python ./script/build.py cross <-arch { x86_64 | aarch64 }> <-os { osx | win | nix }> [-mode { (debug) | release }]
#
# What is that
#
#   Native:
#       Compile the __backend__ as a dynamic library that targets the current host machine.
#       
#       For example, build the __backend__ on MacOS with M1 chip, the result will be
#       running on aarch64 architecture OS X operating system.
#
#   Cross:
#       Cross-Compile the __backend__ as a dynamic library that is able to run on 
#       specific architecture and operating system.
#       
#       The architecture and operating system will be determined by the user input.

import os
import click

UNKNOWN_TARGET = 'unknown-unknown-unknown-unknown'

@click.group()
@click.option('--release/--no-release', default=False)
@click.pass_context
def cli(ctx, release):
    ab_dir = os.path.dirname(os.path.realpath(__file__))

    ctx.ensure_object(dict)
    ctx.obj['MODE'] = release
    ctx.obj['ABDIR'] = ab_dir

@cli.command()
@click.pass_context
def native(ctx):
    mode = '--release' if ctx.obj['MODE'] else ''
    ab_dir = ctx.obj['ABDIR']
    os.system(f'cd {ab_dir}/../ic-agent-backend && cargo rustc {mode} -- --crate-type=cdylib')

@cli.command()
@click.option('--arch', required=True, type=click.Choice(['x86_64', 'aarch64'], case_sensitive=False))
@click.option('--os', 'os_', required=True, type=click.Choice(['osx', 'win', 'nix'], case_sensitive=False))
@click.pass_context
def cross(ctx, arch, os_):
    mode = '--release' if ctx.obj['MODE'] else ''
    ab_dir = ctx.obj['ABDIR']
    
    target = UNKNOWN_TARGET

    if arch == 'x86_64':
        if os_ == 'osx':
            click.echo('x86_64-osx will support soon!')
        elif os_ == 'win':
            target = 'x86_64-pc-windows-gnu'
        elif os_ == 'nix':
            target = 'x86_64-unknown-linux-gnu'
    elif arch == 'aarch64':
        if os_ == 'osx':
            click.echo('aarch64-osx will support soon!')
        elif os_ == 'win':
            click.echo('aarch64-win will support soon!')
        elif os_ == 'nix':
            target = 'aarch64-unknown-linux-gnu'


    if target != UNKNOWN_TARGET:
        os.system(f'cd {ab_dir}/../ic-agent-backend && cross rustc {mode} -- --crate-type=cdylib --target {target}')
    else:
        click.echo(f'No Support For {arch}-{os_}')


if __name__ == '__main__':
    cli()