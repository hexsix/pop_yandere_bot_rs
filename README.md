# PopYandeReBot

A telegram bot for RSS yande.re popular recent written in Rust

See example at https://t.me/yandere_pop_recent

## 1 Run

There are several ways to run, but you should configure it first

### 1.1 Configs

#### 1.1.1 Toml

Use the `configs_example.toml` in the repo, and rename it `configs.toml`

```bash
$ cp configs_example.toml configs.toml
$ vim configs.toml
```

The following must be configured

- `db.database_url`
- `telegram.token`
- `telegram.channel_id`

#### 1.1.2 Env

Env variables will override the toml configs

```bash
$ APP_db={database_url=\"redis://localhost:6379\"} APP_telegram={token=\"1234567890:abcdefghijklmnopqrstuvwxyz\",channel_id=\"-0123456\"}
```

### 1.2 Docker

```bash
$ docker run --name pop_yandere_bot hexsix/pop_yandere_bot:latest
```

### 1.3 Linux

```bash
$ ./pop_yandere_bot
```
