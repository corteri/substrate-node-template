#![cfg_attr(not(feature="std"),no_std)]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(clippy::clippy::too_many_arguments)]
use sp_std::vec::Vec;
use alioth_class::ClassData;

sp_api::decl_runtime_apis!{
    pub trait ClassApi<AccountId>
    where 
    AccountId:sp_api::Encode,
    {
        fn get_class(db_id:Vec<u8>,app_id:Vec<u8>,acc:AccountId)->Vec<ClassData>;
    }
}