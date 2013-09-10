use std::{os, float, uint};
fn main() {
	let args: ~[~str] = os::args();
	let mut total: float = 0.0;
	let mut count = 0;
	for uint::range(1,args.len()) |i| {
		match float::from_str(args[i]) {
			Some(val) => {total += val; count += 1;}
			None => {println(fmt!("Bad input: %s", args[i]));}
		}
	}
	println(fmt!("Average = %f", total/(count as float)));
}
