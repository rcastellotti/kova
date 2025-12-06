
testing is done with https://garagehq.deuxfleurs.fr
go through the quickstart guide and update the values hardcoded in `main.rs` .

1. clone and compile garage 
2.
```sh
./garage/target/release/garage layout assign -z dc1 -c 1G <node_id>
./garage/target/release/garage layout apply --version 1
```
```sh
./garage/target/release/garage bucket create kova1
./garage/target/release/garage bucket create kova2
./garage/target/release/garage bucket create kova3
./garage/target/release/garage bucket create kova4
./garage/target/release/garage bucket create kova5
./garage/target/release/garage bucket create kova6
./garage/target/release/garage key create kova-test-api-key
./garage/target/release/garage bucket allow --read --write --owner kova1 --key kova-test-api-key
./garage/target/release/garage bucket allow --read --write --owner kova2 --key kova-test-api-key
./garage/target/release/garage bucket allow --read --write --owner kova3 --key kova-test-api-key
./garage/target/release/garage bucket allow --read --write --owner kova4 --key kova-test-api-key
./garage/target/release/garage bucket allow --read --write --owner kova5 --key kova-test-api-key
```

```sh
./garage/target/release/garage -c garage.toml server
```
