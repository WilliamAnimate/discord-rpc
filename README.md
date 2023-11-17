# discord-rpc

This absolute joke of an html file is my attempt at reviving [ThatOneCalculator's RPC maker](https://github.com/ThatOneCalculator/DiscordRPCMaker) back from the dead. However, this repo **does not** use any code from that repo.

## Building

To build in dev mode (faster) you run b.bat

To build in release mode (as in very small executable), you run c.bat

> [!IMPORTANT]
> You **WILL** need the nightly toolchain installed for release mode, simply run `rustup toolchain install nightly-2023-11-11 && rustup override set nightly-2023-11-11`

## Compiling for other platforms

So far, I can verify that this works on Windows 10, however, it *should* be possible to compile for Linux and MacOS, since this repo *does not* conain any code specific to Windows.

## Powered by

[This crate](https://github.com/sardonicism-04/discord-rich-presence)

## Downloads

but why?
