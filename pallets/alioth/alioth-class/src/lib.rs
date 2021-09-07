#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{decl_event,decl_module,decl_storage,ensure,decl_error,dispatch::{DispatchResult},codec::{Encode,Decode}};
use frame_system::{ensure_signed};
//use serde_json::{json};
use frame_support::traits::Vec;
use frame_support::sp_runtime::RuntimeDebug;
use frame_support::sp_std::vec;
#[cfg(feature = "std")]
use serde::{Serialize,Deserialize};

pub trait Config:frame_system::Config+alioth_db::Config+alioth_register::Config{
    type Event:From <Event>+Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Clone,Eq, PartialEq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData{
    class_id:Vec<u8>,
    column:Vec<u8>,
    column_type:Vec<u8>,
}
decl_storage! {
    trait Store for Module<T:Config> as ClassUser{
        ClassStore get(fn class_data):map hasher(blake2_128_concat) Vec<u8>=>Vec<ClassData>;
    }
}
decl_module! {
    pub struct Module<T:Config> for enum Call where origin: T::Origin{
        fn deposit_event()= default;
        type Error = Error<T>;
        //firstly create the class after which we will start our next part of coding
        #[weight=10_000]
        fn create_class(origin,data:Vec<u8>,column:Vec<u8>,cid:Vec<u8>,db_id:Vec<u8>,app_id:Vec<u8>)->DispatchResult{
            let caller = ensure_signed(origin)?;
            let register_data = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(register_data.0,Error::<T>::AppNotFound);
            let db_data = alioth_db::Module::<T>::check_db(&app_id,&db_id);
            ensure!(db_data.0,Error::<T>::DbNotExist);
            let class_data = Self::check_class(&db_id,&cid);
            ensure!(!class_data.0,Error::<T>::ClassAlreadyExist);
            let data1 = ClassData{
                class_id:cid.clone(),
                column:column,
                column_type:data,
            };
            if class_data.1==0{

                let mut vec_data = <ClassStore>::get(&cid);
                vec_data.insert(vec_data.len(),data1);
                <ClassStore>::insert(db_id,vec_data);
                Self::deposit_event(Event::ClassCreated());
            }
            else{
                let vec_data = vec![data1];
                <ClassStore>::insert(db_id,vec_data);
                Self::deposit_event(Event::ClassCreated());
            }
            Ok(())
        }
        #[weight=10_000]
        fn update_class(origin,app_id:Vec<u8>,cid:Vec<u8>,db_id:Vec<u8>,data_column:Vec<u8>,data_type:Vec<u8>)->DispatchResult{
            let caller = ensure_signed(origin)?;
            let register_data = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(register_data.0,Error::<T>::AppNotFound);
            let db_data = alioth_db::Module::<T>::check_db(&app_id,&db_id);
            ensure!(db_data.0,Error::<T>::DbNotExist);
            let class_data = Self::check_class(&db_id,&cid);
            ensure!(class_data.0,Error::<T>::ClassAlreadyExist);
            let mut count = 0;
            let data_recieve = <ClassStore>::get(&cid);
            let mut data_2 = data_recieve.clone();
            let mut check = false;
            for i in data_recieve{
                if i.class_id==cid{
                    data_2.remove(count);
                    let data_to_insert= ClassData{
                        class_id:cid.clone(),
                        column:data_column,
                        column_type:data_type,
                    };
                    data_2.insert(count,data_to_insert);
                    <ClassStore>::insert(db_id,data_2);
                    check = true;
                    break;
                }
                    count = count+1;
            }
            ensure!(check,Error::<T>::ClassNotUpdated);
            Self::deposit_event(Event::ClassUpdated());
              Ok(())
        }
        #[weight=10_000]
        fn revoke_class(origin,app_id:Vec<u8>,cid:Vec<u8>,db_id:Vec<u8>)->DispatchResult{
            let caller = ensure_signed(origin)?;
            let register_data = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(register_data.0,Error::<T>::AppNotFound);
            let db_data = alioth_db::Module::<T>::check_db(&app_id,&db_id);
            ensure!(db_data.0,Error::<T>::DbNotExist);
            let class_data = Self::check_class(&db_id,&cid);
            ensure!(class_data.0,Error::<T>::ClassAlreadyExist);
            let mut count = 0;
            let data_recieve = <ClassStore>::get(&cid);
            let mut data_2 = data_recieve.clone();
            let mut check = false;
            for i in data_recieve{
                if i.class_id==cid{
                    data_2.remove(count);
                    check = true;
                    break;
                }
                    count = count+1;
            }
            ensure!(check,Error::<T>::ClassNotRevoked);
            Self::deposit_event(Event::ClassRevoked());
              Ok(())
        }
    }
}
decl_event!(pub enum Event{
    ClassCreated(),
    ErrorDuringClassCreation(),
    ClassUpdated(),
    ClassRevoked(),
});
decl_error! {
    pub enum Error for Module<T:Config>{
        ClassCreationFailed,
        ClassAlreadyExist,
        AppNotFound,
        DbNotExist,
        ClassNotExist,
        ClassNotUpdated,
        ClassNotRevoked,
    }
}
impl <T:Config> Module<T>{
    pub fn check_class(db_id:&Vec<u8>,cid:&Vec<u8>)->(bool,u16){
        if <ClassStore>::contains_key(db_id){
            let data = <ClassStore>::get(db_id);
            let mut check = false;
            for input in data{
                if input.class_id==*cid{
                    check=true;
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

    //verify the app_id with the account_id

    pub fn get_class(db_id:Vec<u8>,app_id:Vec<u8>,acc:T::AccountId)->Vec<ClassData>{
        //check whether the particulas acc contains the given app_id
        let register_data = alioth_register::Module::<T>::check_ownership(&acc,&app_id);
        if register_data.0{
            //checking whetehr the db exist in the app db collection 
            let db_data = alioth_db::Module::<T>::check_db(&app_id,&db_id);
            if db_data.0{
                let data = <ClassStore>::get(db_id);
                //now from here we will send all Class Data To Our End User
            data
            }
            else{
                let default_data = ClassData{
                    column:b"null".to_vec(),
                    column_type:b"null".to_vec(),
                    class_id:b"null".to_vec(),
                };
                vec![default_data]
                //we have to send the default value to the end user
            }
        }
        else{
            //we have to send the default value to the end user
            let default_data = ClassData{
                column:b"null".to_vec(),
                column_type:b"null".to_vec(),
                class_id:b"null".to_vec(),
            };
            vec![default_data]
        }
    }
}