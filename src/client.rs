use isahc::prelude::*;

use super::primitives;
use super::responses;
use super::rpc::{ request_builder, RPCRequest };

#[derive(Debug, Clone)]
pub struct Client {
	host: String,
	id: i32
}

impl Client {
	pub fn new(host: &str) -> Client {
		Client { host: String::from(host), id: 1 }
	}

	pub fn accounts(&self) -> Result<responses::Accounts, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("accounts"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<responses::Accounts>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn block_number(&self) -> Result<responses::BlockNumber, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("blockNumber"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<responses::BlockNumber>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn consensus(&self) -> Result<responses::Consensus, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("consensus"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<responses::Consensus>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn create_account(&self) -> Result<responses::CreateAccount, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("createAccount"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<responses::CreateAccount>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn create_raw_transaction(&self, raw_transaction: &primitives::OutgoingTransaction) -> Result<responses::CreateRawTransaction, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("createRawTransaction"), raw_transaction, &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<responses::CreateRawTransaction>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_account(&self, id: &str) -> Result<responses::GetAccount, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("getAccount"), vec!(String::from(id)), &self.id);
		
		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<responses::GetAccount>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn hashrate(&self) -> Result<responses::Hashrate, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("hashrate"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<responses::Hashrate>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn log(&self, tag: &str, level: &str) -> Result<responses::Log, Box<dyn std::error::Error>>{
		let request: RPCRequest<Vec<String>> = request_builder(String::from("log"), vec!(String::from(tag), String::from(level)), &self.id);
		
		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<responses::Log>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}
}