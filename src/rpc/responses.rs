use serde::{ Deserialize };

#[derive(Debug, Deserialize)]
pub struct Account {
	pub id: String,
	pub address: String,
	pub balance: i32,
	pub r#type: i32
}

#[derive(Debug, Deserialize)]
pub struct Accounts {
	pub id: String,
	pub jsonrpc: String,
	pub result: Vec<Account>
}

#[derive(Debug, Deserialize)]
pub struct BlockNumber {
	pub id: String,
	pub jsonrpc: String,
	pub result: i32
}

#[derive(Debug, Deserialize)]
pub struct Hashrate {
	pub id: String,
	pub jsonrpc: String,
	pub result: i32
}

#[derive(Debug, Deserialize)]
pub struct Log{
  pub id: String,
	pub jsonrpc: String,
  pub result: bool
}