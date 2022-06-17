// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(min_specialization)]
#[cfg(feature = "psp35")]
#[openbrush::contract]
mod psp35_enumerable {
    use ink_lang as ink;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp35::{
            extensions::{
                batch::*,
                burnable::*,
                enumerable::*,
                mintable::*,
            },
            Id,
        },
        test_utils::{
            accounts,
            change_caller,
        },
    };

    #[derive(Default, SpreadAllocate, PSP35Storage)]
    #[ink(storage)]
    pub struct PSP35Struct {
        #[PSP35StorageField]
        psp35: PSP35Data<EnumerableBalances>,
    }

    impl PSP35Internal for PSP35Struct {
        fn _do_safe_transfer_check(
            &mut self,
            _operator: &AccountId,
            _from: &AccountId,
            _to: &AccountId,
            _ids_amounts: &Vec<(Id, Balance)>,
            _data: &Vec<u8>,
        ) -> Result<(), PSP35Error> {
            Ok(())
        }
    }

    impl PSP35 for PSP35Struct {}

    impl PSP35Mintable for PSP35Struct {}

    impl PSP35Burnable for PSP35Struct {}

    impl PSP35Batch for PSP35Struct {}

    impl PSP35Enumerable for PSP35Struct {}

    impl PSP35Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }

    #[ink::test]
    fn enumerable_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let nft = PSP35Struct::new();
        // check that alice does not have token by index
        assert_eq!(
            nft.owners_token_by_index(accounts.alice, 0u128),
            Err(PSP35Error::TokenNotExists)
        );
        // token by index 1 does not exists
        assert_eq!(nft.token_by_index(0u128), Err(PSP35Error::TokenNotExists));
    }

    #[ink::test]
    fn enumerable_mint_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();

        let token_id = Id::U128(1);

        // Create token Id 1 for Alice
        assert!(nft._mint_to(accounts.alice, vec![(token_id.clone(), 20)]).is_ok());
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(token_id.clone()));
        // check token by index
        assert_eq!(nft.token_by_index(0u128), Ok(token_id));
    }

    #[ink::test]
    fn enumerable_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        // Create token Id 1 and Id 2 for Alice

        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_amount1 = 1;
        let token_amount2 = 20;

        assert!(nft
            ._mint_to(
                accounts.alice,
                vec![(token_id1.clone(), token_amount1), (token_id2.clone(), token_amount2)]
            )
            .is_ok());
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(token_id1.clone()));
        // act. transfer token from alice to bob
        assert!(nft
            .transfer(accounts.bob, token_id1.clone(), token_amount1, vec![])
            .is_ok());
        // bob owns token
        assert_eq!(nft.owners_token_by_index(accounts.bob, 0u128), Ok(token_id1));
        // alice does not own token Id 1
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(token_id2.clone()));
        assert_eq!(
            nft.owners_token_by_index(accounts.alice, 1u128),
            Err(PSP35Error::TokenNotExists)
        );
        // act. transfer token from alice to alice
        assert!(nft.transfer(accounts.bob, token_id2.clone(), 10, vec![]).is_ok());
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(token_id2.clone()));
        // check Bob token by index
        assert_eq!(nft.owners_token_by_index(accounts.bob, 1u128), Ok(token_id2));
    }

    #[ink::test]
    fn enumerable_batch_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        // Create token Id 1 and Id 2 for Alice

        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_amount1 = 1;
        let token_amount2 = 20;

        assert!(nft
            ._mint_to(
                accounts.alice,
                vec![(token_id1.clone(), token_amount1), (token_id2.clone(), token_amount2)]
            )
            .is_ok());
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(token_id1.clone()));
        // act. transfer token from alice to bob
        assert!(nft
            .transfer(accounts.bob, token_id1.clone(), token_amount1, vec![])
            .is_ok());
        // bob owns token
        assert_eq!(nft.owners_token_by_index(accounts.bob, 0u128), Ok(token_id1));
        // alice does not own token Id 1
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(token_id2.clone()));
        assert_eq!(
            nft.owners_token_by_index(accounts.alice, 1u128),
            Err(PSP35Error::TokenNotExists)
        );
        // act. transfer token from alice to alice
        assert!(nft
            .transfer(accounts.alice, token_id2.clone(), token_amount2, vec![])
            .is_ok());
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(token_id2));
    }

    #[ink::test]
    fn token_by_index_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();

        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let token_id3 = Id::U128(3);
        let token_amount1 = 1u128;
        let token_amount2 = 1u128;
        let token_amount3 = 1u128;

        // Create token Id 1 for Alice
        assert!(nft
            ._mint_to(
                accounts.alice,
                vec![
                    (token_id1.clone(), token_amount1),
                    (token_id2.clone(), token_amount2),
                    (token_id3.clone(), token_amount3)
                ]
            )
            .is_ok());

        assert!(nft
            .transfer(accounts.bob, token_id1.clone(), token_amount1, vec![])
            .is_ok());
        assert!(nft
            .transfer(accounts.bob, token_id3.clone(), token_amount3, vec![])
            .is_ok());
        change_caller(accounts.bob);
        assert!(nft
            .transfer(accounts.alice, token_id1.clone(), token_amount1, vec![])
            .is_ok());
        assert!(nft.burn(accounts.alice, vec![(token_id2, token_amount2)]).is_ok());
        assert!(nft
            .transfer(accounts.alice, token_id3.clone(), token_amount3, vec![])
            .is_ok());
        change_caller(accounts.alice);
        assert!(nft
            .transfer(accounts.bob, token_id3.clone(), token_amount3, vec![])
            .is_ok());
        // alice does not own token
        assert_eq!(nft.token_by_index(0u128), Ok(token_id1));
        assert_eq!(nft.token_by_index(1u128), Ok(token_id3));
        assert_eq!(nft.token_by_index(2u128), Err(PSP35Error::TokenNotExists));
    }

    #[ink::test]
    fn enumerable_burn_works() {
        let accounts = accounts();
        let token_id = Id::U128(1);
        let token_amount = 1u128;
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft
            ._mint_to(accounts.alice, vec![(token_id.clone(), token_amount)])
            .is_ok());
        // alice still owns token id 1
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(token_id.clone()));
        // index 0 points to token with id 1
        assert_eq!(nft.token_by_index(0u128), Ok(token_id.clone()));
        // Destroy token Id 1.
        assert!(nft.burn(accounts.alice, vec![(token_id, token_amount)]).is_ok());
        // alice does not owns any tokens
        assert_eq!(
            nft.owners_token_by_index(accounts.alice, 0u128),
            Err(PSP35Error::TokenNotExists)
        );
        // token by index 1 does not exists
        assert_eq!(nft.token_by_index(0u128), Err(PSP35Error::TokenNotExists));
    }
}
