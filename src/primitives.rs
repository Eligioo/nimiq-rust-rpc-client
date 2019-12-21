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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GetWork {
	pub data: String,
	pub suffix: String,
	pub target: i64,
	pub algorithm: String
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Transaction {
	pub hash: String,
	#[serde(rename = "blockHash")]
	pub block_hash: String,
	#[serde(rename = "blockNumber")]
	pub block_number: i64,
	pub timestamp: i64,
	pub confirmations: i64,
	#[serde(rename = "transactionIndex")]
	#[serde(skip_deserializing)] //TODO skip deserializing, have to look into making field optional
	transaction_index: ::serde_json::Value,
	pub from: String,
	#[serde(rename = "fromAddress")]
	pub from_address: String,
	pub to: String,
	#[serde(rename = "toAddress")]
	pub to_address: String,
	pub value: i64,
	pub fee: i64,
	pub data: ::serde_json::Value,
	pub flags: i64
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TransactionReceipt {
	#[serde(rename = "transactionHash")]
	pub transaction_hash: String,
	#[serde(rename = "transactionIndex")]
	pub transaction_index: i64,
	#[serde(rename = "blockNumber")]
	pub block_number: i64,
	#[serde(rename = "blockHash")]
	pub block_hash: String,
	pub confirmations: i64,
	pub timestamp: i64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)] //TODO untagged for now, have to look more into this
pub enum TransactionSequence{
	BlockHashes(Vec<String>),
	Transactions(Vec<Transaction>)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutgoingTransaction {
	pub from: &'static str,
	pub to: &'static str,
	pub value: i32,
	pub fee: i32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
	pub id: String,
	pub address: String,
	pub public_key: String
}