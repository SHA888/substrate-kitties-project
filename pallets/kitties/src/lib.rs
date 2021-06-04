#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageDoubleMap,
    traits::Randomness, RuntimeDebug,
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struck Kitty(pub[u8; 16]);

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as Kitties {
        // Stores all the kitties, key is the kitty id
        pub Kitties get(fn kitties): double_map (blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;
        // Stores the next kitty ID
        pub NextKittyId get(fn next_kitty_id): 32;
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
    pub enum Error for Module<T: Confif> {
        KittiesIdOverflow,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T:Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        // Create a new kitty
        #[weight = 1000]
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;

            // TODO: ensure kitty id does not overflow
            // return Err(Error::<T>::KittiesIdOverflow.into());
        }
    
    }
}
