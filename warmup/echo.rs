use std::{os, uint};

fn main() {
	let args: ~[~str] = os::args();
	for uint::range(1,args.len()) |i| {
		print(args[i]);
		print(" ");
	}
	println("");
}
