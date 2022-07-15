#!.venv/bin/python3

# How to use
#
#   Sub CLI: Native
#       ./build [{ --no-release | --release }] native
#
#   Sub CLI: Cross
#       ./build [{ (--no-release) | -- release }] cross [--arch { x86_64 | aarch64 }] [--os { osx | win | nix }]
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
    script_dir = os.path.dirname(os.path.realpath(__file__))
    project_dir = os.path.abspath(os.path.join(script_dir, os.pardir))

    ctx.ensure_object(dict)
    ctx.obj['MODE'] = release
    ctx.obj['SCRIPT_DIR'] = script_dir
    ctx.obj['PROJECT_DIR'] = project_dir

@cli.command()
@click.pass_context
def native(ctx):
    mode = '--release' if ctx.obj['MODE'] else ''
    
    project_dir = ctx.obj['PROJECT_DIR']
    
    target = os.popen('rustup default | sed -e "s/^stable-//" -e "s/(default)$//" -e "s/^nightly-//"').read().replace('\n', '')
    target_dir = f'{project_dir}/target/native-{target}'

    cmd = f'cargo rustc {mode} --manifest-path={project_dir}/ic-agent-ffi/Cargo.toml --target-dir={target_dir} -- --crate-type=cdylib'
    stats = os.system(cmd)
    code = os.WEXITSTATUS(stats)

    if code != 0:
        click.echo(click.style("ERROR", fg="red") + f": Failed to native compile to target: {target}", err=True)
        raise click.Abort()
    else:
        click.echo(click.style("OK", fg="green") + f": Succeed to native compile to target: {target}")



@cli.command()
@click.option('--arch', required=True, type=click.Choice(['x86_64', 'aarch64'], case_sensitive=False))
@click.option('--os', 'os_', required=True, type=click.Choice(['osx', 'win', 'nix'], case_sensitive=False))
@click.pass_context
def cross(ctx, arch, os_):
    mode = '--release' if ctx.obj['MODE'] else ''

    project_dir = ctx.obj['PROJECT_DIR']
    
    target = UNKNOWN_TARGET

    if arch == 'x86_64':
        if os_ == 'osx':
            target = "x86_64-apple-darwin"
        elif os_ == 'win':
            target = "x86_64-pc-windows-gnu"
        elif os_ == 'nix':
            target = "x86_64-unknown-linux-gnu"
    elif arch == 'aarch64':
        if os_ == 'osx':
            target = "aarch64-apple-darwin"
        elif os_ == 'win':
            click.echo(click.style("ERROR", fg="red") + ": Not support target: aarch64-pc-windows-gnu", err=True)
            raise click.Abort()
        elif os_ == 'nix':
            click.echo(click.style("ERROR", fg="red") + ": Not support target: aarch64-unknown-linux-gnu", err=True)
            raise click.Abort()

    if target != UNKNOWN_TARGET:
        target_dir = f'{project_dir}/target/cross-{target}'

        cmd = f'cross rustc {mode} --manifest-path={project_dir}/ic-agent-ffi/Cargo.toml --target-dir={target_dir} --target={target} -- --crate-type=cdylib'

        stats = os.system(cmd)
        code = os.WEXITSTATUS(stats)

        if code != 0:
            click.echo(click.style("ERROR", fg="red") + f": Failed to cross compile to target: {target}", err=True)
            raise click.Abort()
        else:
            click.echo(click.style("OK", fg="green") + f": Succeed to cross compile to target: {target}")


if __name__ == '__main__':
    cli()