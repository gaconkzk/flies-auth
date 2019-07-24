production_build := "cargo build --release"

# for development watch for change recompile and run
# note: we need to start the postgres server first at localhost:5432
#   should use the `db` created in docker-compose
startdb:
  docker-compose -f docker/docker-compose.yml up -d db

watch:
  watchexec --restart "just dockit && docker-compose -f docker/docker-compose.yml up auth"

build:
  {{production_build}}

_build_musl:
  {{production_build}}

_init_docker:
  rm -rf docker/.deploy
  mkdir -p docker/.deploy

clean:
  cargo clean

dockit: _init_docker _build_musl
  cp target/release/flies-auth docker/.deploy
  docker build -q -t flies-auth docker
