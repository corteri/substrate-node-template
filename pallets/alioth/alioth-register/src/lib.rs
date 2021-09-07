#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{decl_error,ensure,decl_event,decl_storage,decl_module,dispatch::{DispatchResult},codec::{Encode,Decode}};
use frame_system::{ensure_signed};
use frame_support::sp_runtime::RuntimeDebug;
use frame_support::traits::Vec;
use frame_support::sp_std::vec;
//use frame_support::{Serialize,Deserialize};
//use frame_support::serde::{Serialize,Deserialize};

#[cfg(feature = "std")]
use serde::{Serialize,Deserialize};

pub trait Config:frame_system::Config {
	type Event:From <Event>+Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Clone,Eq, PartialEq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Meta{
	uid:Vec<u8>,
	timestamp:u32,
}

decl_storage! {
	trait Store for Module<T:Config> as App{
		RDATA get(fn r_data):map hasher(twox_64_concat) T::AccountId=>Vec<Meta>;
	}
}
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		type Error = Error<T>;
		#[weight=10_000]
		fn sign_up_app(origin,ruid:Vec<u8>)->DispatchResult{
			let who = ensure_signed(origin)?;
			//check whether the app exist or not in the Vector.
			let ruid1 = ruid.clone();
			let data_insert = Meta{
				uid:ruid,
				timestamp:4,
			};
			let data = Self::check_ownership(&who,&ruid1);
			//we have to check from the collection creation about the uid
			ensure!(!data.0,Error::<T>::AppAlreadyExist);
			if data.1==0{
				let vec_data = vec![data_insert];
				<RDATA<T>>::insert(who,vec_data);
				Self::deposit_event(Event::AppRegistered());
			}
			else{
				let mut vec_data_re = <RDATA<T>>::get(&who);
				vec_data_re.insert(vec_data_re.len(),data_insert);
				<RDATA<T>>::insert(who,vec_data_re);
				Self::deposit_event(Event::AppRegistered());
			}
			Ok(())
		}
		#[weight=10_000]
		fn revoke_app(origin,uid:Vec<u8>)->DispatchResult{
			let who = ensure_signed(origin)?;
			ensure!(<RDATA<T>>::contains_key(&who),Error::<T>::AccountDoesNotExist);
			let  take_data  = <RDATA<T>>::get(&who);
			let mut check = false;
             let mut take_data2 = take_data.clone();
			 let mut i = 0;
			for input in take_data{
				if input.uid==uid{
				take_data2.remove(i);
				check = true;
				break;
				}
				i = i+1;
			}
			ensure!(check,Error::<T>::AppNotExist);
			Self::deposit_event(Event::AppRemoved());
			Ok(())
		}
	}
}
decl_event!(pub enum Event{
	AppRegistered(),
	AppRemoved(),
	AppRevoked(),
});
decl_error! {
	pub enum Error for Module<T:Config>{
		AppRegistrationFailed,
		AmountDoesNotExist,
		SystemAlreadyOnFullCapacity,
		AccountDoesNotExist,
		AppNotExist,
		AppAlreadyExist,
	}
}
impl <T:Config> Module<T>{
	pub fn check_ownership(acc_id:&T::AccountId,uid:&Vec<u8>)->(bool,u16){
		if <RDATA<T>>::contains_key(acc_id){
			let mut check = false;
			let take_data = <RDATA<T>>::get(acc_id);
			for input in take_data{
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
	pub fn get_apps_detail(app_id:T::AccountId)->Vec<Meta>{
		if <RDATA<T>>::contains_key(&app_id){
			let data = <RDATA<T>>::get(&app_id);
			data
		}
		else{
			let data_to_fool = Meta{
				uid:b"hello".to_vec(),
				timestamp:4,
			};
			vec![data_to_fool]
		}
	}
}