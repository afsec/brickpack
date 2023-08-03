set dotenv-load

default:
    cargo build


release:
    cargo build --release

run-native package:
    cd {{justfile_directory()}}/src/{{package}} ; cargo run

build package:
    cd {{justfile_directory()}}/src/{{package}} ; cargo build