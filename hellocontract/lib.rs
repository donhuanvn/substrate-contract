#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_lang::utils::initialize_contract;
use ink_prelude::vec::Vec;
use ink_storage::traits::SpreadAllocate;
use ink_storage::Mapping;
use ink_prelude::string::ToString;
use ink_prelude::string::String;

#[ink::contract]
mod hellocontract {
    use super::*;

    #[ink(event)]
    pub struct Transferred {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Hellocontract {
        /// Stores a single `bool` value on the storage.
        value: bool,
        a: i32,
        b: Vec<i32>,
        map: Mapping<AccountId, u32>,
        total_supply: Balance,
        description: String,
        statement: Hash
    }

    impl Hellocontract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            init_value: bool,
            init_a: i32,
            init_b: Vec<i32>,
            count: u32,
            initial_supply: Balance,
            description: String,
            statement: Hash
        ) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();

                Self::env().emit_event(Transferred {
                    from: None,
                    to: Some(caller),
                    value: initial_supply
                });

                contract.map.insert(&caller, &count);
                contract.value = init_value;
                contract.a = init_a;
                contract.b = init_b;
                contract.description = description;
                contract.statement = statement;
            })
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            initialize_contract(|contract: &mut Self| {
                *contract = Self::new(
                    true, 
                    -2, 
                    Default::default(),
                    10u32,
                    1000,
                    "Test it".to_string(),
                    Hash::from([0x99; 32]),
                );
            })
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn set_number(&mut self, number: i32) {
            self.a = number;
        }

        #[ink(message)]
        pub fn get_number(&self) -> i32 {
            let from = self.env().caller();

            self.env().emit_event(Transferred {
                from: Some(from),
                to: None,
                value: 10
            });

            self.a
        }

        #[ink(message)]
        pub fn get_vector_item(&self, index: u32) -> i32 {
            if index >= self.b.len() as u32 {
                return -1;
            }
            self.b[index as usize]
        }

        #[ink(message)]
        pub fn get_map_item(&self) -> u32 {
            let caller = Self::env().caller();
            self.map.get(&caller).unwrap_or(u32::MAX)
        }

        #[ink(message)]
        pub fn get_string(&self) -> String {
            self.description.clone()
        }

        #[ink(message)]
        pub fn get_hash(&self) -> Hash {
            self.statement
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let hellocontract = Hellocontract::default();
            assert_eq!(hellocontract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut hellocontract = Hellocontract::new(false);
            assert_eq!(hellocontract.get(), false);
            hellocontract.flip();
            assert_eq!(hellocontract.get(), true);
        }
    }
}
