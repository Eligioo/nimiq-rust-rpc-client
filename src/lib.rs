use isahc::prelude::*;
mod rpc;

#[derive(Debug, Clone)]
pub struct Client {
	host: String,
	id: i32
}

impl Client {
	pub fn new(host: &str) -> Client {
		Client { host: String::from(host), id: 1 }
	}

	pub fn accounts(&self) -> Result<rpc::responses::Accounts, Box<dyn std::error::Error>>{
		let request = rpc::request_builder(String::from("accounts"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<rpc::responses::Accounts>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn block_number(&self) -> Result<rpc::responses::BlockNumber, Box<dyn std::error::Error>>{
		let request = rpc::request_builder(String::from("blockNumber"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<rpc::responses::BlockNumber>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn hashrate(&self) -> Result<rpc::responses::Hashrate, Box<dyn std::error::Error>>{
		let request = rpc::request_builder(String::from("hashrate"), vec!(), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<rpc::responses::Hashrate>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn log(&self, tag: &str, level: &str) -> Result<rpc::responses::Log, Box<dyn std::error::Error>>{
		let a = 2;
		let request = rpc::request_builder(String::from("log"), vec!(String::from(tag), String::from(level)), &a);
		
		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<rpc::responses::Log>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}
}