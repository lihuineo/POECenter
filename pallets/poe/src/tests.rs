use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::ConstU32};
use sp_runtime::BoundedVec;

#[test]
fn create_claim_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let data = BoundedVec::<u8, ConstU32<512>>::try_from(vec![1; 8].clone()).unwrap();
		let account = 123u64;

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(account), data.clone()));
		assert_eq!(PoeModule::proofs(&data).unwrap().0, account);
		System::assert_last_event(Event::ClaimCreated(account, data.clone()).into());

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(account), data.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	});
}

#[test]
fn revoke_claim_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let data = BoundedVec::<u8, ConstU32<512>>::try_from(vec![1; 8].clone()).unwrap();
		let x_account = 123u64;
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(x_account), data.clone());

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(x_account), data.clone()));
		System::assert_last_event(Event::ClaimRevoked(x_account, data.clone()).into());

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(x_account), data.clone()),
			Error::<Test>::ClaimNotExist
		);

		let y_account = 456u64;
		_ = PoeModule::create_claim(RuntimeOrigin::signed(y_account), data.clone());
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(x_account), data.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let data = BoundedVec::<u8, ConstU32<512>>::try_from(vec![1; 8].clone()).unwrap();
		let x_account = 123u64;
		let y_account = 456u64;
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(x_account), data.clone());

		assert_ok!(PoeModule::transfer_claim(
			RuntimeOrigin::signed(x_account),
			data.clone(),
			y_account
		));
		System::assert_last_event(
			Event::ClaimTransferred(x_account, data.clone(), y_account).into(),
		);
		assert_ne!(PoeModule::proofs(&data).unwrap().0, x_account);
		assert_eq!(PoeModule::proofs(&data).unwrap().0, y_account);

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(y_account), data.clone(), y_account),
			Error::<Test>::NotNeedTransfer
		);
	})
}
