#![cfg_attr(not(feature = "std"), no_std, no_main)]


use ink::prelude::vec::Vec;
use ink::{
    env::{
        call::{build_call, ExecutionInput, FromAccountId, Selector},
        DefaultEnvironment, Error as InkEnvError,
    },
    LangError,
    primitives::{AccountId, Hash}
};

#[ink::contract]
mod shop_contract {
    use super::*;
    use ink::codegen::TraitCallBuilder;
    use ink::env::call::FromAccountId;
    pub type TokenId = u128;

    /// Error for when the beneficiary is a zero address.
    /// & Error for when the releasable balance is zero.
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        InvalidBeneficiary,
        ZeroReleasableBalance,
    }

    
    #[ink(storage)]
    #[derive(Default)]
    pub struct TherasShop {
        // other_contract: OtherContractRef,
        // contract signer -> OFFCHAIN
        // FEE
    }

    impl TherasShop {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// this is a cross call contract to erc1155 starship or erc1155 materials and other
        /// todo: add payments erc20 and psp-22 beside native
        /// todo: add offchain signer 
        #[ink(message, payable)]
        pub fn buy_product(&mut self, contract_address: AccountId, token_id: TokenId, value: Balance) {
            let payment = self.env().transferred_value();
            assert!(payment >= value, "Insufficient payment");

            let mint_selector = Selector::new(ink::selector_bytes!("mint"));

            let result = build_call::<DefaultEnvironment>()
                .call(contract_address)
                .exec_input(
                    ExecutionInput::new(mint_selector)
                        .push_arg(token_id)
                        .push_arg(value),
                )
                .returns::<Result<(), InkEnvError>>()
                .invoke();
                // .try_invoke();

            match result {
                Ok(_) => {
                    // Handle successful minting
                },
                Err(e) => {
                    // Handle the error
                },
            }
                
            // supposed to transfer into respective contract instead
            self.env().transfer(self.env().account_id(), payment).unwrap();
        }

        /// similar to buy_product will execute with offchain signer and claim products
        #[ink(message, payable)]
        pub fn claim_product(&mut self, contract_address: AccountId, token_id: TokenId) {

            let mint_selector = Selector::new(ink::selector_bytes!("mint"));

            let result = build_call::<DefaultEnvironment>()
                .call(contract_address)
                .exec_input(
                    ExecutionInput::new(mint_selector)
                        .push_arg(token_id)
                        .push_arg(value),
                )
                .returns::<Result<(), InkEnvError>>()
                .invoke();
                // .try_invoke();

            match result {
                Ok(_) => {
                    // Handle successful minting
                },
                Err(e) => {
                    // Handle the error
                },
            }
                
        }

    }
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;
