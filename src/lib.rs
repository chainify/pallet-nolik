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

        /// The maximum amount of Addresses a single account can own.
        #[pallet::constant]
        type MaxAddressOwners: Get<u32>;

        /// The maximum amount of Addresses that can be added to whitelist
        #[pallet::constant]
        type MaxWhiteListAddress: Get<u32>;

        /// The maximum amount of Addresses that can be added to blacklist
        #[pallet::constant]
        type MaxBlackListAddress: Get<u32>;
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// Handles checking whether the address is owned by account
        AddressNotOwned,
        /// Handles checking whether the account is already in the list of owners
        AccountInOwners,
        /// Handles checking whether trying to add the same address to whitelist or blacklist
        SameAddress,
        /// Handles checking whether the address is in the blacklist of recipient
        AddressInBlackList,
        /// Handles checking whether the address is already in the whitelist
        AlreadyInWhiteList,
        /// Address is not in the whitelist
        AddressNotInWhiteList,
        /// Handles checking whether the address is already in the blacklist
        AlreadyInBlackList,
        /// An address cannot have more owners than `MaxAddressOwners`
        ExceedMaxAddressOwners,
        /// An account cannot add more than `MaxWhiteListAddress` addresses to the whitelist 
        ExceedMaxWhiteListAddress,
        /// An account cannot add more than `MaxBlackListAddress` addresses to the blacklist
        ExceedMaxBlackListAddress,
    }

    // Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new owner was added to address \[address, account_id\]
        AddOwner(Vec<u8>, T::AccountId),
        /// A new address was added to whitelist \[added_to, new_address\]
        AddWhiteList(Vec<u8>, Vec<u8>),
        /// A new address was added to blacklist \[added_to, new_address\]
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
    #[pallet::getter(fn whitelists)]
    /// Keeps track of addresse's whitelist.
    pub(super) type WhiteLists<T: Config> =
        StorageMap<_, Twox64Concat, Address, BoundedVec<Address, T::MaxAddressOwners>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn blacklists)]
    /// Keeps track of addresse's blacklist.
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

        /// Add new owner to address.
        ///
        /// One address can have multiple owners.
        /// The maximum number of owners is specified in `MaxAddressOwners` parameter.
        /// Only the owner of an address can add new owners.
        /// If an address does not have any owners anyone can become the owner.
        /// The message can be sent only from an address that has an owner.
        /// Each time the message is sent, the network validates the owner before broadcasting the message.
        #[pallet::weight(10_000)]
        pub fn add_owner(
            origin: OriginFor<T>,
            address: Vec<u8>,
            owner: Option<T::AccountId>,
            ) -> DispatchResult {

            let account = ensure_signed(origin)?;
            let new_owner = match owner {
                Some(res) => res,
                None => account.clone()
            };

            let has_owners = Self::has_owners(&address);
            if has_owners {
                ensure!(Self::is_owned(&new_owner, &address) == false, <Error<T>>::AccountInOwners);
                ensure!(Self::is_owned(&account, &address), <Error<T>>::AddressNotOwned);
            }
            

            <AddressOwners<T>>::try_mutate(&address, |owner_vec| {
                owner_vec.try_push(new_owner.clone())
            }).map_err(|_| <Error<T>>::ExceedMaxAddressOwners)?;

            Self::deposit_event(Event::AddOwner(address, new_owner));
            Ok(())
        }

        /// Add address to the whitelist.
        ///
        /// A whitelist is a set of addresses that have permission to send the message to a particular recipient.
        /// If the whitelist is empty the message will be accepted from any sender unless it is not in the blacklist.
        /// If the whitelist is not empty the message will be accepted only from addresses in list.
        /// The same address cannot be on the whitelist and in the blacklist at the same time.
        #[pallet::weight(10_000)]
        pub fn add_to_whitelist(
            origin: OriginFor<T>,
            add_to: Vec<u8>,
            new_address: Vec<u8>,
            ) -> DispatchResult {

            let account = ensure_signed(origin)?;

            ensure!(add_to != new_address, <Error<T>>::SameAddress);
            ensure!(Self::is_owned(&account, &add_to), <Error<T>>::AddressNotOwned);
            ensure!(Self::not_in_whitelist(&add_to, &new_address), <Error<T>>::AlreadyInWhiteList);
            ensure!(Self::not_in_blacklist(&add_to, &new_address), <Error<T>>::AlreadyInBlackList);

            <WhiteLists<T>>::try_mutate(&add_to, |address_vec| {
                address_vec.try_push(new_address.clone())
            }).map_err(|_| <Error<T>>::ExceedMaxWhiteListAddress)?;

            Self::deposit_event(Event::AddWhiteList(add_to, new_address));
            Ok(())
        }

        /// Add address to the blacklist.
        ///
        /// It's a hash of an address that adds other addresses to its blacklist.
        /// A blacklist is a set of addresses that DO NOT have permission to send the message to a particular address.
        /// If the blacklist is empty the message will be accepted from any sender unless there are no whitelist restrictions.
        /// The same address cannot be on the blacklist and in the whitelist at the same time.
        #[pallet::weight(10_000)]
        pub fn add_to_blacklist(
            origin: OriginFor<T>,
            add_to: Vec<u8>,
            new_address:Vec<u8>,
            ) -> DispatchResult {

            let account = ensure_signed(origin)?;

            ensure!(add_to != new_address, <Error<T>>::SameAddress);
            ensure!(Self::is_owned(&account, &add_to), <Error<T>>::AddressNotOwned);
            ensure!(Self::not_in_whitelist(&add_to, &new_address), <Error<T>>::AlreadyInWhiteList);
            ensure!(Self::not_in_blacklist(&add_to, &new_address), <Error<T>>::AlreadyInBlackList);

            <BlackLists<T>>::try_mutate(&add_to, |address_vec| {
                address_vec.try_push(new_address.clone())
            }).map_err(|_| <Error<T>>::ExceedMaxBlackListAddress)?;

            Self::deposit_event(Event::AddBlackList(add_to, new_address));
            Ok(())
        }

        /// Send the message.
        ///
        /// This sender's address should be owned by the account.
        /// If it is not owned the message will be rejected by the network.
        /// The message will be received only if the sender has a right to send the message.
        /// Those rights are controlled by the recipient through a whitelist and a blacklist of senders.
        /// If the sender does not have a right to send the message it will be rejected by the network.
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
            ensure!(Self::not_in_blacklist(&recipient, &sender), <Error<T>>::AddressInBlackList);

            let with_whitelist = Self::has_whitelist(&recipient);
            if with_whitelist {
                ensure!(Self::in_whitelist(&recipient, &sender), <Error<T>>::AddressNotInWhiteList);
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

        pub fn has_whitelist(address: &Vec<u8>) -> bool {
            let whitelist = <WhiteLists<T>>::get(address).into_inner();
            match whitelist.len() > 0 {
                true => true,
                false => false,
            }
        }

        pub fn in_whitelist(recipient: &Vec<u8>, sender: &Vec<u8>) -> bool {
            let whitelist  = <WhiteLists<T>>::get(recipient).into_inner();
            match whitelist.iter().any(|el| el == sender) {
                true => true,
                false => false,
            }
        }

        pub fn not_in_whitelist(add_to: &Vec<u8>, new_address: &Vec<u8>) -> bool {
            let whitelist = <WhiteLists<T>>::get(add_to).into_inner();
            match whitelist.iter().any(|el| el == new_address) {
                true => false,
                false => true,
            }
        }

        pub fn not_in_blacklist(add_to: &Vec<u8>, new_address: &Vec<u8>) -> bool {
            let blacklist = <BlackLists<T>>::get(add_to).into_inner();
            match blacklist.iter().any(|el| el == new_address) {
                true => false,
                false => true,
            }
        }

    }
}
