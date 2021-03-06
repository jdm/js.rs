use js::value::{Value, ValueData, ResultValue, VUndefined, to_value, from_value};
use std::gc::Gc;
use std::iter::FromIterator;
use std::io::stdio::stderr;
use time::{now, strftime};
/// Print a javascript value to the standard output stream
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let args : Vec<StrBuf> = FromIterator::from_iter(args.iter().map(|x|from_value::<StrBuf>(*x).unwrap()));
	println!("{}: {}", strftime("%X", &now()), args.connect(" "));
	Ok(Gc::new(VUndefined))
}
/// Print a javascript value to the standard error stream
pub fn error(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let args : Vec<StrBuf> = FromIterator::from_iter(args.iter().map(|x|from_value::<StrBuf>(*x).unwrap()));
	match writeln!(&mut stderr().unwrap(), "{}: {}", strftime("%X", &now()), args.connect(" ")) {
		Ok(_) => Ok(Gc::new(VUndefined)),
		Err(io_error) => Err(to_value(io_error.to_str().into_strbuf()))
	}
}
/// Create a new `console` object
pub fn _create(global : Value) -> Value {
	let console = ValueData::new_obj(Some(global));
	let console_ptr = console.borrow();
	console_ptr.set_field_slice("log", to_value(log));
	console_ptr.set_field_slice("error", to_value(error));
	console_ptr.set_field_slice("exception", to_value(error));
	console
}
/// Initialise the global object with the `console` object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("console", _create(global));
}