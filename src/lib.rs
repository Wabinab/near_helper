//! # NEAR-SDK-RS 4.0 pre-release
//! 
//! Some helper for NEAR 4.0 after upgrading from previous
//! versions. 

// use near_sdk::{env, require, utils};
use std::collections::HashMap;

pub mod timestamp;

pub use crate::timestamp::*;

/// Checks for successful promise. 
#[deprecated(
  since="0.2.0", 
  note="please use near_sdk::utils::is_promise_success"
)]
pub fn is_promise_success(){
    // utils::is_promise_success()
    // false
}


/// The equivalent of .expect() but a lightweight version
/// to reduce compiled-wasm size. 
#[deprecated(
  since="0.3.0",
  note="just do unwrap_or_else(|| env::panic_str(...)) manually."
)]
pub fn expect_lightweight<T>(
  _option: Option<T>,
  _message: &str,
) {
    // option.unwrap_or_else(|| env::panic_str(message))
}


/// Assert predecessor is current, very frequently used
/// assertion. 
/// Similar to near_sdk::utils::assert_self except you can
/// enter a custom message for claribility. 
#[deprecated(
  since="0.3.0",
  note="use near_sdk::utils::assert_self instead."
)]
pub fn assert_predecessor_is_current(_message: &str) {
    // require!(
    //   env::predecessor_account_id() == env::current_account_id(),
    //   message
    // )
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
///   "3.19326".to_owned()
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
///   "0.021409".to_owned()
/// );
/// ```
pub fn yoctonear_to_near(amount: u128) -> String {
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
        if i < amount_len {
          num.push(amount_bytes[i] as char)
        }
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

    num = num.trim_end_matches('0').to_owned();
    num = num.trim_end_matches('.').to_owned();

    num
}


/// NEAR to yoctonear conversion. 
/// 
/// Example:
/// ```
/// let amount = "3.214".to_owned();
/// 
/// assert_eq!(
///   near_helper::near_to_yoctonear(amount),
///   3_214_000_000_000_000_000_000_000u128
/// );
/// ```
/// 
/// 
/// Will fail if somehow you insert a value less than 1 yoctoNEAR. 
pub fn near_to_yoctonear(amount: String) -> u128 {
    if amount == "0" { return 0; }
    // let amount_str = amount.to_string();
    let amount_str = amount;
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



/// Return the scientific notation of a given value, 
/// based on the digit you want to keep. This offers 
/// you to calculate using the number of kept digits,
/// hence simplifying calculations. 
/// 
/// This is useful if you want to keep calculation simple. 
/// (UNDER TESTING CURRENTLY FOR PROVE.) Instead of 
/// calculating with u128, use u32. 
/// 
/// CAVEAT: You lose accuracy depending on how much
/// digit you choose to keep. All digits that you
/// don't keep will be replaced by "0". Example: 
/// if you keep 3 digits, and your value is 1.234x10^25, 
/// it would return (123, 23) as it now moves to 123x10^23
/// equivalent to 1.23x10^25. 
/// 
/// ```
/// let amount: u128 = 2_913_464_000_000_000_000;
/// 
/// assert_eq!(
///   near_helper::as_scientific_notation(amount, 3),
///   (291u32, 16u8)
/// );
/// ```
pub fn as_scientific_notation(
  amount: u128,
  keep_digit: usize
) -> (u32, u8) {
    let amount: String = amount.to_string();
    let amount_bytes = amount.as_bytes();
    let amount_len = amount_bytes.len();

    let mut return_digit: String = "".to_owned();
    for i in 0..keep_digit {
      return_digit.push(amount_bytes[i] as char);
    }

    let power_of: u8 = amount_len as u8 - keep_digit as u8;
    (return_digit.parse().unwrap(), power_of)
}


#[cfg(test)]
mod tests {
    use super::*;
    const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

    #[test]
    fn test_yoctonear_to_near_conversion_correct_below_decimals() {
      assert_eq!(yoctonear_to_near(ONE_NEAR / 500), "0.002".to_owned());
    }

    #[test]
    fn test_yoctonear_conversion_correct_above_decimals() {
      assert_eq!(yoctonear_to_near(ONE_NEAR * 12), "12".to_owned());
    }


    // #[test]
    // fn test_yoctonear_conversion_too_small() {
    //   // assert_eq!(yoctonear_to_near(10000), "0".to_owned());
    // }

    #[test]
    fn test_yoctonear_zero_conversion_success() {
      assert_eq!(yoctonear_to_near(0), "0".to_owned());
    }

    #[test]
    fn test_near_zero_conversion_success() {
      assert_eq!(near_to_yoctonear("0".to_owned()), 0);
    }


    #[test]
    fn test_near_to_yoctonear_correct_less_than_one_near() {
      assert_eq!(near_to_yoctonear("0.0021489".to_owned()), 2_148_900_000_000_000_000_000);
    }


    #[test]
    fn test_near_to_yoctonear_correct_more_than_one_near() {
      assert_eq!(near_to_yoctonear("127.864".to_owned()), 127_864_000_000_000_000_000_000_000);
    }

    // =======================================
    // Chrono
    fn datetime_comparer(datetime: HashMap<&'static str, String>, 
      year: &str, month: &str, day: &str, hour: &str, mins: &str, 
      secs: &str
    ) {
      println!("{:?}", datetime);
      assert_eq!(datetime.get("year").unwrap().clone(), year.to_owned(), "year wrong.");
      assert_eq!(datetime.get("month").unwrap().clone(), month.to_owned(), "month wrong.");
      assert_eq!(datetime.get("day").unwrap().clone(), day.to_owned(), "day wrong.");
      assert_eq!(datetime.get("hour").unwrap().clone(), hour.to_owned(), "hour wrong.");
      assert_eq!(datetime.get("min").unwrap().clone(), mins.to_owned(), "min wrong.");
      assert_eq!(datetime.get("sec").unwrap().clone(), secs.to_owned(), "sec wrong.");
    }

    #[test]
    fn test_datetime_1() {
      datetime_comparer(
        timestamp_millis_to_datetime(388453887000),
        "1982", "4", "23", "23", "51", "27"
      );
    }

    #[test]
    fn test_datetime_2() {
      datetime_comparer(
        timestamp_millis_to_datetime(0),
        "1970", "1", "1", "0", "0", "0"
      );
    }

    #[test]
    fn test_datetime_3() {
      datetime_comparer(
        timestamp_millis_to_datetime(1704067202000),
        "2024", "1", "1", "0", "0", "2"
      );
    }

    #[test]
    fn test_datetime_4_endtime() {
      datetime_comparer(
        timestamp_millis_to_datetime(1388534399000),
        "2013", "12", "31", "23", "59", "59"
      );
    }

    #[test]
    fn test_datetime_5_leapfeb() {
      datetime_comparer(
        timestamp_millis_to_datetime(1709208000000),
        "2024", "2", "29", "12", "0", "0"
      );
    }

    #[test]
    fn test_datetime_6() {
      datetime_comparer(
        timestamp_millis_to_datetime(1435649522000),
        "2015", "6", "30", "7", "32", "2"
      );
    }

    #[test]
    fn test_datetime_7() {
      datetime_comparer(
        timestamp_millis_to_datetime(1409265002000),
        "2014", "8", "28", "22", "30", "2"
      );
    }

}