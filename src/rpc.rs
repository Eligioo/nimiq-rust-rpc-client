use serde::{ Serialize };

#[derive(Debug, Serialize)]
pub struct RPCRequest <T> {
	jsonrpc: String,
	method: String,
	params: T,
	id: String
}

pub fn request_builder<T>(method: String, params: T, id: &i32) -> RPCRequest<T> {
	RPCRequest {
		jsonrpc: String::from("2.0"),
		method: method,
		params: params,
		id: id.to_string()
	}
}