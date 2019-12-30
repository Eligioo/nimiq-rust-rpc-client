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

	/// Returns a list of addresses owned by client.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Vector of accounts owned by the client.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.accounts();
	/// ```
	pub fn accounts(&self) -> Result<Vec<super::primitives::Account>, Error>{
		let response = self.send("accounts", ()).json::<RPCResponse<Vec<primitives::Account>>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns the height of most recent block.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// The current block height the client is on.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.block_number();
	/// ```
	pub fn block_number(&self) -> Result<u64, Error>{
		let response = self.send("blockNumber", ()).json::<RPCResponse<u64>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns information on the current consensus state.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// String describing the consensus state. `"established"` is the value for a good state, other values indicate bad state.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.consensus();
	/// ```
	pub fn consensus(&self) -> Result<String, Error>{
		let response = self.send("consensus", ()).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Creates a new account and stores its private key in the client store.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Information on the wallet that was created using the command.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.create_account();
	/// ```
	pub fn create_account(&self) -> Result<primitives::Wallet, Error>{
		let response = self.send("createAccount", ()).json::<RPCResponse<primitives::Wallet>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Creates and signs a transaction without sending it. The transaction can then be send via `sendRawTransaction` without accidentally replaying it.
	///
	/// # Arguments
	///
	/// * `raw_transaction`: `&primitives::OutgoingTransaction`
	/// 
	/// # Returns
	/// 
	/// The Hex-encoded transaction.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let tx = nimiq_rpc::primitives::OutgoingTransaction {
	///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42",
	///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3",
	///    value: 100, //Lunas
	///    fee: 0
	///	};
	/// let result = client.create_raw_transaction(&tx);
	/// ```
	pub fn create_raw_transaction(&self, raw_transaction: &primitives::OutgoingTransaction) -> Result<String, Error>{
		let response = self.send("createRawTransaction", raw_transaction).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns details for the account of given address.
	///
	/// # Arguments
	///
	/// * `String`: Address to check for balance.
	/// 
	/// # Returns
	/// 
	/// Details about the account. Returns the default empty basic account for non-existing accounts.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_account("ad25610feb43d75307763d3f010822a757027429");
	/// ```
	pub fn get_account(&self, id: &str) -> Result<primitives::Account, Error>{
		let response = self.send("getAccount", id).json::<RPCResponse<primitives::Account>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns the balance of the account of given address.
	///
	/// # Arguments
	///
	/// * `String`: Address to check for balance.
	/// 
	/// # Returns
	/// 
	/// Details about the account. The current balance at the specified address (in smalest unit).
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_balance("ad25610feb43d75307763d3f010822a757027429");
	/// ```
	pub fn get_balance(&self, id: &str) -> Result<u64, Error>{
		let response = self.send("getBalance", id).json::<RPCResponse<u64>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns information about a block by hash.
	///
	/// # Arguments
	///
	/// * `String`: Hash of the block to gather information on.
	/// * `Boolean`: If `true` it returns the full transaction objects, if `false` only the hashes of the transactions.
	/// 
	/// # Returns
	/// 
	/// A block object or `null` when no block was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_by_hash("14c91f6d6f3a0b62271e546bb09461231ab7e4d1ddc2c3e1b93de52d48a1da87", false);
	/// ```
	pub fn get_block_by_hash(&self, block_hash: &str, full_transactions: bool) -> Result<primitives::Block, Error>{
		let response = self.send("getBlockByHash", (block_hash, full_transactions)).json::<RPCResponse<primitives::Block>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns information about a block by block number.
	///
	/// # Arguments
	///
	/// * `Int`: The height of the block to gather information on.
	/// * `Boolean`: If `true` it returns the full transaction objects, if `false` only the hashes of the transactions.
	/// 
	/// # Returns
	/// 
	/// A block object or `null` when no block was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_by_number(1234, false);
	/// ```
	pub fn get_block_by_number(&self, block_number: u64, full_transactions: bool) -> Result<primitives::Block, Error>{
		let response = self.send("getBlockByNumber", (block_number, full_transactions)).json::<RPCResponse<primitives::Block>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns a template to build the next block for mining. This will consider pool instructions when connected to a pool.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// A block template object.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_template();
	/// ```
	pub fn get_block_template(&self) -> Result<primitives::FullBlock, Error>{
		let response = self.send("getBlockTemplate", ()).json::<RPCResponse<primitives::FullBlock>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns the number of transactions in a block from a block matching the given block hash.
	///
	/// # Arguments
	///
	/// * `String`: Hash of the block.
	/// 
	/// # Returns
	/// 
	/// Number of transactions in the block found, or `null`, when no block was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_transaction_count_by_hash("dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f");
	/// ```
	pub fn get_block_transaction_count_by_hash(&self, block_hash: &str) -> Result<u16, Error>{
		let response = self.send("getBlockTransactionCountByHash", block_hash).json::<RPCResponse<u16>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns the number of transactions in a block matching the given block number.
	///
	/// # Arguments
	///
	/// * `Int`: Height of the block.
	/// 
	/// # Returns
	/// 
	/// Number of transactions in the block found, or `null`, when no block was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_transaction_count_by_number(76415);
	/// ```
	pub fn get_block_transaction_count_by_number(&self, block_number: u64) -> Result<u16, Error>{
		let response = self.send("getBlockTransactionCountByNumber", block_number).json::<RPCResponse<u16>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns information about a transaction by block hash and transaction index position.
	///
	/// # Arguments
	///
	/// * `String`: Hash of the block containing the transaction
	/// * `Int`: Index of the transaction in the block
	/// 
	/// # Returns
	/// 
	/// A transaction object or `null` when no transaction was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transaction_by_block_hash_and_index("dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f", 20);
	/// ```
	pub fn get_transaction_by_block_hash_and_index(&self, block_hash: &str, index: u64) -> Result<primitives::Transaction, Error>{
		let response = self.send("getTransactionByBlockHashAndIndex", (block_hash, index)).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns information about a transaction by block number and transaction index position.
	///
	/// # Arguments
	///
	/// * `Int`: Height of the block containing the transaction
	/// * `Int`: Index of the transaction in the block
	/// 
	/// # Returns
	/// 
	/// A transaction object or `null` when no transaction was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transaction_by_block_number_and_index(76415, 20);
	/// ```
	pub fn get_transaction_by_block_number_and_index(&self, block_number: u64, index: u16) -> Result<primitives::Transaction, Error>{
		let response = self.send("getTransactionByBlockNumberAndIndex", (block_number, index)).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns the information about a transaction requested by transaction hash.
	///
	/// # Arguments
	///
	/// * `String`: Hash of a transaction
	/// 
	/// # Returns
	/// 
	/// A transaction object or `null` when no transaction was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transaction_by_hash("465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554");
	/// ```
	pub fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<primitives::Transaction, Error>{
		let response = self.send("getTransactionByHash", transaction_hash).json::<RPCResponse<primitives::Transaction>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns the receipt of a transaction by transaction hash.
	/// `Note` That the receipt is not available for pending transactions.
	///
	/// # Arguments
	///
	/// * `String`: Hash of a transaction
	/// 
	/// # Returns
	/// 
	/// A transaction receipt object, or `null` when no receipt was found
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transaction_receipt("465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554");
	/// ```
	pub fn get_transaction_receipt(&self, transaction_hash: &str) -> Result<primitives::TransactionReceipt, Error>{
		let response = self.send("getTransactionReceipt", transaction_hash).json::<RPCResponse<primitives::TransactionReceipt>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns the latest transactions successfully performed by or for an address.
	/// `Note` That this information might change when blocks are rewinded on the local state due to forks.
	///
	/// # Arguments
	///
	/// * `String`: Address of which transactions should be gathered.
	/// * `Int`: Number of transactions that shall be returned.
	/// 
	/// # Returns
	/// 
	/// Vector of transactions linked to the requested address.
	/// `Note` The array will not contain more than the requested amount of transactions, but might contain less, even when more transactions happened. Any interpretation of the length of this array might result in worng assumptions.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transactions_by_address("NQ69 9A4A MB83 HXDQ 4J46 BH5R 4JFF QMA9 C3GN", 10);
	/// ```
	pub fn get_transactions_by_address(&self, address: &str, amount: u16) -> Result<Vec<primitives::Transaction>, Error>{
		let response = self.send("getTransactionsByAddress", (address, amount)).json::<RPCResponse<Vec<primitives::Transaction>>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns instructions to mine the next block. This will consider pool instructions when connected to a pool.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Mining work instructions
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_work();
	/// ```
	pub fn get_work(&self) -> Result<primitives::GetWork, Error>{
		let response = self.send("getWork", ()).json::<RPCResponse<primitives::GetWork>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns the number of hashes per second that the node is mining with.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Number of hashes per second.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.hashrate();
	/// ```
	pub fn hashrate(&self) -> Result<f64, Error>{
		let response = self.send("hashrate", ()).json::<RPCResponse<f64>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Sets the log level of the node.
	///
	/// # Arguments
	///
	/// * `String`: Tag: If `'*'` the log level is set globally, otherwise the log level is applied only on this tag.
	/// * `String`: Minimum log level to display. (Valid options: `trace`, `verbose`, `debug`, `info`, `warn`, `error`, `assert`)
	/// 
	/// # Returns
	/// 
	/// `true`
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.log("*", "log");
	/// ```
	pub fn log(&self, tag: &str, level: &str) -> Result<bool, Error>{
		let response = self.send("log", (tag, level)).json::<RPCResponse<bool>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn mempool_content(&self) -> Result<Vec<String>, Error>{
		let response = self.send("mempoolContent", ()).json::<RPCResponse<Vec<String>>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn miner_address(&self) -> Result<String, Error>{
		let response = self.send("minerAddress", ()).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn miner_threads(&self) -> Result<u8, Error>{
		let response = self.send("minerThreads", ()).json::<RPCResponse<u8>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn miner_threads_with_update(&self, threads: u16) -> Result<u16, Error>{
		let response = self.send("minerThreads", threads).json::<RPCResponse<u16>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn min_fee_per_byte(&self) -> Result<u32, Error>{
		let response = self.send("minFeePerByte", ()).json::<RPCResponse<u32>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn min_fee_per_byte_with_update(&self, fee: u32) -> Result<u32, Error>{
		let response = self.send("minFeePerByte", fee).json::<RPCResponse<u32>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns `true` if client is actively mining new blocks.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Returns `true` if the client is mining, otherwise `false`.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.mining();
	/// ```
	pub fn mining(&self) -> Result<bool, Error>{
		let response = self.send("mining", ()).json::<RPCResponse<bool>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns number of peers currently connected to the client.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Integer of the number of connected peers.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.peer_count();
	/// ```
	pub fn peer_count(&self) -> Result<i8, Error>{
		let response = self.send("peerCount", ()).json::<RPCResponse<i8>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn peer_list(&self) -> Result<Vec<primitives::PeerList>, Error>{
		let response = self.send("peerList", ()).json::<RPCResponse<Vec<primitives::PeerList>>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn peer_state(&self, peer_address: &str) -> Result<primitives::PeerState, Error>{
		let response = self.send("peerState", peer_address).json::<RPCResponse<primitives::PeerState>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn peer_state_with_update(&self, peer_address: &str, set: &str) -> Result<primitives::PeerState, Error>{
		let response = self.send("peerState", (peer_address, set)).json::<RPCResponse<primitives::PeerState>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn pool_confirmed_balance(&self) -> Result<u64, Error>{
		let response = self.send("poolConfirmedBalance", ()).json::<RPCResponse<u64>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	pub fn pool_connection_state(&self) -> Result<u8, Error>{
		let response = self.send("poolConnectionState", ()).json::<RPCResponse<u8>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Sends a signed message call transaction or a contract creation, if the data field contains code.
	///
	/// # Arguments
	///
	/// * `String`: The hex encoded signed transaction
	/// 
	/// # Returns
	/// 
	/// The Hex-encoded transaction hash.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let tx = nimiq_rpc::primitives::OutgoingTransaction {
	///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42",
	///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3",
	///    value: 100, //Lunas
	///    fee: 0
	///	};
	/// let result = client.create_raw_transaction(&tx);
	/// let hash = client.send_raw_transaction(&result);
	/// ```
	pub fn send_raw_transaction(&self, transaction_hash: &str) -> Result<String, Error>{
		let response = self.send("sendRawTransaction", transaction_hash).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Creates new message call transaction or a contract creation, if the data field contains code.
	///
	/// # Arguments
	///
	/// * `OutgoingTransaction`: The transaction object
	/// 
	/// # Returns
	/// 
	/// The Hex-encoded transaction hash.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let tx = nimiq_rpc::primitives::OutgoingTransaction {
	///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42",
	///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3",
	///    value: 100, //Lunas
	///    fee: 0
	///	};
	/// let result = client.send_transaction(&tx);
	/// ```
	pub fn send_transaction(&self, transaction: &primitives::OutgoingTransaction) -> Result<String, Error>{
		let response = self.send("sendTransaction", transaction).json::<RPCResponse<String>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Submits a block to the node. When the block is valid, the node will forward it to other nodes in the network.
	///
	/// # Arguments
	///
	/// * `String`: Hex-encoded full block (including header, interlink and body). When submitting work from `getWork`, remember to include the `suffix`.
	/// 
	/// # Returns
	/// 
	/// Nothing
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.submit_block("0da1....234");
	/// ```
	pub fn submit_block(&self, full_block: &str) -> Result<(), Error>{
		let response = self.send("submitBlock", full_block).json::<RPCResponse<()>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}

	/// Returns an object with data about the sync status or `false`.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// An object with sync status data or `false`, when not syncing
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.syncing();
	/// ```
	pub fn syncing(&self) -> Result<primitives::Syncing, Error>{
		let response = self.send("syncing", ()).json::<RPCResponse<primitives::Syncing>>();
		match response {
			Ok(RPCResponse::Success(v)) => Ok(v.result),
			Ok(RPCResponse::Error(v)) => Err(v.error),
			Err(_) => {
				Err(
					Error {
						code: String::from("-32700"),
						message: "Couldn't deserialize response from server!".to_string()
					}
				)
			}
		}
	}
}