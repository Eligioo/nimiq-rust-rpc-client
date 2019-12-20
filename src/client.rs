use isahc::prelude::*;

use super::primitives;
use super::rpc::{ request_builder, RPCRequest, RPCResponse };

#[derive(Debug, Clone)]
pub struct Client {
	host: String,
	id: i32
}

impl Client {
	pub fn new(host: &str) -> Client {
		Client { host: String::from(host), id: 1 }
	}

	pub fn accounts(&self) -> Result<Vec<super::primitives::Account>, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("accounts"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<Vec<primitives::Account>>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn block_number(&self) -> Result<i32, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("blockNumber"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<i32>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn consensus(&self) -> Result<String, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("consensus"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<String>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn create_account(&self) -> Result<primitives::Wallet, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("createAccount"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::Wallet>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn create_raw_transaction(&self, raw_transaction: &primitives::OutgoingTransaction) -> Result<String, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("createRawTransaction"), raw_transaction, &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<String>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_account(&self, id: &str) -> Result<primitives::Account, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("getAccount"), vec!(String::from(id)), &self.id);
		
		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::Account>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_balance(&self, id: &str) -> Result<i32, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("getBalance"), vec!(String::from(id)), &self.id);
		
		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<i32>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_block_by_hash(&self, id: &str, full_transactions: bool) -> Result<primitives::Block, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getBlockByHash"), (id, full_transactions), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::Block>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn hashrate(&self) -> Result<i32, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("hashrate"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<i32>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn log(&self, tag: &str, level: &str) -> Result<bool, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("log"), vec!(String::from(tag), String::from(level)), &self.id);
		
		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<bool>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}
}