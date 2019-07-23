production_build := "cargo build --release"

build:
  {{production_build}}

_build_musl:
  {{production_build}} --target x86_64-unknown-linux-musl

_init_docker:
  rm -rf docker/.deploy
  mkdir -p docker/.deploy

clean:
  cargo clean

dockit: clean _init_docker _build_musl
  cp target/x86_64-unknown-linux-musl/release/flies-auth docker/.deploy
  docker build -t flies-auth docker
