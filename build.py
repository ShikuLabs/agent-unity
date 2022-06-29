#!/usr/bin/env python

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

# TODO: 脚本依赖
# 1. python3

# TODO
# 1. 读取脚本路径
# 2. 得到用户输入命令 sub-cmd, mode..
# 3. 处理native子命令(简单)
# 4. 处理cross子命令(困难)
# 5. 结束

import click

@click.group()
def cli():
    pass

@cli.command()
def initdb():
    click.echo('Init db')

@cli.command()
def dropdb():
    click.echo('Drop db')

if __name__ == '__main__':
    cli()