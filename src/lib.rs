#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]

mod jab;
mod utilt;
mod dtilt;
mod dash;
mod dsmash;
mod nair;
mod fair;
mod bair;
mod uair;
mod dair;
mod counter;
mod sideb;
mod run;

#[skyline::main(name = "smashline_test")]
pub fn main() {
	jab::install();
	utilt::install();
	dtilt::install();
	dash::install();
	dsmash::install();
	nair::install();
	fair::install();
	bair::install();
	uair::install();
	dair::install();
	counter::install();
	sideb::install();
	run::install();
}