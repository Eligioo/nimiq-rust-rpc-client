use serde::{ Deserialize };

#[derive(Debug, Deserialize)]
pub struct Address {
	pub id: String,
	pub address: String
}

#[derive(Debug, Deserialize)]
pub struct Account {
	pub id: String,
	pub address: String,
	pub balance: i32,
	pub r#type: i32
}

#[derive(Debug, Deserialize)]
pub struct Block {
	number: i32,
	hash: String,
	pow: String,
	parent_hash: String,
	nonce: i32,
	body_hash: String,
	account_hash: String,
	miner: String,
	miner_address: String,
	difficulty: String,
	extra_data: String,
	size: i32,
	timestamp: i32,
	transactions: Vec<Transaction>
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
	hash: String,
	block_hash: String,
	block_number: i32,
	timestamp: i32,
	confirmations: i32,
	transaction_index: i32,
	from: String,
	from_address: String,
	to: String,
	to_address: String,
	value: i32,
	fee: i32,
	data: String,
	flags: i32
}

#[derive(Debug, Deserialize)]
pub struct OutgoingTransaction {
	from: Address,
	from_type: i32,
	to: Address,
	to_type: i32,
	value: i32,
	fee: i32,
	data: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
	id: String,
	address: String,
	public_key: String
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
pub struct Consensus {
	pub id: String,
	pub jsonrpc: String,
	pub result: String
}

#[derive(Debug, Deserialize)]
pub struct CreateAccount {
	pub id: String,
	pub jsonrpc: String,
	pub result: Wallet
}

#[derive(Debug, Deserialize)]
pub struct CreateRawTransaction {
	pub id: String,
	pub jsonrpc: String,
	pub result: String
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