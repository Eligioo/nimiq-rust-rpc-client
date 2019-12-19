use serde::{ Serialize };

#[derive(Debug, Serialize)]
pub struct RPCRequest {
	jsonrpc: String,
	method: String,
	params: Vec<String>,
	id: String
}

pub fn request_builder(method: String, params: Vec<String>, id: &i32) -> RPCRequest {
	RPCRequest {
		jsonrpc: String::from("2.0"),
		method: method,
		params: params,
		id: id.to_string()
	}
}