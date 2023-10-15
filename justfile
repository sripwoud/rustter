TRUNK_CONFIG_FILE := if os() == "windows" { "Trunk.win.toml" } else { "Trunk.toml" }
TRUNK_RELEASE_CONFIG_FILE := if os() == "windows" { "Trunk-release.win.toml" } else { "Trunk.toml" }

# build in release mode
build:
    # build frontend
    trunk --config {{TRUNK_RELEASE_CONFIG_FILE}} build
    # build backend
    cargo build --release --workspace --exclude frontend

# run `cargo check`
check:
    cargo check -p frontend --target wasm32-unknown-unknown
    cargo check --workspace --exclude frontend

# run `cargo clippy`
lint:
    cargo clippy -p frontend --target wasm32-unknown-unknown
    cargo clippy --workspace --exclude frontend

# run `clippy fix`
fix:
    cargo clippy -p frontend --fix --target wasm32-unknown-unknown --allow-dirty
    cargo clippy --workspace --fix --exclude frontend --allow-dirty

# run `cargo fmt`
fmt:
    cargo fmt


# build docs. use --open to open in browser
doc *ARGS:
    cargo doc -F docbuild {{ ARGS }}

# run frontend dev server. use --open to open a new browser
serve-ui *ARGS:
    [ ! -d "./target/dist" ] && mkdir -p "./target/dist" || true
    trunk --config {{TRUNK_CONFIG_FILE}} serve {{ ARGS }}

# run API server
serve-api *ARGS:
    cargo run -p rustter_server {{ ARGS }}

# set up project dependencies
init:
    cargo run -p project-init
    cd frontend && npm install

# migration related commands
# apply migrations
db-migrate:
    diesel migration run
    # test migration
    diesel migration redo

# reset the database
db-reset:
    diesel database reset
    diesel database reset --database-url $TEST_DATABASE_URL

# create a new database migration
db-new-migration NAME:
    diesel migration generate {{ NAME }}

