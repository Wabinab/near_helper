# NEAR Helper

### Edit: 11 Jan 2024
Since dependence on near-sdk makes this library really difficult to use, we're gonna remove all functions that depends on that library, and remove that library. That leaves only conversion of near to yoctonear useful. 

Why this library instead of `NearToken`? Because near token has a weird `to_near()` functions that returns `u128` instead of `f64` or `String`, so if you parse 1.25N, it'll return as 1N instead, losing all the back. You could use `to_millinear()`; but of course, if you need scale towards the micro (more than 3 decimal place), it's the same problem again. 

Also, we changed return values for those f64 to String; so they can be used with [`string_calc`](https://crates.io/crates/string_calc) Decimal type (which `Decimal` is `String` in disguise). 

---

Some NEAR helpers. Some are deprecated as they got integrated into near sdk. Others remains (although inside near sdk, but one prefer one's version for some reason). Others are helpers for conversions. 

If you have a better way of doing stuffs, or some suggestions for test that you think should pass but might fail, open up a request in github here: https://github.com/Wabinab/near_helper/issues

