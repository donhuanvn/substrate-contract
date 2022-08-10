#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod voting_contract {
    use ink_lang::utils::initialize_contract;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::Mapping;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct VotingContract {
        statement: String,
        threshold: u32,
        voted_count: u32,
        voted_parties: Mapping<AccountId, u32>,
    }

    impl VotingContract {
        #[ink(constructor)]
        pub fn new(init_statement: String, init_threshold: u32) -> Self {
            initialize_contract(|contract: &mut Self| {
                contract.statement = init_statement;
                contract.threshold = init_threshold;
                contract.voted_count = 0;
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            initialize_contract(|_| {})
        }

        #[ink(message)]
        pub fn get_statement(&self) -> String {
            self.statement.clone()
        }

        #[ink(message)]
        pub fn get_threshold(&self) -> u32 {
            self.threshold
        }

        #[ink(message)]
        pub fn get_current_vote_count(&self) -> u32 {
            self.voted_count
        }

        #[ink(message)]
        pub fn what_my_vote(&self) -> u32 {
            let caller = self.env().caller();
            self.voted_parties.get(&caller).unwrap_or_default()
        }

        #[ink(message)]
        pub fn vote(&mut self) {
            let caller = self.env().caller();
            let voted_value = self.voted_parties.get(&caller).unwrap_or_default();
            if voted_value < 2 {
                let party_count = voted_value + 1;
                self.voted_parties.insert(&caller, &party_count);
                self.voted_count += 1;
            }
        }

        #[ink(message)]
        pub fn check(&self) -> bool {
            self.voted_count >= self.threshold
        }

        /// Terminates with the caller as beneficiary.
        #[ink(message)]
        pub fn terminate_me(&mut self) {
            self.env().terminate_contract(self.env().caller());
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
            let voting_contract = VotingContract::default();
            assert_eq!(voting_contract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut voting_contract = VotingContract::new(false);
            assert_eq!(voting_contract.get(), false);
            voting_contract.flip();
            assert_eq!(voting_contract.get(), true);
        }
    }
}
