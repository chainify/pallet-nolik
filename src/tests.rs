use crate::{mock::*, Error};
use rand::{thread_rng, Rng};
use frame_support::{assert_noop, assert_ok};
//use crate::{AddressOwners, WhiteLists, BlackLists, Messages};

fn gen_address() -> [u8; 32] {
    let mut rng = thread_rng();
    rng.gen()
}

fn gen_address_hash(address: [u8; 32]) -> Vec<u8> {
    sp_core::blake2_128(&address).to_vec()
}

#[test]
fn adding_new_owner() {
	new_test_ext().execute_with(|| {
        let address = gen_address();
        let address_hash = gen_address_hash(address);
		assert_ok!(Nolik::add_owner(Origin::signed(1), address_hash));
	});
}

#[test]
fn exceeding_owners_capacity() {
    new_test_ext().execute_with(|| {
        let address = gen_address();

        let mut i = 0;
        while i < 4 {
            let address_hash = gen_address_hash(address);
            assert_ok!(Nolik::add_owner(Origin::signed(1), address_hash));
            i += 1;
        }

        let address_hash = gen_address_hash(address);
        assert_noop!(
            Nolik::add_owner(Origin::signed(1), address_hash), 
            Error::<Test>::ExceedMaxAddressOwners,
        );
    });
}

#[test]
fn adding_to_white_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
		assert_ok!(Nolik::add_to_white_list(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ));
	});
}

#[test]
fn exceeding_white_list_capacity() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));

        let mut i = 0;
        while i < 4 {
            let address = gen_address();
            let address_hash = gen_address_hash(address);

            assert_ok!(Nolik::add_to_white_list(
                Origin::signed(1),
                address_1_hash.clone(),
                address_hash,
            ));
            i += 1;
        }

		assert_noop!(Nolik::add_to_white_list(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::ExceedMaxWhiteListAddress);
	});
}

#[test]
fn adding_duplicate_to_white_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
		assert_ok!(Nolik::add_to_white_list(
            Origin::signed(1),
            address_1_hash.clone(),
            address_2_hash.clone(),
        ));
		assert_noop!(Nolik::add_to_white_list(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AlreadyInWhiteList);
	});
}

#[test]
fn adding_to_black_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
		assert_ok!(Nolik::add_to_black_list(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ));
	});
}

#[test]
fn exceeding_black_list_capacity() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));

        let mut i = 0;
        while i < 4 {
            let address = gen_address();
            let address_hash = gen_address_hash(address);

            assert_ok!(Nolik::add_to_black_list(
                Origin::signed(1),
                address_1_hash.clone(),
                address_hash,
            ));
            i += 1;
        }

		assert_noop!(Nolik::add_to_black_list(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::ExceedMaxBlackListAddress);
	});
}

#[test]
fn adding_duplicate_to_black_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
		assert_ok!(Nolik::add_to_black_list(
            Origin::signed(1),
            address_1_hash.clone(),
            address_2_hash.clone(),
        ));
		assert_noop!(Nolik::add_to_black_list(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AlreadyInBlackList);
	});
}

#[test]
fn adding_to_white_list_on_existance_in_black_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
		assert_ok!(Nolik::add_to_black_list(
            Origin::signed(1),
            address_1_hash.clone(),
            address_2_hash.clone(),
        ));
		assert_noop!(Nolik::add_to_white_list(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AlreadyInBlackList);
	});
}

#[test]
fn adding_to_black_list_on_existance_in_white_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
		assert_ok!(Nolik::add_to_white_list(
            Origin::signed(1),
            address_1_hash.clone(),
            address_2_hash.clone(),
        ));
		assert_noop!(Nolik::add_to_black_list(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
        ), Error::<Test>::AlreadyInWhiteList);
	});
}

#[test]
fn send_message_when_no_white_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
        assert_ok!(Nolik::add_owner(Origin::signed(2), address_2_hash.clone()));

        assert_ok!(Nolik::send_message(
            Origin::signed(1),
            address_1_hash,
            address_2_hash,
            ipfs_id.to_vec(),
        ));
	});
}

#[test]
fn send_message_when_not_in_white_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_3 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);
        let address_3_hash = gen_address_hash(address_3);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
        assert_ok!(Nolik::add_owner(Origin::signed(2), address_2_hash.clone()));

		assert_ok!(Nolik::add_to_white_list(
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
fn send_message_when_in_white_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
        assert_ok!(Nolik::add_owner(Origin::signed(2), address_2_hash.clone()));

		assert_ok!(Nolik::add_to_white_list(
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
fn send_message_when_in_black_list() {
	new_test_ext().execute_with(|| {
        let address_1 = gen_address();
        let address_2 = gen_address();
        let address_1_hash = gen_address_hash(address_1);
        let address_2_hash = gen_address_hash(address_2);
        let ipfs_id = "QmcpfNLr43wdKMLbJ4nu4yBDKDxQggSRcLVEoUYFcjJNZR".as_bytes();

        assert_ok!(Nolik::add_owner(Origin::signed(1), address_1_hash.clone()));
        assert_ok!(Nolik::add_owner(Origin::signed(2), address_2_hash.clone()));

		assert_ok!(Nolik::add_to_black_list(
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
