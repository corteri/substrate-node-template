#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]
use sp_std::vec::Vec;
//use sp_core::H256;
use alioth_register::Meta;
sp_api::decl_runtime_apis!{
    pub trait RegisterApi<AccountId>
    where 
    AccountId:sp_api::Encode,
    {
        fn get_apps_detail(app_id:AccountId)->Vec<Meta>;
    }
}