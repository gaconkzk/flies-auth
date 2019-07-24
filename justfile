# for development watch for change recompile and run
# note: we need to start the postgres server first at localhost:5432
#   should use the `db` created in docker-compose

export RUSTC_WRAPPER := `which sccache`

startdb:
  docker-compose -f docker/docker-compose.yml up -d db

watch:
  watchexec --restart "just dockit && docker-compose -f docker/docker-compose.yml up auth"

build:
  cargo build --release

_init_docker:
  rm -rf docker/.deploy
  mkdir -p docker/.deploy

test:
  cargo test

clean:
  cargo clean

dockit: _init_docker build
  cp target/release/flies-auth docker/.deploy
  docker build -q -t flies-auth docker
