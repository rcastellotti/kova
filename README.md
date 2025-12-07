to develop `kova` you need to have a running instance of an s3 compatible object storge. 

i am using [garage](https://garagehq.deuxfleurs.fr), here is how i set it up:

## obtain and compile `garage`
```
git clone https://git.deuxfleurs.fr/Deuxfleurs/garage.git
cd garage
cargo build --release
```

## setup garage

go through the quickstart guide and update the values hardcoded in `main.rs` to simplify testing.

crate 6 testing buckets

```sh
./garage/target/release/garage -c garage.toml bucket create kova1
./garage/target/release/garage -c garage.toml bucket create kova2
./garage/target/release/garage -c garage.toml bucket create kova3
./garage/target/release/garage -c garage.toml bucket create kova4
./garage/target/release/garage -c garage.toml bucket create kova5
./garage/target/release/garage -c garage.toml bucket create kova6
```

create a test api key and configure access to some buckets

```sh
./garage/target/release/garage -c garage.toml key create kova-test-api-key # hardcode these values in main.rs
./garage/target/release/garage -c garage.toml bucket allow --read --write --owner kova1 --key kova-test-api-key
./garage/target/release/garage -c garage.toml bucket allow --read --write --owner kova2 --key kova-test-api-key
./garage/target/release/garage -c garage.toml bucket allow --read --write --owner kova3 --key kova-test-api-key
./garage/target/release/garage -c garage.toml bucket allow --read --write --owner kova4 --key kova-test-api-key
./garage/target/release/garage -c garage.toml bucket allow --read --write --owner kova5 --key kova-test-api-key
```

start garage server

```sh
./garage/target/release/garage -c garage.toml server # config you created above
```
