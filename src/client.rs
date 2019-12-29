use isahc::prelude::*;

use super::primitives;

#[derive(Debug, serde::Serialize)]
pub struct RPCRequest <T> {
	jsonrpc: String,
	method: String,
	params: T,
	id: String
}

#[derive(Debug, Default, serde_derive::Deserialize)]
pub struct RPCError {
	pub jsonrpc: String,
	#[serde(skip_deserializing)]
	pub error: Error,
	pub id: String
}

#[derive(Debug, Default, serde_derive::Deserialize)]
pub struct Error {
	pub code: String,
	pub message: String
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum RPCResponse <T> {
	Success(RPCSuccess<T>),
	Error(RPCError)
}

#[derive(Debug, serde::Deserialize)]
struct RPCSuccess <T> {
	pub id: String,
	pub jsonrpc: String,
	pub result: T
}

#[derive(Debug)]
pub struct Client {
	host: String,
	id: u64
}

impl Client {
	pub fn new(host: &str) -> Client {
		Client { host: String::from(host), id: 1 }
	}

	fn send<T: serde::Serialize>(&self, method: &str, params: T) -> Response<isahc::Body> {
		let body = serde_json::to_vec(&RPCRequest {
			jsonrpc: String::from("2.0"),
			method: String::from(method),
			id: String::from("1"),
			params: &params
		}).unwrap();

		isahc::post(&self.host, body)
		.unwrap()
	}

	pub fn accounts(&self) -> Result<Vec<super::primitives::Account>, Error>{
		let response = self.send("accounts", ()).json::<RPCResponse<Vec<primitives::Account>>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn block_number(&self) -> Result<u64, Error>{
		let response = self.send("blockNumber", ()).json::<RPCResponse<u64>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn consensus(&self) -> Result<String, Error>{
		let response = self.send("consensus", ()).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn create_account(&self) -> Result<primitives::Wallet, Error>{
		let response = self.send("createAccount", ()).json::<RPCResponse<primitives::Wallet>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn create_raw_transaction(&self, raw_transaction: &primitives::OutgoingTransaction) -> Result<String, Error>{
		let response = self.send("createRawTransaction", raw_transaction).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_account(&self, id: &str) -> Result<primitives::Account, Error>{
		let response = self.send("getAccount", id).json::<RPCResponse<primitives::Account>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_balance(&self, id: &str) -> Result<u64, Error>{
		let response = self.send("getBalance", id).json::<RPCResponse<u64>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_block_by_hash(&self, block_hash: &str, full_transactions: bool) -> Result<primitives::Block, Error>{
		let response = self.send("getBlockByHash", (block_hash, full_transactions)).json::<RPCResponse<primitives::Block>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_block_by_number(&self, block_number: u64, full_transactions: bool) -> Result<primitives::Block, Error>{
		let response = self.send("getBlockByNumber", (block_number, full_transactions)).json::<RPCResponse<primitives::Block>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_block_template(&self) -> Result<primitives::FullBlock, Error>{
		let response = self.send("getBlockTemplate", ()).json::<RPCResponse<primitives::FullBlock>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_block_transaction_count_by_hash(&self, block_hash: &str) -> Result<u16, Error>{
		let response = self.send("getBlockTransactionCountByHash", block_hash).json::<RPCResponse<u16>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_block_transaction_count_by_number(&self, block_number: u64) -> Result<u16, Error>{
		let response = self.send("getBlockTransactionCountByNumber", block_number).json::<RPCResponse<u16>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_transaction_by_block_hash_and_index(&self, block_hash: &str, index: u64) -> Result<primitives::Transaction, Error>{
		let response = self.send("getTransactionByBlockHashAndIndex", (block_hash, index)).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_transaction_by_block_number_and_index(&self, block_number: u64, index: u16) -> Result<primitives::Transaction, Error>{
		let response = self.send("getTransactionByBlockNumberAndIndex", (block_number, index)).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<primitives::Transaction, Error>{
		let response = self.send("getTransactionByHash", transaction_hash).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_transaction_receipt(&self, transaction_hash: &str) -> Result<primitives::TransactionReceipt, Error>{
		let response = self.send("getTransactionReceipt", transaction_hash).json::<RPCResponse<primitives::TransactionReceipt>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_transactions_by_address(&self, address: &str, amount: u16) -> Result<Vec<primitives::Transaction>, Error>{
		let response = self.send("getTransactionsByAddress", (address, amount)).json::<RPCResponse<Vec<primitives::Transaction>>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn get_work(&self) -> Result<primitives::GetWork, Error>{
		let response = self.send("getWork", ()).json::<RPCResponse<primitives::GetWork>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn hashrate(&self) -> Result<u64, Error>{
		let response = self.send("hashrate", ()).json::<RPCResponse<u64>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn log(&self, tag: &str, level: &str) -> Result<bool, Error>{
		let response = self.send("log", (tag, level)).json::<RPCResponse<bool>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn mempool_content(&self) -> Result<Vec<String>, Error>{
		let response = self.send("mempoolContent", ()).json::<RPCResponse<Vec<String>>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn miner_address(&self) -> Result<String, Error>{
		let response = self.send("minerAddress", ()).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn miner_threads(&self) -> Result<u8, Error>{
		let response = self.send("minerThreads", ()).json::<RPCResponse<u8>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn miner_threads_with_update(&self, threads: u16) -> Result<u16, Error>{
		let response = self.send("minerThreads", threads).json::<RPCResponse<u16>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn min_fee_per_byte(&self) -> Result<u32, Error>{
		let response = self.send("minFeePerByte", ()).json::<RPCResponse<u32>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn min_fee_per_byte_with_update(&self, fee: u32) -> Result<u32, Error>{
		let response = self.send("minFeePerByte", fee).json::<RPCResponse<u32>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn mining(&self) -> Result<bool, Error>{
		let response = self.send("mining", ()).json::<RPCResponse<bool>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn peer_count(&self) -> Result<i8, Error>{
		let response = self.send("peerCount", ()).json::<RPCResponse<i8>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn peer_list(&self) -> Result<Vec<primitives::PeerList>, Error>{
		let response = self.send("peerList", ()).json::<RPCResponse<Vec<primitives::PeerList>>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn peer_state(&self, peer_address: &str) -> Result<primitives::PeerState, Error>{
		let response = self.send("peerState", peer_address).json::<RPCResponse<primitives::PeerState>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn peer_state_with_update(&self, peer_address: &str, set: &str) -> Result<primitives::PeerState, Error>{
		let response = self.send("peerState", (peer_address, set)).json::<RPCResponse<primitives::PeerState>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn pool_confirmed_balance(&self) -> Result<u64, Error>{
		let response = self.send("poolConfirmedBalance", ()).json::<RPCResponse<u64>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn pool_connection_state(&self) -> Result<u8, Error>{
		let response = self.send("poolConnectionState", ()).json::<RPCResponse<u8>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn send_raw_transaction(&self, transaction_hash: &str) -> Result<String, Error>{
		let response = self.send("sendRawTransaction", transaction_hash).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn send_transaction(&self, transaction: &primitives::OutgoingTransaction) -> Result<String, Error>{
		let response = self.send("sendTransaction", transaction).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn submit_block(&self, full_block: &str) -> Result<(), Error>{
		let response = self.send("submitBlock", full_block).json::<RPCResponse<()>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}

	pub fn syncing(&self) -> Result<primitives::Syncing, Error>{
		let response = self.send("syncing", ()).json::<RPCResponse<primitives::Syncing>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => panic!("Couldn't deserialize response from server!")
		}
	}
}