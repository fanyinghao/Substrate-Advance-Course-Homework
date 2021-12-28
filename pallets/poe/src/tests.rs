use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
#[test]
fn create_claim_should_work() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(ALICE), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((ALICE, frame_system::Pallet::<Test>::block_number())),
		);
	});
}

#[test]
fn transer_claim_should_work() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(ALICE), claim.clone()));
		assert_ok!(PoeModule::trans_claim(Origin::signed(ALICE), claim.clone(),BOB));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((BOB, frame_system::Pallet::<Test>::block_number())),
		);
	});
}




#[test]
fn revoke_claim_should_work() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(ALICE), claim.clone()));
		assert_ok!(PoeModule::revoked_claim(Origin::signed(ALICE), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			None,
		);

	});
}

#[test]
fn create_claim_throw_claim_over_length() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2, 3,4,5,6];
		assert_noop!(PoeModule::create_claim(Origin::signed(ALICE), claim.clone()),Error::<Test>::ClaimOverLength);
	});
}
