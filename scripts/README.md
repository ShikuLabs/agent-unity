# Introduction

_**Scripts Only Run On Unix-like or OSX or WSL/WSL2, NO WINDOWS!**_

## Purpose

Makes it easy to build, test and publish.

## Dependencies

1. [docker](https://www.docker.com/products/docker-desktop/)
2. [cross-rs](https://github.com/cross-rs/cross)

## How to build?

__`THE PATH`: `./ic-agent-frontend/Plugins/{os}/{arch}/ic-agent.{dll/so/dylib}`__

The build means to compile the __backend__ to dynamic library, and then copy the dynamic library to __`THE PATH`__;

For instance, build the __backend__ on MacOS with M1 chip, the result dynmaic library will be copied to `./ic-agent-frontend/Plugins/osx/aarch64/ic-agent.dylib`.

### 1. Native Build(Useful in Developing)

Compile the __backend__ as dynamic library to host system then copy it to __`THE PATH`__.

```sh
# Debug
sh ./native-build
# Or
sh ./native-build --debug

# Release
sh ./native-build --release
```

### 2. Cross Build(Useful in Publish)

Cross-compile the __backend__ as dynamic library to specific system then copy it to __`THE PATH`__.

__Cross-Build only supports three operating system and two architecture right now!__

#### Support OS & Arch

| Platform | x86 | x86_64/amd64 | armv7 | arm64/aarch64 |
| :------: | :-: | :----------: | :---: | :-----------: |
|   OSX    | ❌  |      ✅      |  ❌   |      ✅       |
|   WIN    | ❌  |      ✅      |  ❌   |      ✅       |
|   NIX    | ❌  |      ✅      |  ❌   |      ✅       |

```sh
# Defination
#
# <os>   : { osx | win | nix }
# <arch> : { x86_64 | aarch64 }
# [mode] : { debug | release }
sh ./cross-build <os> <arch> [mode]

# For example, build to MacOS with apple silicon(Debug)
sh ./cross-build osx aarch64
# Or
sh ./cross-build osx aarch64 --debug
# Build to MacOS with apple silicon(Release)
sh ./cross-build osx aarch64 --release
```

## How to test?

### 1. Run backend unit-tests & integration-tests;

TODO

### 2. Run frontend unit-tests(for api) & system-tests;

TODO

## How to publish?

The publish means to compile the __backend__ to different os & arch and then pack them with __frontend__ as __Unity Package__.

```sh
# Defination
#
# [exclude]
#   intro: The targets in [exclude] will not be built;
#   value: { x86_64-osx | aarch64-osx | x86_64-win | aarch64-win | x86_64-nix | aarch64-nix }
sh ./publish [exclude]

# For example, publish wihtout x86_64-osx and aarch64-win;
sh ./publish --exclude=x86_64-osx, aarch64-win
```