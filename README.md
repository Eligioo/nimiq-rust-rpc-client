# Nimiq RPC client
[![Crates.io](https://img.shields.io/crates/v/nimiq_rpc.svg)](https://crates.io/crates/nimiq_rpc)
[![Crates.io](https://img.shields.io/crates/d/nimiq_rpc.svg)](https://crates.io/crates/nimiq_rpc)


[![Donate NIM](https://www.nimiq.com/accept-donations/img/donationBtnImg/gold-small.svg)](https://safe.nimiq.com/#_request/NQ7461S82FD3RVPGHU091Y5777E6BL38TQH2_)

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

# What is Nimiq?

Nimiq is a decentralized, censorship-resistant payment protocol native to the web, with its own diverse ecosystem of apps. The native NIM token is transacted within Nimiq as a store and transfer of value: it acts as digital cash. The cutting-edge, browser-first blockchain approach means that users directly connect to the blockchain with nothing more than a browser. Therefore anyone with an up-to-date browser can join the payment network directly, pay and accept payments without having to install software or rely on unnecessary intermediaries. This gives Nimiq its ‘it just works’ characteristic, which is further strengthened by an ethos of simplicity and ease of use. NIM is designed to be a cryptocurrency used by the masses.

[Read more at nimiq.com](nimiq.com)

# Installation:

```
[dependencies]
nimiq_rpc = "0.1.1"
```
