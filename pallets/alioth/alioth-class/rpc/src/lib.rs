//use alioth_register::Meta;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId,traits::Block as BlockT};
use std::sync::Arc;
use alioth_class_runtime::ClassApi as ClassStorageApi;
use alioth_class::ClassData;
//use sp_core::H256;

#[rpc]
pub trait ClassApi<BlockHash,AccountId> {
	#[rpc(name = "getClass")]
    fn get_class(&self,at:Option<BlockHash>,db_id:Vec<u8>,app_id:Vec<u8>,acc:AccountId)->Result<Vec<ClassData>>;
}

/// A struct that implements the `SumStorageApi`.
pub struct ClassStorage<C, M> {
	// If you have more generics, no need to SumStorage<C, M, N, P, ...>
	// just use a tuple like SumStorage<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> ClassStorage<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

/// Error type of this RPC api.
// pub enum Error {
// 	/// The transaction was not decodable.
// 	DecodeError,
// 	/// The call to runtime failed.
// 	RuntimeError,
// }
//
// impl From<Error> for i64 {
// 	fn from(e: Error) -> i64 {
// 		match e {
// 			Error::RuntimeError => 1,
// 			Error::DecodeError => 2,
// 		}
// 	}
// }

impl<C, Block,AccountId> ClassApi<<Block as BlockT>::Hash,AccountId> for ClassStorage<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	AccountId:sp_api::Encode,
	C::Api: ClassStorageApi<Block,AccountId>,
{
	fn get_class(&self, at: Option<<Block as BlockT>::Hash>,db_id:Vec<u8>,app_id:Vec<u8>,acc:AccountId) -> Result<Vec<ClassData>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_class(&at,db_id,app_id,acc);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
