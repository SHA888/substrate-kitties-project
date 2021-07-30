#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*,
    traits::{Randomness, Currency, ExistenceRequirement},
    transactional,
};
use frame_system::{
    pallet_prelude::*,
    offchain::{SendTransactionTypes, SubmitTransaction},
};

use sp_std::{
    prelude::*,
    convert::TryInto
};

use sp_io::hashing::blake2_128;
use sp_runtime::offchain::storage_lock::{StorageLock, BlockAndTime};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaChaRng,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub use pallet::*;

#[cfg(test)]
mod test;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights;

pub use weights::WeightInfo;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Kitty(pub[u8; 16]);

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq)]
pub enum KittyGender {
    Male,
    Female,
}

impl Kitty {
    pub fn gender(&self) -> KittyGender {
        if self.0[0] % 2 == 0 {
            KittyGender::Male
        } else {
            KittyGender::Female
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + orml_nft::Config<TokenData = Kitty, ClassData = ()> + SendTransactionTypes<Call<Self>> {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness::<Self::Hash, Self::BlockNumber>;
        type Currency: Currency<Self::AccountId>;
        type WeightInfo: WeightInfo;
        #[pallet::constant]
        type DefaultDifficulty: Get<u32>;
    }

    pub type KittyIndexOf<T> = <T as orml_nft::Config>::TokenId;
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::

    // Get kitty price. None means not for sale.
    #[pallet::storage]
    #[pallet::getter(fn kitty_prices)]
    pub type KittyPrices<T: Config> = StorageMap<
        _,
        Blake2_128Concat, KittyIndexOf<T>,
        BalanceOf<T>, OptionQuery
    >;

    // The class id for orml_nft
    #[pallet::storage]
    #[pallet::getter(fn class id)]
    pub type ClassId<T: Config> = StorageValue<_, T::ClassId, ValueQuery>;

    // Nonce for auto breed to prevent replay attack
    #[pallet::storage]
    #[pallet::getter(fn auto breed_nonce)]
    pub type AutoBreedNonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(Default)
    pub struct GenesisConfig;
    
    #[pallet::genesis_build]]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            // create a NFT class
            let class_id = orml_nft::Pallet::<T>::create_class(&Default::default(), Vec::new(), ())
                .expect("Cannot fail or invalid chain spec");
            ClassId::<T>::put(class_id);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(
        T::AccountId = "AccountId", KittyIndexOf<T> = "KittyIndex", Option<BalanceOf<T>> = "Option<Balance>", BalanceOf<T> = "Balance",
    )]
    pub enum Event<T: Config> {
        /// A kitty is created. \[owner, kitty_id, kitty\]
    }


}




/* 
// FRAME V.1 syntaxes
//
// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
     trait Store for Module<T: Config> as Kitties {
        // Stores all the kitties, key is the kitty id
        pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;
        // Stores the next kitty ID
        pub NextKittyId get(fn next_kitty_id): u32;
    }
}

decl_event! {
    pub enum Event<T> where
    <T as frame_system::Config>::AccountId,
    {
        // A kitty is created. \[owner, kitty_id, kitty\]
        KittyCreated(AccountId, u32, Kitty),
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        KittiesIdOverflow,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // type Error = Error<T>;

        fn deposit_event() = default;

        // Create a new kitty
        #[weight = 1000]
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;

             // TODO: ensure kitty id does not overflow
             // return Err(Error::<T>::KittiesIdOverflow.into());

             // Generate a random 128bit value
            let payload = (
                <pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
                &sender,
                <frame_system::Module<T>>::extrinsic_index(),
            );
            let dna = payload.using_encoded(blake2_128);

             // Create and store kitty
             let kitty = Kitty(dna);
             let kitty_id = Self::next_kitty_id();
             Kitties::<T>::insert(&sender, kitty_id, kitty.clone());
             NextKittyId::put(kitty_id + 1);

             // Emit event
             Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id, kitty))
        }
    }
}
*/