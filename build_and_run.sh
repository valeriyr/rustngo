cargo build --manifest-path ./socket/Cargo.toml

export CGO_LDFLAGS="-L$(pwd)/socket/target/debug"
export LD_LIBRARY_PATH="$(pwd)/socket/target/debug"

rm -f ./app

go build -o app app.go

./app 
