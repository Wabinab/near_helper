//! Because we can't use the chrono crate (even with serde and wasmbind 
//! it doesn't seems to compile properly), here we are. Because we have
//! `near_sdk::env::block_timestamp_ms()`, here tries to convert that.



use crate::*;

/// Convert timestamp millis to datetime.
/// The datetime format will be in HashMap, as if we implement
/// our own struct, it may have trouble deserializing. 
/// (As usual, we don't want to depend on near_sdk so no
/// BorshSerialize and BorshDeserialize available.)
/// 
/// If you want something easier to use, you should do your struct
/// and pass it through to convert to your local format, 
/// most probably one that supports serde and borsh. 
/// 
/// (Unlike javascript, we use "day" instead of "date", and have
/// no "day" (no monday tuesday, etc.) support.)
/// The hashmap has "year", "month", "day", "hour", "min", "sec"
/// We reverse engineer this from `chrono`, with extra restrictions.
/// That is, anything before timestamp 0 isn't supported. 
/// 
/// Currently, no support for conversion to specific timezone, only
/// UTC. 
/// 
/// Example: 
/// ```
/// let millis: u64 = 1715603315000;  // 2024-05-13 12:28:35 UTC. 
/// let datetime = near_helper::timestamp_millis_to_datetime(millis);
/// 
/// assert_eq!(datetime.get("year").unwrap().clone(), "2024".to_owned());
/// assert_eq!(datetime.get("month").unwrap().clone(), "5".to_owned());
/// assert_eq!(datetime.get("day").unwrap().clone(), "13".to_owned());
/// assert_eq!(datetime.get("hour").unwrap().clone(), "12".to_owned());
/// assert_eq!(datetime.get("min").unwrap().clone(), "28".to_owned());
/// assert_eq!(datetime.get("sec").unwrap().clone(), "35".to_owned());
/// ```
pub fn timestamp_millis_to_datetime(millis: u64) -> HashMap<&'static str, String> {
  let secs = millis.div_euclid(1000);
  // let nsecs = millis.rem_euclid(1000) as u32 * 1_000_000;
  return from_timestamp(secs);
}


// =====================================
// Private functions
fn from_timestamp(secs: u64) -> HashMap<&'static str, String> {
  let days = secs.div_euclid(86_400);
  let secs = secs.rem_euclid(86_400);
  let mut date = from_num_days_unix(days);
  let time = from_midnight(secs);
  date.extend(time);
  return date;
}

fn from_num_days_unix(days: u64) -> HashMap<&'static str, String> {
  let mut ret_date = HashMap::new();

  let mut diff_year = days.div_euclid(365);
  let floor_year = (365.25 * diff_year as f64).floor() as u64;
  if floor_year > days { diff_year -= 1; }
  let year = 1970 + diff_year;
  let extra_days = days + 1 - (365.25 * diff_year as f64).round() as u64;
  let leap = year % 4 == 0;
  let month = get_month(extra_days, leap);
  let date = get_date(extra_days, month.parse().unwrap(), leap);

  ret_date.insert("year", year.to_string());
  ret_date.insert("month", month);
  ret_date.insert("day", date);

  return ret_date;
}

fn get_month(days: u64, leap: bool) -> String {
  let months = _months(leap);
//   println!("Days: {}", days);
  for i in 0..months.len() {
    if days <= months[i] {
      return (i + 1).to_string();
    }
  }
  return "".to_owned();
}


fn get_date(days: u64, month: usize, leap: bool) -> String {
  let mut days = days;
  if leap { days += 1; }  // correction to leap. Not sure why logic fails without this.
  let months = _months(leap);
  let breakpoint = match month.checked_sub(2) {
    Some(_) => months[month-2],   // -1 for zero-based, -1 for the prev month. 
    None => 0
  };
  println!("Days: {}\tBreakpoint: {}", days, breakpoint);
  let date = days.checked_sub(breakpoint).unwrap();
  return date.to_string();
}


fn _months(leap: bool) -> Vec<u64> {
  let months: Vec<u64> = match leap {
    true => vec![31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
    false => vec![31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365]
  };
  months
}

fn from_midnight(tot_secs: u64) -> HashMap<&'static str, String> {
  let mut ret_time = HashMap::new();

  let hour = tot_secs / 3600;
  let mins = (tot_secs - (hour * 3600)) / 60;
  let secs = tot_secs - (hour * 3600 + mins * 60);

  ret_time.insert("hour", hour.to_string());
  ret_time.insert("min", mins.to_string());
  ret_time.insert("sec", secs.to_string());
  return ret_time;
}