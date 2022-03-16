//! # NEAR-SDK-RS 4.0 pre-release
//! 
//! Some helper for NEAR 4.0 after upgrading from previous
//! versions. 

use near_sdk::{env, PromiseResult, require};


/// Checks for successful promise. 
pub fn is_promise_success() -> bool {
  require!(
    env::promise_results_count() == 1,
    "Contract expected a result on the callback."
  );

  match env::promise_result(0) {
    PromiseResult::Successful(_) => true,
    _ => false,
  }
}


/// The equivalent of .expect() but a lightweight version
/// to reduce compiled-wasm size. 
pub fn expect_lightweight<T>(
  option: Option<T>,
  message: &str,
) -> T {
  option.unwrap_or_else(|| env::panic_str(message))
}


/// Assert predecessor is current, very frequently used
/// assertion. 
pub fn assert_predecessor_is_current(message: &str) {
  require!(
    env::predecessor_account_id() == env::current_account_id(),
    message
  )
}