#![cfg_attr(not(feature="std"),no_std)]
use alioth_ob::ObjectData;
use sp_std::vec::Vec;
sp_api::decl_runtime_apis!{
    pub trait ObApi<AccountId>
    where 
    AccountId:sp_api::Encode,
    {
       // fn get_class(db_id:Vec<u8>,app_id:Vec<u8>,acc:AccountId)->Vec<ClassData>;
       fn object_all(acc:AccountId,class_id:Vec<u8>,app_id:Vec<u8>)->Vec<ObjectData>;
       fn object_unique(acc:AccountId,class_id:Vec<u8>,app_id:Vec<u8>,oid:Vec<u8>)->ObjectData;
       fn object_user(acc:AccountId,class_id:Vec<u8>,app_id:Vec<u8>,uuid:Vec<u8>)->Vec<ObjectData>;
    }
}