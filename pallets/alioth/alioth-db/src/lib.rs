#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{decl_event,decl_module,decl_storage,ensure,decl_error,dispatch::{DispatchResult},codec::{Encode,Decode}};
use frame_system::{ensure_signed};
use frame_support::traits::Vec;
use frame_support::sp_runtime::RuntimeDebug;
use frame_support::sp_std::vec;
#[cfg(feature = "std")]
use serde::{Serialize,Deserialize};


pub trait Config:frame_system::Config+alioth_register::Config{
    type Event:From <Event>+Into<<Self as frame_system::Config>::Event>;
}
#[derive(Encode, Decode, Clone,Eq, PartialEq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]pub struct DbMeta{
    did:Vec<u8>,
    timestamp:u8,
}
decl_storage! {
    trait Store for Module<T:Config> as Database{
        DbData get(fn db_data):map hasher(blake2_128_concat) Vec<u8>=>Vec<DbMeta>;
    }
}
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event()= default;
        type Error = Error<T>;
        #[weight=10_000]
        fn create_db(origin,app_id:Vec<u8>,db_id:Vec<u8>)->DispatchResult{
            let caller = ensure_signed(origin)?;
            let data_from_app:(bool,u16) = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(data_from_app.0,Error::<T>::AppNotFound);
            let check_db = Self::check_db(&app_id, &db_id);
            ensure!(!check_db.0,Error::<T>::DbAlreadyExist);
            let data = DbMeta{
                did:db_id,
                timestamp:6,
            };
            if check_db.1==0{
                let vec_data = vec![data];
                <DbData>::insert(app_id,vec_data);
                Self::deposit_event(Event::DbCreated());
            }
            else{
                let mut vec_data = <DbData>::get(&app_id);
                vec_data.insert(vec_data.len(),data);
                <DbData>::insert(app_id,vec_data);
                Self::deposit_event(Event::DbCreated());
            }
            Ok(())
        }
        #[weight=10_000]
        fn revoke_db(origin,app_id:Vec<u8>,db_id:Vec<u8>)->DispatchResult
        {
            let caller = ensure_signed(origin)?;
            let data_from_app = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(data_from_app.0,Error::<T>::AppNotFound);
            let check_db = Self::check_db(&app_id, &db_id);
            ensure!(check_db.0,Error::<T>::DbNotExist);
            let mut count = 0;
            let  data = <DbData>::get(&app_id);
            let mut data1 = data.clone();
            let mut check = false;
            for input in data{
                if input.did==db_id{
                    data1.remove(count);
                    check = true;
                    break;
                }
                count = count+1;
            }
            ensure!(check,Error::<T>::DbDidNotRevoked);
            <DbData>::insert(app_id,data1);
            Self::deposit_event(Event::DbRevoked());
            Ok(())
        }   
    }
}
decl_event!(
    pub enum Event{
        DbCreated(),
        DbRevoked(),
    }
);
decl_error! {
    pub enum Error for Module<T:Config>{
        DbCreationFailed,
        AppNotFound,
        DbAlreadyExist,
        DbNotExist,
        DbDidNotRevoked,
    }
}
impl <T:Config> Module<T>{
    pub fn check_db(app_id:&Vec<u8>,db_id:&Vec<u8>)->(bool,u16){
        if <DbData>::contains_key(app_id){
            let data = <DbData>::get(app_id);
            let mut check = false;
            for index in data{
                if index.did==*db_id{
                    check = true;
                    break;
                }
            }
            if check{
                (true,1)
            }
            else{
                (false,2)
            }
        }
        else{
            (false,0)
        }
    }
    pub fn get_db(app_id:Vec<u8>)->Vec<DbMeta>{
        if <DbData>::contains_key(&app_id){
            let data = <DbData>::get(&app_id);
            data
        }
        else{
            let data = DbMeta{
                did:b"data not found".to_vec(),
                timestamp:0,
            };
            vec![data]
        }
    }
}