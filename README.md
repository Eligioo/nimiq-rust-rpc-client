# Nimiq RPC client
![https://img.shields.io/crates/v/nimiq_rpc?label=nimiq_rpc](https://img.shields.io/crates/v/nimiq_rpc?label=nimiq_rpc)

A Nimiq RPC client for the Rust programming language


# Example:
```rust
use nimiq_rpc::Client;

fn main() {
	let client = Client::new("http://seed-host.com:8648/");
	
	println!("{:?}", client.accounts().unwrap());
	println!("{:?}", client.block_number().unwrap());
	println!("{:?}", client.hashrate().unwrap());
	println!("{:?}", client.log("*", "log").unwrap());
}
```

# Installation:

```
[dependencies]
nimiq_rpc = "0.1.0"
```
