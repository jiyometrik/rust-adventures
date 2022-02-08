// list of datatypes for ref
// https://doc.rust-lang.org/book/ch03-02-data-types.html

use std::i128;

fn main() {
	// 1. TYPE CASTING
	// if using .parse() to convert string to integer, must include the type
	//         ***
	let guess: u32 = "42".parse().expect("Not a number!");

	// 2. DATATYPES
	// int (i8/i16/i32/i64/i128 [signed int] / u8/u16/u32/u64/u128 [unsigned int])
	let x = 2; // i32 (default)
	let y: i128 = 173521635169351153; // big number, i128

	// float (f32/f64)
	let a = 2.0; // f64 (default)
	let b: f32 = 3.0; // f32

	// bool
	let t = true;
	let f: bool = false; // explicit

	// char (single quotes only, strings are in double quotes, no explicit type annotation)
	let c = 'c';
	let z = '#';
	let heart_eyed_cat = '😻';

	// tuples (cannot change size)
	let tup: (i32, f64, u8) = (500, 6.4, 1); // use parens to surround a tuple
	let (x, y, z) = tup; // destructuring
										 // println!("tuple: {}", tup); // cannot directly print a tuple out
	println!("second element of tuple: {}", y); // destructuring
	println!("third element of tuple: {}", tup.2); // accessing using index notation

	// arrays (cannot change size)
	let arr: [i64; 5] = [100, 200, 300, 400, 500]; // define all values
	let arr2 = [3; 5]; // specify initial value of all elements in array
	println!("first element of arr: {}", arr[0]);

	// 3. OPERATIONS (https://doc.rust-lang.org/book/appendix-02-operators.html)
	let sum = 5 + 10;
	let diff = 95.5 - 4.3;
	let prod: f64 = 4.0 * 30.0;
	let quotient_raw = 56.7 / 32.2; // quotient (float) works with float operands only
	let quotient_floored = 2 / 3; // floored quotient (integer) works with integer operands only
	let modu = 43 % 5;
}
