#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc165 {
    use ink::env::hash::{HashOutput, Keccak256};
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct ERC165 {
        /// Mapping to store supported interface IDs
        supported_interfaces: Mapping<[u8; 4], bool>,
    }

    impl ERC165 {
        /// Constructor to initialize the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self {
                supported_interfaces: Mapping::default(),
            };

            // Register the ERC165 interface itself
            let erc165_interface_id = Self::compute_interface_id(b"supports_interface([u8;4])");
            instance
                .supported_interfaces
                .insert(erc165_interface_id, &true);

            instance
        }

        /// Function to compute the 4-byte interface ID
        pub fn compute_interface_id(signature: &[u8]) -> [u8; 4] {
            // Prepare a buffer for the hash result
            let mut output = <Keccak256 as HashOutput>::Type::default();

            // Perform the hashing
            ink::env::hash_bytes::<Keccak256>(signature, &mut output);

            // Return the first 4 bytes as the interface ID
            [output[0], output[1], output[2], output[3]]
        }

        /// Function to add a new supported interface
        #[ink(message)]
        pub fn add_interface(&mut self, interface_id: [u8; 4]) {
            // Check if the interface ID is already added
            if self.supported_interfaces.contains(&interface_id) {
                panic!("Interface ID already exists");
            }
            self.supported_interfaces.insert(interface_id, &true);
        }

        /// Function to check if an interface is supported
        #[ink(message)]
        pub fn supports_interface(&self, interface_id: [u8; 4]) -> bool {
            self.supported_interfaces.get(interface_id).unwrap_or(false)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_supports_interface() {
            let mut contract = ERC165::new();

            // Compute a sample interface ID
            let sample_id = ERC165::compute_interface_id(b"sample_function()");

            // Initially, the sample ID is not supported
            assert!(!contract.supports_interface(sample_id));

            // Add the sample ID and check again
            contract.add_interface(sample_id);
            assert!(contract.supports_interface(sample_id));
        }
    }
}
