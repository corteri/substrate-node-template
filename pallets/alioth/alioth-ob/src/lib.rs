//now create a parsing algorith for delete and update purpose;
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{StorageMap, codec::{Encode,Decode}, decl_error, decl_event, decl_module, decl_storage, dispatch::{DispatchResult}, ensure};
use frame_system::{ensure_signed};
use sp_core::{H512,  sr25519::{Public,Signature}};
use frame_support::traits::Vec;
//use frame_support::sp_runtime::RuntimeDebug;
use frame_support::sp_std::vec;
#[cfg(feature = "std")]
use serde::{Serialize,Deserialize};
#[derive(Encode, Decode, Clone,Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ObjectData{
    duid:Vec<u8>,
    signed_data:Vec<u8>,
    unsigned_data:Vec<u8>,
    uuid:Vec<u8>,
    signature:H512,
}  
#[derive(Encode, Decode, Clone,Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct LeftData{
    app_id:Vec<u8>,
    duid:Vec<u8>,
    unsigned_data:Vec<u8>,
    uuid:Vec<u8>,
    dbid:Vec<u8>,
}


pub trait Config:frame_system::Config+alioth_users::Config+alioth_class::Config+alioth_register::Config{
    type Event:From <Event>+Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T:Config> as Object{
        ObjectStore get(fn object_store):map hasher(blake2_128_concat) Vec<u8>=>Vec<ObjectData>;
        LeftStore get(fn left_store):map hasher(blake2_128_concat) Vec<u8>=>Vec<LeftData>;
    }
}
decl_module! {
    pub struct Module<T:Config> for enum Call where origin: T::Origin{
        fn deposit_event() = default;
        type Error = Error<T>;
        //This is new Data Function Because earlier was not succeed because of json incompatibility
        #[weight=10_000]
        pub fn insert_data_class(origin,app_id:Vec<u8>,db_id:Vec<u8>,cid:Vec<u8>,signeddata:Vec<u8>,unsigneddata:Vec<u8>,uuid:Vec<u8>,ouid:Vec<u8>,signature:H512)->DispatchResult{
            let caller = ensure_signed(origin)?;
            /*
            Doing Some Checking here to verify all the stuff releated to object data*/
            //checking whether the caller owns the app or not
            let app_data = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(app_data.0,Error::<T>::AppNotFound);
            //checking whether the given uuid exists in the provided app_id or not.
            let user_data = alioth_users::Module::<T>::check_user_extern(&uuid,&app_id);
            ensure!(user_data.0,Error::<T>::UserNotFound);
            //checking whether the requested class and database exists inside the app or not.
            let class_data = alioth_class::Module::<T>::check_class(&db_id,&cid);
            ensure!(class_data.0,Error::<T>::ClassOrDatabaseNotFound);
            //trying to get the public key of the provided uuid to verify the signed data
            let public_key_tuple = alioth_users::Module::<T>::get_public_key(&uuid,&app_id);
            //now we are going to verify the signature
            ensure!(sp_io::crypto::sr25519_verify(&Signature::from_h512(signature),&signeddata,&Public::from_h256(public_key_tuple.1)),Error::<T>::SignatureNotVerified);
            //checking whether the given uid already present inside the data or not
            let class_check = Self::check_obj_store(&ouid,&cid);
            let left_data = Self::check_left_data(&uuid,&ouid);
            ensure!(!class_check.0,Error::<T>::ProvidedOuidAlreadyPresent);
            ensure!(!left_data.0,Error::<T>::ProvidedOuidAlreadtPresentInUnsigneddata);
            //we are inserting the data inside the  ObjectStore
            let object_to_insert = ObjectData{
                duid:ouid.clone(),
                signed_data:signeddata,
                unsigned_data:unsigneddata.clone(),
                uuid:uuid.clone(),
                signature:signature,
            };
            if class_check.1==0{
                //it means the class is empty and we have to insert the first Object
                let vec_data = vec![object_to_insert];
                <ObjectStore>::insert(cid,vec_data);
                Self::deposit_event(Event::ObjectCreated());
            }
            else{
                //it means class is not empty and we have to append our ObjectData to the List
                let mut vec_data = <ObjectStore>::get(&cid);
                vec_data.insert(vec_data.len(),object_to_insert);
                <ObjectStore>::insert(cid,vec_data);
                Self::deposit_event(Event::ObjectCreated());
            }

            //we are inserting the data inside the LeftStorage
            let left_to_insert = LeftData{
                app_id:app_id,
                duid:ouid.clone(),
                unsigned_data:unsigneddata,
                uuid:uuid.clone(),
                dbid:db_id,
            };
            if left_data.1==2{
                //It Means The Left Storage Is Not Empty And We Have To Append Our LeftData To The List
                let mut vec_data = <LeftStore>::get(&uuid);
                vec_data.insert(vec_data.len(),left_to_insert);
                <LeftStore>::insert(uuid,vec_data);
                Self::deposit_event(Event::LeftDataAdded());
            }
            else{
                //It Means The LeftStorage Is Empty And We Have To Insert The First Object.
                let vec_data = vec![left_to_insert];
                <LeftStore>::insert(uuid,vec_data);
                Self::deposit_event(Event::LeftDataAdded());
            }

            //checking whether the data with ouid already present or not
            

            Ok(())
        }










/*
        #[weight=10_000]
        pub fn insert_data_class(origin,app_id:Vec<u8>,db_id:Vec<u8>,cid:Vec<u8>,data:Vec<u8>,signature:H512)->DispatchResult{
            let caller = ensure_signed(origin)?;
            let app_data = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(app_data.0,Error::<T>::AppNotFound);
            let data_str = String::from_utf8(data.clone()).unwrap();
            let data_json:(_,usize) =  serde_json_core::from_str(&data_str).unwrap();
            let jus:Vec<u8> = data_json.0["uid"];
                        //json!(data_str);
            let uid:Vec<u8> = jus::<T>::["uid"].to_string().as_bytes().to_vec();
            //let uuid_str = data_json["uuid"].to_string();
            let uuid:Vec<u8> = data_json["uuid"].to_string().as_bytes().to_vec();
            let  data1 = data.clone();
            let data1 = String::from_utf8(data1).unwrap();
            let user_data = alioth_users::Module::<T>::check_user_extern(&uuid,&app_id);
            ensure!(user_data.0,Error::<T>::UserNotFound);
            let class_data = alioth_class::Module::<T>::check_class(&db_id,&cid);
            ensure!(class_data.0,Error::<T>::ClassOrDatabaseNotFound);
            //I needed a public key for this purpose we are going to write a function in alioth-users
            let pub_key_ref:(bool,H256) = alioth_users::Module::<T>::get_public_key(&uuid,&app_id);
            ensure!(pub_key_ref.0,Error::<T>::ObjectNotCreated);

            //now we are going to verify the signature for the further transaction
            ensure!(sp_io::crypto::sr25519_verify(&Signature::from_h512(signature),data1.as_bytes(),&Public::from_h256(pub_key_ref.1)),Error::<T>::SignatureNotVerified);
            //its over now we are going ahead to check the uniqness of data and insert it later.
            let data_from_self = Self::check_obj_store(&uid,&cid);
            ensure!(!data_from_self.0,Error::<T>::DataIsNotUnique);
            let struct_data = ObjectData{
                duid:uid,
                object:data,
            };
            if data_from_self.1==0{
                let vec_data = vec![struct_data];
                <ObjectStore>::insert(cid,vec_data);
                Self::deposit_event(Event::ObjectCreated());
            }
            else{
                let mut vec_data = <ObjectStore>::get(&cid);
                vec_data.insert(vec_data.len(),struct_data);
                <ObjectStore>::insert(cid,vec_data);
                Self::deposit_event(Event::ObjectCreated());
            }
            Ok(())
        }
        */
        #[weight=10_000]
        pub fn revoke_obj(origin,app_id:Vec<u8>,db_id:Vec<u8>,cid:Vec<u8>,oid:Vec<u8>)->DispatchResult{
            //when ever the data deleted from the db we have to inform the users about this actions , that's why we are user centric
            let caller = ensure_signed(origin)?;
            //starting some necessary verification

            let app_data = alioth_register::Module::<T>::check_ownership(&caller,&app_id);
            ensure!(app_data.0,Error::<T>::AppNotFound);
            let class_data = alioth_class::Module::<T>::check_class(&db_id,&cid);
            ensure!(class_data.0,Error::<T>::ClassOrDatabaseNotFound);
            let data_from_self = Self::check_obj_store(&oid,&cid);
            ensure!(data_from_self.0,Error::<T>::ClassNotFound);
            //now the logic for revoke beging
            let mut check = false;
            let  data_for_revenge = <ObjectStore>::get(&cid);
            let mut data_for_revenge_1 = data_for_revenge.clone();
            let mut count = 0;
            for input in data_for_revenge{
                if input.duid == oid{
                   /*
                   ToDo:-
                   1)Inform the owner of the data about this change,
                   2)Instead of deleting we have to setup the flag that shows data deleted
                   3)Delete The Data Only When The Users Wants It To Be Deleted
                   */
                    data_for_revenge_1.remove(count);
                    check = true;
                }
                    count = count+1;
            }
            ensure!(check,Error::<T>::ObjectRevokingFailed);
            <ObjectStore>::insert(oid,data_for_revenge_1);
            Self::deposit_event(Event::ObjectRevoked());
            Ok(())
        }
        /*
        ToDo:-
        1)Create An Algorith For Relation inBetween The Data And The Users For The Faster Transaction.
        2)Implement And Test The Algorithm
        */
    }
}
decl_event!(
    pub enum Event{
        ObjectCreated(),
        ObjectRevoked(),
        LeftDataAdded(),
    }
);
decl_error! {
    pub enum Error for Module<T:Config>{
        ObjectNotCreated,
        AppNotFound,
        UserNotFound,
        ClassOrDatabaseNotFound,
        SignatureNotVerified,
        DataIsNotUnique,
        ClassNotFound,
        ObjectRevokingFailed,
        ProvidedOuidAlreadyPresent,
        ProvidedOuidAlreadtPresentInUnsigneddata,
    }
}
impl <T:Config> Module<T> {
    fn check_obj_store(uid:&Vec<u8>,cid:&Vec<u8>)->(bool,u8){
        if <ObjectStore>::contains_key(cid){
            let data_store = <ObjectStore>::get(cid);
            let mut check = false;
            for input in data_store{
                if input.duid==*uid{
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

    fn check_left_data(uuid:&Vec<u8>,ouid:&Vec<u8>)->(bool,u8){
        if <LeftStore>::contains_key(uuid){
            let take_data = <LeftStore>::get(uuid);
            let mut check = false;
            for input in take_data{
                if input.duid==*ouid{
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

   pub fn object_all(acc:T::AccountId,class_id:Vec<u8>,app_id:Vec<u8>)->Vec<ObjectData>{
       //check the data
       //we also have to verify everything with the data.
       let app_data = alioth_register::Module::<T>::check_ownership(&acc,&app_id);
       if app_data.0{
        if <ObjectStore>::contains_key(&class_id){
            let data = <ObjectStore>::get(&app_id);
            data
        }
        else{
            let default_data = ObjectData{
                uuid:b"null".to_vec(),
                unsigned_data:b"null".to_vec(),
                signed_data:b"null".to_vec(),
                signature:H512::default(),
                duid:b"null".to_vec(),
            };
            vec![default_data]
        }
    }
       else{
        let default_data = ObjectData{
            uuid:b"null".to_vec(),
            unsigned_data:b"null".to_vec(),
            signed_data:b"null".to_vec(),
            signature:H512::default(),
            duid:b"null".to_vec(),
        };
        vec![default_data]
       }
    }

    pub fn object_unique(acc:T::AccountId,class_id:Vec<u8>,app_id:Vec<u8>,oid:Vec<u8>)->ObjectData{
        let app_data = alioth_register::Module::<T>::check_ownership(&acc,&app_id);
        if app_data.0{
            let mut default_data = ObjectData{
                uuid:b"null".to_vec(),
                unsigned_data:b"null".to_vec(),
                signed_data:b"null".to_vec(),
                signature:H512::default(),
                duid:oid.clone(),
            };
            let data = <ObjectStore>::get(&class_id);
            for input in data{
                if input.duid == oid{
                    default_data.uuid = input.uuid;
                    default_data.unsigned_data = input.unsigned_data;
                    default_data.signed_data =input.signed_data;
                    default_data.signature = input.signature;
                    break;
                }
            }
            default_data
        } 
        else{
            let default_data = ObjectData{
                uuid:b"null".to_vec(),
                unsigned_data:b"null".to_vec(),
                signed_data:b"null".to_vec(),
                signature:H512::default(),
                duid:oid.clone(),
            };
            default_data
        }
    }
    pub fn object_user(acc:T::AccountId,class_id:Vec<u8>,app_id:Vec<u8>,uuid:Vec<u8>)->Vec<ObjectData>{
        let app_data = alioth_register::Module::<T>::check_ownership(&acc,&app_id);
        if app_data.0{
            let default_data = ObjectData{
                uuid:b"null".to_vec(),
                unsigned_data:b"null".to_vec(),
                signed_data:b"null".to_vec(),
                signature:H512::default(),
                duid:uuid.clone(),
            };
            let mut data_to_send:Vec<ObjectData> = vec![default_data];
            let data = <ObjectStore>::get(&class_id);
            for input in data{
                if input.uuid == uuid{
                    data_to_send.remove(data_to_send.len()-1);
                   data_to_send.insert(data_to_send.len(),input);
                }
            }
            data_to_send
        } 
        else{
            let default_data = ObjectData{
                uuid:b"null".to_vec(),
                unsigned_data:b"null".to_vec(),
                signed_data:b"null".to_vec(),
                signature:H512::default(),
                duid:uuid,
            };
            vec![default_data]
        }
    }
}