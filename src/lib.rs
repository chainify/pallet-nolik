#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::transactional;
    use scale_info::prelude::vec::Vec;

    type Address = Vec<u8>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The maximum amount of Addresses a sinle account can own.
        #[pallet::constant]
        type MaxAddressOwners: Get<u32>;

        /// The maximum amount of Addresses that can be added to white list
        #[pallet::constant]
        type MaxWhiteListAddress: Get<u32>;

        /// The maximum amount of Addresses that can be added to white list
        #[pallet::constant]
        type MaxBlackListAddress: Get<u32>;
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// Handles checking whether the address is owned by account
        AddressNotOwned,
        /// Handkes checking whether trying to add the same address to white list or black list
        SameAddress,
        /// Handles checking whether the address in black list of recipient
        AddressInBlackList,
        /// Handles checking whether the address is already in white list
        AlreadyInWhiteList,
        /// Address is not in white list
        AddressNotInWhiteList,
        /// Handles checking whether the address is already in black list
        AlreadyInBlackList,
        /// An account cannot own more Addresses than `MaxAddressOwned`
        ExceedMaxAddressOwners,
        /// An account cannot add Addresses to White List more than `MaxWhiteListAddress`
        ExceedMaxWhiteListAddress,
        /// An account cannot add Addresses to Black List more than `MaxWhiteListAddress`
        ExceedMaxBlackListAddress,
    }

    // Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new owner was added to address \[address, account_id\]
        AddOwner(Vec<u8>, T::AccountId),
        /// A new address was added to white list \[added_to, new_address\]
        AddWhiteList(Vec<u8>, Vec<u8>),
        /// A new address was added to black list \[added_to, new_address\]
        AddBlackList(Vec<u8>, Vec<u8>),
        /// A new message was sent \[sender, recipient, ipfs_hash\]
        MessageSent(Vec<u8>, Vec<u8>, Vec<u8>),
    }

    #[pallet::storage]
    #[pallet::getter(fn addresses_owned)]
    /// Keeps track of address owners.
    pub(super) type AddressOwners<T: Config> =
        StorageMap<_, Twox64Concat, Address, BoundedVec<T::AccountId, T::MaxAddressOwners>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn white_lists)]
    /// Keeps track of addresse's white list.
    pub(super) type WhiteLists<T: Config> =
        StorageMap<_, Twox64Concat, Address, BoundedVec<Address, T::MaxAddressOwners>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn black_lists)]
    /// Keeps track of addresse's black list.
    pub(super) type BlackLists<T: Config> =
        StorageMap<_, Twox64Concat, Address, BoundedVec<Address, T::MaxAddressOwners>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn messages_count)]
    /// Keep track of the number of messages (sent + received) per address.
    pub(super) type MessagesCount<T: Config> = StorageMap<_, Twox64Concat, Address, u32>;

    #[pallet::storage]
    #[pallet::getter(fn messages)]
    /// Keeps track of sent and received messages.
    pub(super) type Messages<T: Config> =
        StorageDoubleMap<_, Twox64Concat, Address, Twox64Concat, u32, Vec<u8>>;


    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000)]
        pub fn add_owner(
            origin: OriginFor<T>,
            address: Vec<u8>,
            ) -> DispatchResult {

            let owner = ensure_signed(origin)?;

            let has_owners = Self::has_owners(&address);
            if has_owners {
                ensure!(Self::is_owned(&owner, &address), <Error<T>>::AddressNotOwned);
            }
            

            <AddressOwners<T>>::try_mutate(&address, |owner_vec| {
                owner_vec.try_push(owner.clone())
            }).map_err(|_| <Error<T>>::ExceedMaxAddressOwners)?;

            Self::deposit_event(Event::AddOwner(address, owner));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn add_to_white_list(
            origin: OriginFor<T>,
            add_to: Vec<u8>,
            new_address: Vec<u8>,
            ) -> DispatchResult {

            let account = ensure_signed(origin)?;

            ensure!(add_to != new_address, <Error<T>>::SameAddress);
            ensure!(Self::is_owned(&account, &add_to), <Error<T>>::AddressNotOwned);
            ensure!(Self::not_in_white_list(&add_to, &new_address), <Error<T>>::AlreadyInWhiteList);
            ensure!(Self::not_in_black_list(&add_to, &new_address), <Error<T>>::AlreadyInBlackList);

            <WhiteLists<T>>::try_mutate(&add_to, |address_vec| {
                address_vec.try_push(new_address.clone())
            }).map_err(|_| <Error<T>>::ExceedMaxWhiteListAddress)?;

            Self::deposit_event(Event::AddWhiteList(add_to, new_address));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn add_to_black_list(
            origin: OriginFor<T>,
            add_to: Vec<u8>,
            new_address:Vec<u8>,
            ) -> DispatchResult {

            let account = ensure_signed(origin)?;

            ensure!(add_to != new_address, <Error<T>>::SameAddress);
            ensure!(Self::is_owned(&account, &add_to), <Error<T>>::AddressNotOwned);
            ensure!(Self::not_in_white_list(&add_to, &new_address), <Error<T>>::AlreadyInWhiteList);
            ensure!(Self::not_in_black_list(&add_to, &new_address), <Error<T>>::AlreadyInBlackList);

            <BlackLists<T>>::try_mutate(&add_to, |address_vec| {
                address_vec.try_push(new_address.clone())
            }).map_err(|_| <Error<T>>::ExceedMaxBlackListAddress)?;

            Self::deposit_event(Event::AddBlackList(add_to, new_address));
            Ok(())
        }

        #[transactional]
        #[pallet::weight(10_000)]
        pub fn send_message(
            origin: OriginFor<T>,
            sender: Vec<u8>,
            recipient: Vec<u8>,
            ipfs_id: Vec<u8>,
            ) -> DispatchResult {
            
            let account = ensure_signed(origin)?;
            ensure!(Self::is_owned(&account, &sender), <Error<T>>::AddressNotOwned);
            ensure!(Self::not_in_black_list(&recipient, &sender), <Error<T>>::AddressInBlackList);

            let with_white_list = Self::has_white_list(&recipient);
            if with_white_list {
                ensure!(Self::in_white_list(&recipient, &sender), <Error<T>>::AddressNotInWhiteList);
            }

            let sender_messages_count = match Self::messages_count(&sender) {
                Some(v) => v + 1,
                None => 1,
            };

            let recipient_messages_count = match Self::messages_count(&recipient) {
                Some(v) => v + 1,
                None => 1,
            };

            <Messages<T>>::insert(&sender, &sender_messages_count, &ipfs_id);
            <Messages<T>>::insert(&recipient, &recipient_messages_count, &ipfs_id);
            <MessagesCount<T>>::insert(&sender, sender_messages_count);
            <MessagesCount<T>>::insert(&recipient, recipient_messages_count);

            Self::deposit_event(Event::MessageSent(sender, recipient, ipfs_id));
            Ok(())
        }

    }



    impl<T: Config> Pallet<T> {
        pub fn has_owners(address: &Vec<u8>) -> bool {
            let owners = <AddressOwners<T>>::get(address).into_inner();
            match owners.len() > 0 {
                true => true,
                false => false,
            }
        }
        pub fn is_owned(account: &T::AccountId, address: &Vec<u8>) -> bool {
            let owners = <AddressOwners<T>>::get(address).into_inner();
            match owners.iter().any(|el| el == account) {
                true => true,
                false => false,
            }
        }

        pub fn has_white_list(address: &Vec<u8>) -> bool {
            let white_list = <WhiteLists<T>>::get(address).into_inner();
            match white_list.len() > 0 {
                true => true,
                false => false,
            }
        }

        pub fn in_white_list(list_of: &Vec<u8>, list_el: &Vec<u8>) -> bool {
            let white_list  = <WhiteLists<T>>::get(list_of).into_inner();
            match white_list.iter().any(|el| el == list_el) {
                true => true,
                false => false,
            }
        }

        pub fn not_in_white_list(list_of: &Vec<u8>, list_el: &Vec<u8>) -> bool {
            let white_list = <WhiteLists<T>>::get(list_of).into_inner();
            match white_list.iter().any(|el| el == list_el) {
                true => false,
                false => true,
            }
        }

        pub fn not_in_black_list(list_of: &Vec<u8>, list_el: &Vec<u8>) -> bool {
            let black_list = <BlackLists<T>>::get(list_of).into_inner();
            match black_list.iter().any(|el| el == list_el) {
                true => false,
                false => true,
            }
        }

    }
}
