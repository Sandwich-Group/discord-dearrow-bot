# Discord DeArrow Bot

A simple Discord bot to respond with DeArrow titles, if it finds any.

![image](https://github.com/user-attachments/assets/790f67b8-cd91-4bd3-a935-78676874fbfc)

## Motivation

There is an existing [DeArrow bot](https://github.com/SB-tools/DeArrow-Bot) that
works fairly well, but we found it to be too intrusive. Instead, we built our
own that leaves the original message intact and only responds with a new title.

## Usage

Set the `DISCORD_TOKEN` environment variable to a Discord bot token and run the
bot.

If you have the [Nix package manager](https://nixos.org/) installed, you can
generate a Docker image:
```sh
$ nix build .#dockerImageStreaming
$ ./result | docker load
$ docker run --rm -it -e DISCORD_TOKEN="$DISCORD_TOKEN" discord-dearrow-bot
```

## License

This project is licensed under CC0, which means you are free to do with this code
as you wish.
