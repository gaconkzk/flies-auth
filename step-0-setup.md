# Project setting up
-----

You need to clone this project from github.

For this service, I'll use PostgreSQL, you can add it easily by running docker command

```shell
# first time run
docker run --name postgresd -e POST_PASSWORD=fl13s_Auth -d postgres -p

# after first time, we can just start
docker start postgresd
```

We also need `libqp` and `musl-gcc` for building diesel. So install it:

```shell
sudo apt install libpq-dev musl-tools musl-dev
```

For building, deploying, and develop this project, I'm using `just`
Install it using `cargo`

```shell
cargo install just
```

For faster build, we caching built 3rds using `sccache`

```shell
cargo install sccache
```

For building normal
```shell
just build
```

For docker image `flies-auth`
```shell
just dockit
```

For cleaning
```shell
just clean
```

In dev mode, we need to rebuild-re-update services when sources changes, so I'm using `watchexec`
for rebuild.

```shell
cargo install watchexec
```

For the first time, we need to start the postgres database server with cmd: `just startdb`

Then enable live-coding with this command (service will be rebuild/restart)
```shell
just watch
```
