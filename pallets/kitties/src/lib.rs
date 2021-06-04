#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::decl_module;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {

}


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

    #[pallet::config]
    decl_module! {
        pub struct Module<T: Config> for enum Call where origin: T:Origin {
        }
    }
}