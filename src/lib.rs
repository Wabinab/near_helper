//! # NEAR-SDK-RS 4.0 pre-release
//! 
//! Some helper for NEAR 4.0 after upgrading from previous
//! versions. 

use near_sdk::{env, require, utils};

/// Checks for successful promise. 
#[deprecated(
  since="0.2.0", 
  note="please use near_sdk::utils::is_promise_success"
)]
pub fn is_promise_success() -> bool {
    utils::is_promise_success()
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
/// Similar to near_sdk::utils::assert_self except you can
/// enter a custom message for claribility. 
pub fn assert_predecessor_is_current(message: &str) {
    require!(
      env::predecessor_account_id() == env::current_account_id(),
      message
    )
}


/// Converts yoctoNEAR to NEAR. 
/// If there are many decimal places, will only leave the "first 5 decimals."
/// 
/// Note since String indexing isn't available, if you misuse it with inserting
/// random non-ASCII value or with non-numbers, it'll panic/fail. 
/// 
/// And since we return f64, if your value is outside this range (very small
/// number like 1000 yoctoNEAR), it won't panic, but there might be floating
/// point errors which this function is not capable of detecting. 
/// 
/// FINAL NOTE: Because we need to index till 4, so if you have less than 4 figures,
/// like 9999 yoctonear, it'll fail. If you have 10000, it'll pass (provided)
/// it's within f64 range and ignore floating point errors. 
/// 
/// Example: 
/// ```
/// // Approx 3.193 NEAR
/// let amount: u128 = 3_193_264_587_249_763_651_824_729;
/// 
/// assert_eq!(
///   near_helper::yoctonear_to_near(amount),
///   3.19326f64
/// );
/// ```
/// 
/// Then Example: 
/// ```
/// // Approx 0.0214 NEAR
/// let amount: u128 = 21_409_258_000_000_000_000_000;
/// 
/// assert_eq!(
///   near_helper::yoctonear_to_near(amount),
///   0.021409f64
/// );
/// ```
pub fn yoctonear_to_near(amount: u128) -> f64 {
    let decimals = 5;

    let amount_str = amount.to_string();
    let amount_bytes = amount_str.as_bytes();

    let amount_len = amount_bytes.len();

    let mut num: String = "".to_owned();
    if amount_len <= 24 {  // below 1 NEAR, which has len = 25
      num.push_str("0.");

      let append_zeros = 24 - amount_len;
      for _ in 0..append_zeros {
        num.push_str("0")
      }

      for i in 0..decimals {
        num.push(amount_bytes[i] as char)
      }

    } else {  // above 1 NEAR
      let left = amount_len - 24;
      
      for i in 0..left {
        num.push(amount_bytes[i] as char)
      }
      
      num.push_str(".");

      for i in left..left+decimals {
        num.push(amount_bytes[i] as char)
      }
    }


    num.parse().unwrap()
}


/// NEAR to yoctonear conversion. 
/// 
/// Will fail if somehow you insert a value less than 1 yoctoNEAR. 
pub fn near_to_yoctonear(amount: f64) -> u128 {
    let amount_str = amount.to_string();
    let amount_bytes = amount_str.as_bytes();
    
    let amount_len = amount_bytes.len();

    let mut num: String = "".to_owned();

    if (amount_bytes[0] as char == '0') && (amount_bytes[1] as char == '.') {
      let mut count = 0;

      while count < amount_len {
        if amount_bytes[count + 2] as char == '0' {
          count += 1;
        } else {
          break
        }
      }

      for i in count+2..amount_len {
        num.push(amount_bytes[i] as char)
      }

      let actual_length = 24 - count;

      for _ in num.len()..actual_length {
        num.push_str("0")
      }

    } else {
      let mut count = 0;

      while count < amount_len {
        if amount_bytes[count] as char != '.' {
          count += 1
        } else {
          break
        }
      }

      // left of decimal
      for i in 0..count {
        num.push(amount_bytes[i] as char)
      }

      // right of decimal
      let remnant_num = amount_len - count;
      let zeros_to_add = 25 - remnant_num;

      for i in count+1..count+remnant_num {
        num.push(amount_bytes[i] as char)
      }

      for _ in 0..zeros_to_add {
        num.push_str("0")
      }
    }

    num.parse().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

    #[test]
    fn test_yoctonear_to_near_conversion_correct_below_decimals() {
      assert_eq!(yoctonear_to_near(ONE_NEAR / 500), 0.002);
    }

    #[test]
    fn test_yoctonear_conversion_correct_above_decimals() {
      assert_eq!(yoctonear_to_near(ONE_NEAR * 12), 12.0);
    }


    #[test]
    fn test_yoctonear_conversion_too_small() {
      yoctonear_to_near(10000);
    }


    #[test]
    fn test_near_to_yoctonear_correct_less_than_one_near() {
      assert_eq!(near_to_yoctonear(0.0021489), 2_148_900_000_000_000_000_000);
    }


    #[test]
    fn test_near_to_yoctonear_correct_more_than_one_near() {
      assert_eq!(near_to_yoctonear(127.864), 127_864_000_000_000_000_000_000_000);
    }
}