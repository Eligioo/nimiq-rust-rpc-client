use isahc::prelude::*;

use super::primitives;


#[derive(Debug, serde::Deserialize)]
struct RPCResponse <T> {
	pub id: String,
	pub jsonrpc: String,
	pub result: T
}

#[derive(Debug)]
pub struct Client {
	host: String,
	id: i32
}

impl Client {
	pub fn new(host: &str) -> Client {
		Client { host: String::from(host), id: 1 }
	}

	fn send<T: serde::Serialize>(&self, method: &str, params: T) -> Response<isahc::Body> {
		println!("{:?}", format!(r#"{{"jsonrpc":"2.0","method":"{}","params":{},"id":"1"}}"#, 
		method, 
		format!("{:?}", serde_json::to_string_pretty(&params).unwrap())
		));
		isahc::post(&self.host, format!(r#"{{"jsonrpc":"2.0","method":"{}","params":{},"id":"1"}}"#, 
				method, 
				format!("{:?}", serde_json::to_vec(&params).unwrap())
			))
			.unwrap()
	}

	pub fn accounts(&self) -> Result<Vec<super::primitives::Account>, Box<dyn std::error::Error>>{
		let response = self.send::<&str>("accounts", "").json::<RPCResponse<Vec<primitives::Account>>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn block_number(&self) -> Result<i32, Box<dyn std::error::Error>>{
		let response = self.send("blockNumber", ()).json::<RPCResponse<i32>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn consensus(&self) -> Result<String, Box<dyn std::error::Error>>{
		let response = self.send("consensus", ()).json::<RPCResponse<String>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn create_account(&self) -> Result<primitives::Wallet, Box<dyn std::error::Error>>{
		let response = self.send("createAccount", ()).json::<RPCResponse<primitives::Wallet>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn create_raw_transaction(&self, raw_transaction: &primitives::OutgoingTransaction) -> Result<String, Box<dyn std::error::Error>>{
		let response = self.send("createRawTransaction", (raw_transaction)).json::<RPCResponse<String>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_account(&self, id: &str) -> Result<primitives::Account, Box<dyn std::error::Error>>{
		let response = self.send("getAccount", id).json::<RPCResponse<primitives::Account>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_balance(&self, id: &str) -> Result<i32, Box<dyn std::error::Error>>{
		let response = self.send("getBalance", id).json::<RPCResponse<i32>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_block_by_hash(&self, block_hash: &str, full_transactions: bool) -> Result<primitives::Block, Box<dyn std::error::Error>>{
		let response = self.send("getBlockByHash", (block_hash, full_transactions)).json::<RPCResponse<primitives::Block>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_block_by_number(&self, block_number: i32, full_transactions: bool) -> Result<primitives::Block, Box<dyn std::error::Error>>{
		let response = self.send("getBlockByNumber", (block_number, full_transactions)).json::<RPCResponse<primitives::Block>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_block_template(&self) -> Result<primitives::FullBlock, Box<dyn std::error::Error>>{
		let response = self.send("getBlockTemplate", ()).json::<RPCResponse<primitives::FullBlock>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_block_transaction_count_by_hash(&self, block_hash: &str) -> Result<i32, Box<dyn std::error::Error>>{
		let response = self.send("getBlockTransactionCountByHash", block_hash).json::<RPCResponse<i32>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_block_transaction_count_by_number(&self, block_number: i32) -> Result<i32, Box<dyn std::error::Error>>{
		let response = self.send("getBlockTransactionCountByNumber", block_number).json::<RPCResponse<i32>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transaction_by_block_hash_and_index(&self, block_hash: &str, index: i32) -> Result<primitives::Transaction, Box<dyn std::error::Error>>{
		let response = self.send("getTransactionByBlockHashAndIndex", (block_hash, index)).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transaction_by_block_number_and_index(&self, block_number: i32, index: i32) -> Result<primitives::Transaction, Box<dyn std::error::Error>>{
		let response = self.send("getTransactionByBlockNumberAndIndex", (block_number, index)).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<primitives::Transaction, Box<dyn std::error::Error>>{
		let response = self.send("getTransactionByHash", transaction_hash).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transaction_receipt(&self, transaction_hash: &str) -> Result<primitives::TransactionReceipt, Box<dyn std::error::Error>>{
		let response = self.send("getTransactionReceipt", transaction_hash).json::<RPCResponse<primitives::TransactionReceipt>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transactions_by_address(&self, address: &str, amount: i32) -> Result<Vec<primitives::Transaction>, Box<dyn std::error::Error>>{
		let response = self.send("getTransactionsByAddress", (address, amount)).json::<RPCResponse<Vec<primitives::Transaction>>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_work(&self) -> Result<primitives::GetWork, Box<dyn std::error::Error>>{
		let response = self.send("getWork", ()).json::<RPCResponse<primitives::GetWork>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn hashrate(&self) -> Result<i32, Box<dyn std::error::Error>>{
		let response = self.send("hashrate", ()).json::<RPCResponse<i32>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn log(&self, tag: &str, level: &str) -> Result<bool, Box<dyn std::error::Error>>{
		let response = self.send("log", (tag, level)).json::<RPCResponse<bool>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn mempool_content(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>{
		let response = self.send("mempoolContent", ()).json::<RPCResponse<Vec<String>>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn miner_address(&self) -> Result<String, Box<dyn std::error::Error>>{
		let response = self.send("minerAddress", ()).json::<RPCResponse<String>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn miner_threads(&self) -> Result<u8, Box<dyn std::error::Error>>{
		let response = self.send("minerThreads", ()).json::<RPCResponse<u8>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn miner_threads_with_update(&self, threads: i16) -> Result<i16, Box<dyn std::error::Error>>{
		let response = self.send("minerThreads", threads).json::<RPCResponse<i16>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn min_fee_per_byte(&self) -> Result<u32, Box<dyn std::error::Error>>{
		let response = self.send("minFeePerByte", ()).json::<RPCResponse<u32>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn min_fee_per_byte_with_update(&self, fee: u32) -> Result<u32, Box<dyn std::error::Error>>{
		let response = self.send("minFeePerByte", fee).json::<RPCResponse<u32>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn mining(&self) -> Result<bool, Box<dyn std::error::Error>>{
		let response = self.send("mining", ()).json::<RPCResponse<bool>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn peer_count(&self) -> Result<i8, Box<dyn std::error::Error>>{
		let response = self.send("peerCount", ()).json::<RPCResponse<i8>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn peer_list(&self) -> Result<Vec<primitives::PeerList>, Box<dyn std::error::Error>>{
		let response = self.send("peerList", ()).json::<RPCResponse<Vec<primitives::PeerList>>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn peer_state(&self, peer_address: &str) -> Result<primitives::PeerState, Box<dyn std::error::Error>>{
		let response = self.send("peerState", peer_address).json::<RPCResponse<primitives::PeerState>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn peer_state_with_update(&self, peer_address: &str, set: &str) -> Result<primitives::PeerState, Box<dyn std::error::Error>>{
		let response = self.send("peerState", (peer_address, set)).json::<RPCResponse<primitives::PeerState>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn pool_confirmed_balance(&self) -> Result<u64, Box<dyn std::error::Error>>{
		let response = self.send("poolConfirmedBalance", ()).json::<RPCResponse<u64>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn pool_connection_state(&self) -> Result<u8, Box<dyn std::error::Error>>{
		let response = self.send("poolConnectionState", ()).json::<RPCResponse<u8>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn send_raw_transaction(&self, transaction_hash: &str) -> Result<String, Box<dyn std::error::Error>>{
		let response = self.send("sendRawTransaction", transaction_hash).json::<RPCResponse<String>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn send_transaction(&self, transaction: &primitives::OutgoingTransaction) -> Result<String, Box<dyn std::error::Error>>{
		let response = self.send("sendTransaction", transaction).json::<RPCResponse<String>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn submit_block(&self, full_block: &str) -> Result<(), Box<dyn std::error::Error>>{
		let response = self.send("submitBlock", full_block).json::<RPCResponse<()>>();
		match response {
			Ok(_) => Ok(()),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn syncing(&self) -> Result<primitives::Syncing, Box<dyn std::error::Error>>{
		let response = self.send("syncing", ()).json::<RPCResponse<primitives::Syncing>>();
		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}
}