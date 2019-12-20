use serde::{ Deserialize, Serialize };

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
#[serde(rename_all = "camelCase")]
#[serde(untagged)] //TODO untagged for now, have to look more into this
pub enum TransactionSequence{
	TransactionHashSequence(Vec<String>),
	TransactionSequence(Vec<Transaction>)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
	pub number: i32,
	pub hash: String,
	pub pow: String,
	pub parent_hash: String,
	pub nonce: i64,
	pub body_hash: String,
	pub accounts_hash: String,
	pub miner: String,
	pub miner_address: String,
	pub difficulty: String,
	pub extra_data: String,
	pub size: i32,
	pub timestamp: i32,
	pub transactions: TransactionSequence
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Transaction {
	hash: String,
	#[serde(rename = "blockHash")]
	block_hash: String,
	#[serde(rename = "blockNumber")]
	block_number: i64,
	timestamp: i64,
	confirmations: i64,
	#[serde(rename = "transactionIndex")]
	transaction_index: i64,
	from: String,
	#[serde(rename = "fromAddress")]
	from_address: String,
	to: String,
	#[serde(rename = "toAddress")]
	to_address: String,
	value: i64,
	fee: i64,
	data: ::serde_json::Value,
	flags: i64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutgoingTransaction {
	pub from: &'static str,
	pub to: &'static str,
	pub value: i32,
	pub fee: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
	id: String,
	address: String,
	public_key: String
}