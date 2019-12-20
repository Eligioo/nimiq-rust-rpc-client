use serde::{ Deserialize };

#[derive(Debug, Deserialize)]
pub struct Accounts {
	pub id: String,
	pub jsonrpc: String,
	pub result: Vec<super::primitives::Account>
}

#[derive(Debug, Deserialize)]
pub struct BlockNumber {
	pub id: String,
	pub jsonrpc: String,
	pub result: i32
}

#[derive(Debug, Deserialize)]
pub struct Consensus {
	pub id: String,
	pub jsonrpc: String,
	pub result: String
}

#[derive(Debug, Deserialize)]
pub struct CreateAccount {
	pub id: String,
	pub jsonrpc: String,
	pub result: super::primitives::Wallet
}

#[derive(Debug, Deserialize)]
pub struct CreateRawTransaction {
	pub id: String,
	pub jsonrpc: String,
	pub result: String
}

#[derive(Debug, Deserialize)]
pub struct GetAccount {
	id: String,
  jsonrpc: String,
	result: super::primitives::Account,
}

#[derive(Debug, Deserialize)]
pub struct GetBalance {
	id: String,
  jsonrpc: String,
	result: i32,
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