//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;
use register_rpc::{RegisterStorage,RegisterApi};
use db_rpc::{DbApi,DbStorage};
use users_rpc::{UserApi,UserStorage};
use node_template_runtime::{opaque::Block, AccountId, Balance, Index};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{Error as BlockChainError, HeaderMetadata, HeaderBackend};
use sp_block_builder::BlockBuilder;
pub use sc_rpc_api::DenyUnsafe;
use sp_transaction_pool::TransactionPool;
use class_rpc::{ClassApi,ClassStorage};
use ob_rpc::{ObApi,ObStorage};

//use sp_runtime::{OpaqueExtrinsic,traits::BlakeTwo256};


/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error=BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
//	C::Api:alioth_rpc_runtime::RegisterApi<sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>>,
	P: TransactionPool + 'static,
	C::Api: alioth_rpc_runtime::RegisterApi<Block,AccountId>,
	C::Api:alioth_db_runtime::DbApi<Block>,
	C::Api:alioth_users_runtime::UserApi<Block,AccountId>,
	C::Api:alioth_class_runtime::ClassApi<Block,AccountId>,
	C::Api:alioth_ob_runtime::ObApi<Block,AccountId>,

{
	use substrate_frame_rpc_system::{FullSystem, SystemApi};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		client,
		pool,
		deny_unsafe,
	} = deps;

	io.extend_with(
		SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe))
	);

	io.extend_with(
		TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone()))
	);

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));



	//register api integration
	io.extend_with(RegisterApi::to_delegate(RegisterStorage::new(client.clone())));

	//database api integration

	io.extend_with(DbApi::to_delegate(DbStorage::new(client.clone())));

	//users api integration
	io.extend_with(UserApi::to_delegate(UserStorage::new(client.clone())));

	//class api integration
	io.extend_with(ClassApi::to_delegate(ClassStorage::new(client.clone())));

	//object api integration

	io.extend_with(ObApi::to_delegate(ObStorage::new(client.clone())));
	io
}
