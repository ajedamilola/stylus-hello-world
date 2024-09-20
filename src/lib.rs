//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Counter{
        uint256 number;
    }
}

#[public]
impl Counter {
    pub fn number(&mut self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }
    pub fn set_number(&mut self, number: U256) {
        self.number.set(number);
    }

    pub fn increment(&mut self) {
        let former = self.number().unwrap();
        self.number.set(former + U256::from(1));
    }

    pub fn decrement(&mut self) {
        let former = self.number().unwrap();
        self.number.set(former - U256::from(1));
    }
}
