use crate::{mock::*, Error};
use rand::{thread_rng, Rng};
use frame_support::{assert_noop, assert_ok};
use frame_system::pallet_prelude::ensure_signed;

fn gen_address() -> [u8; 32] {
    let mut rng = thread_rng();
    rng.gen()
}

fn gen_address_hash(address: [u8; 32]) -> Vec<u8> {
    sp_core::blake2_128(&address).to_vec()
}

#[test]
fn adding_new_owner_self() {
	new_test_ext().execute_with(|| {
        let address = gen_address();
        let address_hash = gen_address_hash(address);
		assert_ok!(Nolik::add_owner(Origin::signed(1), address_hash, None));
	});
}

#[test]
fn adding_new_owner_other() {
	new_test_ext().execute_with(|| {
        let address = gen_address();
        let address_hash = gen_address_hash(address);
        let new_owner = Origin::signed(2);
        let new_account = ensure_signed(new_owner).unwrap();
		assert_ok!(Nolik::add_owner(Origin::signed(1), address_hash, Some(new_account)));
	});
}

#[test]
fn adding_duplicate_owner() {
	new_test_ext().execute_with(|| {
        let address = gen_address();
        let address_hash = gen_address_hash(address);
        let new_owner = Origin::signed(2);
        let new_account = ensure_signed(new_owner).unwrap();
		assert_ok!(Nolik::add_owner(Origin::signed(1), address_hash.clone(), Some(new_account)));
		assert_noop!(Nolik::add_owner(
            Origin::signed(1),
            address_hash,
            Some(new_account),
        ), Error::<Test>::AccountInOwners);
	});
}

#[test]
fn new_owner_adds_new_owner() {
	new_test_ext().execute_with(|| {
        let address = gen_address();
        let address_hash = gen_address_hash(address);
        let new_owner_2 = Origin::signed(2);
        let new_owner_3 = Origin::signed(3);
        let new_account_2 = ensure_signed(new_owner_2).unwrap();
        let new_account_3 = ensure_signed(new_owner_3).unwrap();
		assert_ok!(Nolik::add_owner(Origin::signed(1), address_hash.clone(), Some(new_account_2)));
		assert_ok!(Nolik::add_owner(Origin::signed(2), address_hash.clone(), Some(new_account_3)));
	});
}

#[test]
fn exceeding_owners_capacity() {
    new_test_ext().execute_with(|| {
        let address = gen_address();

        let mut i = 1;
        while i < 5 {
            let address_hash = gen_address_hash(address);
            let mut new_account = None;
            if i > 1 {
                let new_owner = Origin::signed(i);
                new_account = Some(ensure_signed(new_owner).unwrap());
            }

            assert_ok!(Nolik::add_owner(Origin::signed(1), address_hash, new_account));
            i += 1;
        }

        let address_hash = gen_address_hash(address);
        let new_owner = Origin::signed(5);
        let new_account = Some(ensure_signed(new_owner).unwrap());
        assert_noop!(
            Nolik::add_owner(Origin::signed(1), address_hash, new_account), 
            Error::<Test>::ExceedMaxAddressOwners,
        );
    });
}

#[test]
fn adding_same_address_to_whitelist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_1_hash = gen_address_hash(address_1);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_noop!(Nolik::add_to_whitelist(
            Origin::signed(1),
            address_1_hash.clone(),
            address_1_hash,
        ), Error::<Test>::SameAddress);
	});
}

#[test]
fn adding_same_address_to_blacklist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_1_hash = gen_address_hash(address_1);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_noop!(Nolik::add_to_blacklist(
            Origin::signed(1),
            address_1_hash.clone(),
            address_1_hash,
        ), Error::<Test>::SameAddress);
	});
}

#[test]
fn addming_adress_to_whitelist_not_by_owner() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_noop!(Nolik::add_to_whitelist(
            Origin::signed(2),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AddressNotOwned);
	});
}

#[test]
fn addming_adress_to_blacklist_not_by_owner() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_noop!(Nolik::add_to_blacklist(
            Origin::signed(2),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AddressNotOwned);
	});
}

#[test]
fn adding_to_whitelist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_ok!(Nolik::add_to_whitelist(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ));
	});
}

#[test]
fn exceeding_whitelist_capacity() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));

        let mut i = 0;
        while i < 4 {
            let address = gen_address();
            let address_hash = gen_address_hash(address);

            assert_ok!(Nolik::add_to_whitelist(
                Origin::signed(1),
                address_1_hash.clone(),
                address_hash,
            ));
            i += 1;
        }

		assert_noop!(Nolik::add_to_whitelist(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::ExceedMaxWhiteListAddress);
	});
}

#[test]
fn adding_duplicate_to_whitelist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_ok!(Nolik::add_to_whitelist(
            Origin::signed(1),
            address_1_hash.clone(),
            address_2_hash.clone(),
        ));
		assert_noop!(Nolik::add_to_whitelist(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AlreadyInWhiteList);
	});
}

#[test]
fn adding_to_blacklist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_ok!(Nolik::add_to_blacklist(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ));
	});
}

#[test]
fn exceeding_blacklist_capacity() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));

        let mut i = 0;
        while i < 4 {
            let address = gen_address();
            let address_hash = gen_address_hash(address);

            assert_ok!(Nolik::add_to_blacklist(
                Origin::signed(1),
                address_1_hash.clone(),
                address_hash,
            ));
            i += 1;
        }

		assert_noop!(Nolik::add_to_blacklist(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::ExceedMaxBlackListAddress);
	});
}

#[test]
fn adding_duplicate_to_blacklist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_ok!(Nolik::add_to_blacklist(
            Origin::signed(1),
            address_1_hash.clone(),
            address_2_hash.clone(),
        ));
		assert_noop!(Nolik::add_to_blacklist(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AlreadyInBlackList);
	});
}

#[test]
fn adding_to_whitelist_on_existance_in_blacklist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_ok!(Nolik::add_to_blacklist(
            Origin::signed(1),
            address_1_hash.clone(),
            address_2_hash.clone(),
        ));
		assert_noop!(Nolik::add_to_whitelist(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AlreadyInBlackList);
	});
}

#[test]
fn adding_to_blacklist_on_existance_in_whitelist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
		assert_ok!(Nolik::add_to_whitelist(
            Origin::signed(1),
            address_1_hash.clone(),
            address_2_hash.clone(),
        ));
		assert_noop!(Nolik::add_to_blacklist(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AlreadyInWhiteList);
	});
}

#[test]
fn send_message_to_myself() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
        assert_ok!(Nolik::send_message(
            Origin::signed(1),
            address_1_hash.clone(),
            address_1_hash,
            ipfs_id.to_vec(),
        ));
	});
}

#[test]
fn send_message_when_no_whitelist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
        assert_ok!(Nolik::add_owner(Origin::signed(2), address_2_hash.clone(), None));

        assert_ok!(Nolik::send_message(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
            ipfs_id.to_vec(),
        ));
	});
}

#[test]
fn send_message_when_not_in_whitelist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_3 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);
        let address_3_hash = gen_address_hash(address_3);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
        assert_ok!(Nolik::add_owner(Origin::signed(2), address_2_hash.clone(), None));

		assert_ok!(Nolik::add_to_whitelist(
            Origin::signed(2),
            address_2_hash.clone(),
            address_3_hash,
        ));

        assert_noop!(Nolik::send_message(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
            ipfs_id.to_vec(),
        ), Error::<Test>::AddressNotInWhiteList);
	});
}

#[test]
fn send_message_when_in_whitelist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
        assert_ok!(Nolik::add_owner(Origin::signed(2), address_2_hash.clone(), None));

		assert_ok!(Nolik::add_to_whitelist(
            Origin::signed(2),
            address_2_hash.clone(),
            address_1_hash.clone(),
        ));

        assert_ok!(Nolik::send_message(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
            ipfs_id.to_vec(),
        ));
	});
}

#[test]
fn send_message_when_in_blacklist() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone(), None));
        assert_ok!(Nolik::add_owner(Origin::signed(2), address_2_hash.clone(), None));

		assert_ok!(Nolik::add_to_blacklist(
            Origin::signed(2),
            address_2_hash.clone(),
            address_1_hash.clone(),
        ));

        assert_noop!(Nolik::send_message(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
            ipfs_id.to_vec(),
        ), Error::<Test>::AddressInBlackList);
	});
}
