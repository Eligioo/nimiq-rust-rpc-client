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
		let request = request_builder(String::from("accounts"), (), &self.id);

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
		let request = request_builder(String::from("blockNumber"), (), &self.id);

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
		let request = request_builder(String::from("consensus"), (), &self.id);

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
		let request = request_builder(String::from("createAccount"), (), &self.id);

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

	pub fn get_block_by_hash(&self, block_hash: &str, full_transactions: bool) -> Result<primitives::Block, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getBlockByHash"), (block_hash, full_transactions), &self.id);

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

	pub fn get_block_by_number(&self, block_number: i32, full_transactions: bool) -> Result<primitives::Block, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getBlockByNumber"), (block_number, full_transactions), &self.id);

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

	pub fn get_block_template(&self) -> Result<primitives::FullBlock, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getBlockTemplate"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::FullBlock>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_block_transaction_count_by_hash(&self, block_hash: &str) -> Result<i32, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getBlockTransactionCountByHash"), block_hash, &self.id);

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

	pub fn get_block_transaction_count_by_number(&self, block_number: i32) -> Result<i32, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getBlockTransactionCountByNumber"), block_number, &self.id);

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

	pub fn get_transaction_by_block_hash_and_index(&self, block_hash: &str, index: i32) -> Result<primitives::Transaction, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getTransactionByBlockHashAndIndex"), (block_hash, index), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::Transaction>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transaction_by_block_number_and_index(&self, block_number: i32, index: i32) -> Result<primitives::Transaction, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getTransactionByBlockNumberAndIndex"), (block_number, index), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::Transaction>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<primitives::Transaction, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getTransactionByHash"), transaction_hash, &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::Transaction>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transaction_receipt(&self, transaction_hash: &str) -> Result<primitives::TransactionReceipt, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getTransactionReceipt"), transaction_hash, &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::TransactionReceipt>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_transactions_by_address(&self, address: &str, amount: i32) -> Result<Vec<primitives::Transaction>, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getTransactionsByAddress"), (address, amount), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<Vec<primitives::Transaction>>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn get_work(&self) -> Result<primitives::GetWork, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("getWork"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::GetWork>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn hashrate(&self) -> Result<i32, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("hashrate"), (), &self.id);

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

	pub fn mempool_content(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("mempoolContent"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<Vec<String>>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn miner_address(&self) -> Result<String, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("minerAddress"), (), &self.id);

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

	pub fn miner_threads(&self) -> Result<u8, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("minerThreads"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<u8>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn miner_threads_with_update(&self, threads: i16) -> Result<i16, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("minerThreads"), threads, &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<i16>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn min_fee_per_byte(&self) -> Result<u32, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("minFeePerByte"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<u32>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn min_fee_per_byte_with_update(&self, fee: u32) -> Result<u32, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("minFeePerByte"), fee, &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<u32>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn mining(&self) -> Result<bool, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("mining"), (), &self.id);

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

	pub fn peer_count(&self) -> Result<i8, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("peerCount"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<i8>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn peer_list(&self) -> Result<Vec<primitives::PeerList>, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("peerList"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<Vec<primitives::PeerList>>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn peer_state(&self, peer_address: &str) -> Result<primitives::PeerState, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("peerState"), peer_address, &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::PeerState>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn peer_state_with_update(&self, peer_address: &str, set: &str) -> Result<primitives::PeerState, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("peerState"), (peer_address, set), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::PeerState>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn pool_confirmed_balance(&self) -> Result<u64, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("poolConfirmedBalance"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<u64>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn pool_connection_state(&self) -> Result<u8, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("poolConnectionState"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<u8>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn send_raw_transaction(&self, transaction_hash: &str) -> Result<String, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("sendRawTransaction"), transaction_hash, &self.id);

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

	pub fn send_transaction(&self, transaction: &primitives::OutgoingTransaction) -> Result<String, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("sendTransaction"), transaction, &self.id);

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

	pub fn submit_block(&self, full_block: &str) -> Result<(), Box<dyn std::error::Error>>{
		let request = request_builder(String::from("submitWork"), full_block, &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<()>();

		match response {
			Ok(v) => Ok(v),
			Err(e) => panic!(e.to_string())
		}
	}

	pub fn syncing(&self) -> Result<primitives::Syncing, Box<dyn std::error::Error>>{
		let request = request_builder(String::from("syncing"), (), &self.id);

		let response = Request::post(&self.host)
			.header("content-type", "application/json")
			.body(serde_json::to_vec(&request)?)?
			.send()?
			.json::<RPCResponse<primitives::Syncing>>();

		match response {
			Ok(v) => Ok(v.result),
			Err(e) => panic!(e.to_string())
		}
	}
}