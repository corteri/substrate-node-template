#![cfg_attr(not(feature="std"),no_std)]
use alioth_db::DbMeta;
use sp_std::vec::Vec;
sp_api::decl_runtime_apis!{
    pub trait DbApi{
        fn get_db(app_id:Vec<u8>)->Vec<DbMeta>;
    }
}