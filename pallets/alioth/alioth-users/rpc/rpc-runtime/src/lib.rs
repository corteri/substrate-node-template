#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]
use sp_std::vec::Vec;
//use sp_core::H256;
use alioth_users::UserData;
sp_api::decl_runtime_apis!{
    pub trait UserApi<AccountId>
    where 
    AccountId:sp_api::Encode,
    {
        fn check_user_rpc(uid:Vec<u8>,app_id:Vec<u8>,acc:AccountId)->bool;
        fn get_all_users(app_id:Vec<u8>,acc:AccountId)->Vec<UserData>;
        fn get_user(app_id:Vec<u8>,uuid:Vec<u8>,acc:AccountId)->UserData;
    }
}