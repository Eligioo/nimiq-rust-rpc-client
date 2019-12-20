use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize)]
pub struct RPCRequest <T> {
	jsonrpc: String,
	method: String,
	params: T,
	id: String
}

#[derive(Debug, Deserialize)]
pub struct RPCResponse <T> {
	pub id: String,
	pub jsonrpc: String,
	pub result: T
}

pub fn request_builder<T>(method: String, params: T, id: &i32) -> RPCRequest<T> {
	RPCRequest {
		jsonrpc: String::from("2.0"),
		method: method,
		params: params,
		id: id.to_string()
	}
}