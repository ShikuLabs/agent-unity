#!.venv/bin/python3

import os
import click

@click.group()
@click.option('--release/--no-release', default=False)
@click.pass_context
def cli(ctx, release):
    ctx.ensure_object(dict)
    ctx.obj['MODE'] = release

@cli.command()
@click.pass_context
def native(ctx):
    mode = '--release' if ctx.obj['MODE'] else ''
    ab_dir = os.path.dirname(os.path.realpath(__file__))
    os.system(f'cd {ab_dir}/../ic-agent-backend && cargo clean {mode}')

@cli.command()
@click.pass_context
def cross(ctx):
    mode = '--release' if ctx.obj['MODE'] else ''
    click.echo("NOT IMPL :)")

if __name__ == '__main__':
    cli()