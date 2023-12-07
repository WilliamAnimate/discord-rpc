# discord-rpc

This absolute joke of an html file is my attempt at reviving [ThatOneCalculator's RPC maker](https://github.com/ThatOneCalculator/DiscordRPCMaker) back from the dead. However, this repo **does not** use any code from that repo.

## Building

To build in dev mode (development only) you run b.bat

To build in release mode (as in very small executable & fast), you run c.bat

> [!NOTE]
> I have compiled the project with the nightly toolchain, via the command `rustup toolchain install nightly-2023-11-11 && rustup override set nightly-2023-11-11`

## Compiling for other platforms

So far, I can verify that this works on Windows 10, however, it *should* be possible to compile for Linux and MacOS, since this repo *does not* contain any code specific to Windows.

Snap store/flatpak discord **WILL NOT** work, until the crate (discord-rich-presence) fixes this on their end

> [!NOTE]
> you may have to remove all mentions of console.rs to compile for linux (unix) platforms

## Compiling on different platforms

### Windows

`cd src-tauri && cargo b`, but b.bat should do the job.

### Linux

> [!IMPORTANT]
> Only tested on arch linux ~~(i use arch btw)~~
>
> also, you need to remove all mentions of console.rs in main.rs because im bad at rust

`yay -S webkit2gtk` if you do not have it already (use `sudo apt-get install javascriptcoregtk-4.0` for debian based distros)

`cd src-tauri && cargo b`

## Powered by

[This crate](https://github.com/sardonicism-04/discord-rich-presence)

## Downloads

but why?
