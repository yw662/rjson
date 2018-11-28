# rjson : a minimal json parser for rust
* Contain a single standalone `lib.rs` that do all the jobs.
* Impl the traits with your own structs before `parse`.
* A simple Impl can be, `enum` for `Value`, the same `enum` for `Null`, `Vec` for `Array` and `BTreeMap` for `Object`.
* It requires only `core`, nothing else, including `std`.

## Reminder
* We allow `,` after the last item/member of Array/Object.
* We treat unescaped line breaks as normal char, and ignore escaped line breaks.
* We do not support surrogate unicode char.
* We use `f64` for all numbers, but you can use others. Remind: `f64` means `i52`.
* We take `&[char]`, not `&[u8]`, and not `&str`.
* No `stringify` or `encode`, because they should not be a part of the traits.
* Instead of returning `None`, we simply ignore chars after the data.
* The position where data ends is returned through `index`. You can compare it with `len() - 1`.
* This value is also useful when `Option::None` returned, by indicating where the syntax error occurs.
* `parse` may return all possible values, not only `Array` and `Object`.
