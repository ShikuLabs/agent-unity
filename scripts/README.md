_**Scripts Only Run On Unix-like or OSX or WSL/WSL2, NO WINDOWS!**_

## Purpose

🛠️ To automate the boring build flow!

## The dependencies

1. We use python as build script, so your system needs to have python3:

```sh
# Check out whether you have installed python
python --version
# And pip
pip --verison
```

2. You needs to install rust toolchain in your system:

```sh
# Check out whether you have installed rustup
rustup --version
# And cargo
cargo --version
```

3. And you needs to have `Docker` in your system:

```sh
# Check out whether you have installed Docker
docker --version
```

4. Finally, you needs to install [cross-rs](https://github.com/cross-rs/cross):

    (Tips: cross-rs is 'Zero setup' `cross compilation` & `cross testing` of Rust crates)

```sh
# Check out whether you have installed Docker
cross --version
```

## First step

Run `init.sh` in root dir of the project, which helps you init the environment that scripts running on!

```sh
# 1. Go the project root dir(yes, not ./scripts)
# 2. run init.sh
sh ./init.sh
```

## How to Build

### 1. Native build

```sh
# 1. Go to root dir of project(recommand)
# 2. Run the cmd
#   ./build [{ --release | (--no-release) }] native
#
# For example:
#
# a. Build the `ic-agent-ffi` by host system's rust compiler & default target(debug mode);
./build native

# b. Or release mode
./build --release native

# The build result will be placed on ./target folder in root dir of project!

# Tips: you could find rust default target on your host system by:
rustup default
```

### 2. Cross build

```sh
# 1. Go to root dir of project(recommand)
# 2. Run the cmd
#   ./build [{ --release | (--no-release) }] cross [--arch { x86_64 | aarch64 }] [--os { osx | win | nix }]
#
# The command will cross-compile the `ic-agent-ffi` by the target specifics in docker;
#
# For example:
#
# a. Build the `ic-agent-ffi` to *.dll(windows)
./build cross --arch=x86_64 --os=win

# b. Or build the `ic-agent-fii` to *.so(linux)
./build cross --arch=x86_64 --os=nix

# c. Or build the `ic-agent-ffi` to *.dylib(osx)
./build cross --arch=x86_64 --os=osx
# Or
./build cross --arch=aarch64 --os=osx

# The build result will be placed on ./target folder in root dir of project!

# Tips: you could find rust default target on your host system by:
rustup default
```

#### Support OS & Arch

| Platform | x86 | x86_64/amd64 | armv7 | arm64/aarch64 |
| :------: | :-: | :----------: | :---: | :-----------: |
|   OSX    | ❌  |      🚧      |  ❌   |      🚧       |
|   WIN    | ❌  |      ✅      |  ❌   |      ❌       |
|   NIX    | ❌  |      ✅      |  ❌   |      ✅       |

## How to Pack

Pack the build results with `package-template` to [Unity Package](https://docs.unity3d.com/Manual/CustomPackages.html);

```sh
# 1. Go to root dir of project(recommand)
# 2. Run the cmd
#   ./pack [{ (--no-release) | --release }] [--input { (native) | cross | all }] [--version: string] [--compress： { (none) | zip }] [--output: string]
#
# Intro Options
#
# 1. --input: where the compiled libraries come from ("native" default)
# 2. --version: tag the package with version("none" default)
# 3. --compress: which format the package output("none" default)
# 4. --output: the path the package will be placed

# For example:
#
# 1. Pack native only(debug)
./pack --input=native
# Or release
./pack --release --input=native

# 2. Pack native with zip format
./pack --input=native --compress=zip

# 3. Pack cross
./pack --input=cross
```

## Publish: Build + Pack

Compile the `ic-agent-ffi` to multi targets and then pack them all as `Unity Package Format`

```sh
# 1. Go to root dir of project(recommand)
# 2. Run the cmd
#   ./publish [{ (--no-release) | --release }] [--input { (native) | cross | all }] [--version: string] [--compress： { (none) | zip }] [--output: string]
#
# Intro Options
#
# 1. --input: where the compiled libraries come from ("native" default)
# 2. --version: tag the package with version("none" default)
# 3. --compress: which format the package output("none" default)
# 4. --output: the path the package will be placed

# For example:
#
# 1. Publish native only(debug)
./publish --input=native
# Or release
./publish --release --input=native

# 2. publish native with zip format
./publish --input=native --compress=zip

# 3. publish cross
./publish --input=cross
```