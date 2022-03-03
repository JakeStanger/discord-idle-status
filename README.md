# Discord Idle Status

Tiny daemon which sets your rich presence status in Discord to "idle".
Note this doesn't affect your actual online status indicator.

This is a workaround because Electron's idle detection API on Wayland doesn't work.

## Building

Simply

```shell
cargo build --release
cp target/release/discord-idle-status ~/.local/bin # or anywhere on your PATH
```

## Usage

You can run this using `swayidle` to set your status:

```shell
swayidle -w \
  timeout 600 'discord-idle-status' \
  resume 'kill $(cat /tmp/discord-idle-status.pid)'
```

The rich presence fields can be changed in the code. 
If you want to change the status title from "idle" or use any images, you will need to use a different application ID.
You can create an application at <https://discord.com/developers>.