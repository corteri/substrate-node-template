#![cfg_attr(not(feature="std"),no_std)]
use frame_support::{decl_error,ensure,decl_event,decl_storage,decl_module,dispatch::{DispatchResult},codec::{Encode,Decode}};
use frame_system::{ensure_signed};
use frame_support::sp_runtime::RuntimeDebug;
use sp_core::{H256,H512,sr25519::{Signature,Public}};
use parity_scale_codec::alloc::string::ToString;
use frame_support::traits::Vec;
use frame_support::sp_std::vec;
#[cfg(feature = "std")]
use serde::{Serialize,Deserialize};
#[derive(Encode, Decode, Clone,Eq, PartialEq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct UserData{
    uid:Vec<u8>,
    pub_key:H256,
}
pub trait Config:frame_system::Config+alioth_register::Config {
	type Event:From <Event>+Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T:Config> as Users{
        User get(fn user):map hasher(blake2_128_concat) Vec<u8>=>Vec<UserData>;
    }
}
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;
        type Error = Error<T>;
        #[weight=10_000]
        fn register_user(origin,uniq_id:Vec<u8>,public_key:H256,app_id:Vec<u8>,s_data:u32,sig:H512)->DispatchResult{
            let caller = ensure_signed(origin)?;
            let data_str = s_data.to_string();
            ensure!(sp_io::crypto::sr25519_verify(&Signature::from_h512(sig),data_str.as_bytes(),&Public::from_h256(public_key)),Error::<T>::SignatureNotVerified);
            let register_data = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(register_data.0,Error::<T>::AppNotExist);
            //app exist but now we have to check whether the user already exists with its primary key or not
            let user_return_data = Self::check_user(&public_key, &uniq_id,&app_id);
            ensure!(!user_return_data.0,Error::<T>::UserAlreadyExist);
            let data = UserData{
                uid:uniq_id,
                pub_key:public_key,
            };
            if user_return_data.1==2{
                let mut data_return = <User>::get(&app_id);
                data_return.insert(data_return.len(),data);
                <User>::insert(app_id,data_return);
                Self::deposit_event(Event::UserRegistered());
            }
            else{
                let data_to_insert = vec![data];
                <User>::insert(app_id,data_to_insert);
                Self::deposit_event(Event::UserRegistered());
            }
            Ok(())
        }

        #[weight=10_000]
        fn revoke_user(origin,uid:Vec<u8>,app_id:Vec<u8>)->DispatchResult{
            let caller = ensure_signed(origin)?;
            let register_data = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(register_data.0,Error::<T>::AppNotExist);
            let user_return_data = Self::check_user_extern(&uid,&app_id);
            ensure!(user_return_data.0,Error::<T>::UidNotExist);
            let mut check = false;
            let mut index = 0;
            let user_data = <User>::get(&app_id);
            let mut user_data1 = user_data.clone();
            for input in user_data{
                if input.uid==uid{
                    user_data1.remove(index);
                    check = true;
                }
                index = index+1;
            }
            ensure!(check,Error::<T>::UserRevokedFailed);
            Self::deposit_event(Event::UserRevoked());
            Ok(())
        }
    }
}
decl_event!(
    pub enum Event{
        UserRegistered(),
        UserRevoked(),
    }
);
decl_error!{
    pub enum Error for Module<T:Config>{
        UserRegistrationFailed,
        AppNotExist,
        UserAlreadyExist,
        SignatureNotVerified,
        UidNotExist,
        UserRevokedFailed,
    }
}
impl <T:Config> Module<T>{
     fn check_user(pub_key:&H256,uid:&Vec<u8>,app_id:&Vec<u8>)->(bool,u16){
        if <User>::contains_key(app_id){
            let user_data = <User>::get(app_id);
            let mut check = false;
            for input in user_data{
                if input.uid==*uid||input.pub_key==*pub_key{
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
    pub fn check_user_extern(uid:&Vec<u8>,app_id:&Vec<u8>)->(bool,u16){
        if <User>::contains_key(app_id){
            let user_data = <User>::get(app_id);
            let mut check = false;
            for input in user_data{
                if input.uid==*uid{
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

    pub fn get_public_key(uid:&Vec<u8>,app_id:&Vec<u8>)->(bool,H256){
            let user_data = <User>::get(app_id);
            let mut data_to:H256 = H256::default(); 
            let mut check = false;
            for input in user_data{
                if input.uid==*uid{
                    data_to = input.pub_key;
                    check = true;
                    break;
                }
            }
            (check,data_to)
    }

    pub fn check_user_rpc(uid:Vec<u8>,app_id:Vec<u8>,acc:T::AccountId)->bool{
        //checking ownership of the app using the accountid and later on we will verify it.
        let register_data = alioth_register::Module::<T>::check_ownership(&acc,&app_id);
        if register_data.0{
            false
        }
        else{
            if <User>::contains_key(&app_id){
                let user_data = <User>::get(&app_id);
                let mut check = false;
                for input in user_data{
                    if input.uid==uid{
                        check = true;
                        break;
                    }
                }
                if check{
                    true
                }
                else{
                    false
                }
            }
            else{
                false
            }
        }
    }

    pub fn get_all_users(app_id:Vec<u8>,acc:T::AccountId)->Vec<UserData>{
        let register_data = alioth_register::Module::<T>::check_ownership(&acc,&app_id);
        if register_data.0{
            <User>::get(app_id)
        }
        else{
           
            let  user_data = UserData{
                uid:b"not found".to_vec(),
                pub_key:H256::default(),
            };
            vec![user_data]
        }
    }

    pub fn get_user(app_id:Vec<u8>,uuid:Vec<u8>,acc:T::AccountId)->UserData{
        //checking the app id
        let register_data = alioth_register::Module::<T>::check_ownership(&acc,&app_id);
        if register_data.0{
            let all_users = <User>::get(app_id);
            let mut public_key:H256 = H256::default();
            let mut check = false;
            for index in all_users{
                if index.uid==uuid{
                    public_key = index.pub_key;
                    check = true;
                    break;
                }
            }
            if check{
                UserData{
                    uid:uuid,
                    pub_key:public_key,
                }
            }
            else{
                UserData{
                    uid:b"not found".to_vec(),
                    pub_key:H256::default(),
                }
            }
        }
        else{
            UserData{
                uid:b"not found".to_vec(),
                pub_key:H256::default(),
            }
        }
    }
}