# nimiq-rust-rpc-client
A Nimiq RPC client for the Rust Programming language


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