# Agent of Internet Computer for Unity

## The Intro

The project brings the `IC` ecosystem to `Unity`, allowing Unity developers to call the functions of canisters on IC,
providing useful utilities to make authentication and authorization easy.


## The Status

The project is in an early stage, feature-less, documentation-sparse, and API will be changed frequently.

__NOTE__: __Beta version is available, Only for experiment!__

## Milestones

### Milestone 01: 【Draft】Call IC methods on Unity3D

- [x] ✨ Support `HostKeyStore`, a simple encrypted account module;
- [x] ✨ Support login/logout by `HostKeyStore`;
- [x] ✨ Call query methods on ic mainnet;
- [x] ✨ Call update methods on ic mainnet;
- [x] ✨ Support target: x86_64-win;
- [x] ✨ Support target: x86_64-nix;
- [x] ✨ Support target: aarch64-osx;
- [x] ✨ Support target: x86_64-osx;

### Milestone 02: 【Basic】Core features/libraries mapping

- [ ] ✨ Mapping `candid` from rs to cs;
- [x] ✨ Mapping `ic-types` from rs to cs;
- [x] ✨ Mapping `ic-agent` from rs to cs;
- [x] ✨ Mapping `ic-utils` from rs to cs;

## How to build

```sh
# init python env
sh ./init.sh

# make the unity package, will produce a unity package with four targets:
# x86_64-win, x86_64-nix, x86_64-osx, aarch64-win
./publish --release --input=all
```

Look the __[detail](./scripts/README.md)__.

## How to install

- Install of the official UPM package

    __TODO__

- Installation via Git in UPM

    ![upm-via-git](./docs/imgs/upm-via-git.png)

    ```
    https://github.com/ShikuLabs/agent-unity.git#upm
    ```

- Installation via

    ![upm-via-git](./docs/imgs/upm-via-local.png)

    __NOTE__: The installation file will be put on __Release__ page.

## How to use

Please check the samples of ICAgent!

__Window__ -> __Package Manager__ -> `Find IC Agent Package` -> `Choice the sample you want to check`

![upm-samples](./docs/imgs/upm-samples.png)