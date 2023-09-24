# PopYandeReBot

A telegram bot for RSS yande.re popular recent written in Rust

## Run

There are several ways to run

### Configs

First of all, you need to modify your configurations. Use the `configs_example.toml` in the repo, and rename it `configs.toml`

```bash
$ cp configs_example.toml configs.toml
$ vim configs.toml
......
```

### Docker

```bash
$ docker run --name pop_yandere_bot -v ./configs.toml:/app/configs.toml hexsix/pop_yandere_bot:latest
$ docker restart pop_yandere_bot
```

### Linux

Put the `configs.toml` under the same folder and run binary

```bash
$ ./pop_yandere_bot
```

### Windows

Put the `configs.toml` under the same folder and run binary

```powershell
$ .\pop_yandere_bot.exe
```

### macOS

Put the `configs.toml` under the same folder and run binary

```bash
$ ./pop_yandere_bot
```
