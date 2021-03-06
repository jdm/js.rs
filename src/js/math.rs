use js::value::{Value, ValueData, ResultValue, to_value, from_value};
use rand::random;
use std::f64;

/// Get the absolute value of a number
pub fn abs(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().abs()
	} else {
		f64::NAN
	}))
}
/// Get the arccos of a number
pub fn acos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().acos()
	} else {
		f64::NAN
	}))
}
/// Get the arcsine of a number
pub fn asin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().asin()
	} else {
		f64::NAN
	}))
}
/// Get the arctangent of a number
pub fn atan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().atan()
	} else {
		f64::NAN
	}))
}
/// Get the arctangent of a numbers
pub fn atan2(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().atan2(args.get(1).borrow().to_num())
	} else {
		f64::NAN
	}))
}
/// Get the cubic root of a number
pub fn cbrt(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().cbrt()
	} else {
		f64::NAN
	}))
}
/// Get lowest integer above a number
pub fn ceil(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().ceil()
	} else {
		f64::NAN
	}))
}
/// Get the cosine of a number
pub fn cos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().cos()
	} else {
		f64::NAN
	}))
}
/// Get the power to raise the natural logarithm to get the number
pub fn exp(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().exp()
	} else {
		f64::NAN
	}))
}
/// Get the highest integer below a number
pub fn floor(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().floor()
	} else {
		f64::NAN
	}))
}
/// Get the natural logarithm of a number
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().log(f64::consts::E)
	} else {
		f64::NAN
	}))
}
/// Get the maximum of several numbers
pub fn max(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let mut max = f64::NEG_INFINITY;
	for arg in args.iter() {
		let num = arg.borrow().to_num();
		max = max.max(num);
	}
	Ok(to_value(max))
}
/// Get the minimum of several numbers
pub fn min(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let mut max = f64::INFINITY;
	for arg in args.iter() {
		let num = arg.borrow().to_num();
		max = max.min(num);
	}
	Ok(to_value(max))
}
/// Raise a number to a power
pub fn pow(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 2 {
		let num : f64 = from_value(*args.get(0)).unwrap();
		let power : f64 = from_value(*args.get(1)).unwrap();
		num.powf(power)
	} else {
		f64::NAN
	}))
}
/// Generate a random floating-point number between 0 and 1
pub fn _random(_:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(to_value(random::<f64>()))
}
/// Round a number to the nearest integer
pub fn round(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().round()
	} else {
		f64::NAN
	}))
}
/// Get the sine of a number
pub fn sin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().sin()
	} else {
		f64::NAN
	}))
}
/// Get the square root of a number
pub fn sqrt(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().sqrt()
	} else {
		f64::NAN
	}))
}
/// Get the tangent of a number
pub fn tan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(to_value(if args.len() >= 1 {
		from_value::<f64>(*args.get(0)).unwrap().tan()
	} else {
		f64::NAN
	}))
}
/// Create a new `Math` object
pub fn _create(global : Value) -> Value {
	let math = ValueData::new_obj(Some(global));
	let math_ptr = math.borrow();
	math_ptr.set_field_slice("E", to_value(f64::consts::E));
	math_ptr.set_field_slice("LN2", to_value(f64::consts::LN_2));
	math_ptr.set_field_slice("LN10", to_value(f64::consts::LN_10));
	math_ptr.set_field_slice("LOG2E", to_value(f64::consts::LOG2_E));
	math_ptr.set_field_slice("LOG10E", to_value(f64::consts::LOG10_E));
	math_ptr.set_field_slice("SQRT1_2", to_value(0.5f64.sqrt()));
	math_ptr.set_field_slice("SQRT2", to_value(f64::consts::SQRT2));
	math_ptr.set_field_slice("PI", to_value(f64::consts::PI));
	math_ptr.set_field_slice("abs", to_value(abs));
	math_ptr.set_field_slice("acos", to_value(acos));
	math_ptr.set_field_slice("asin", to_value(asin));
	math_ptr.set_field_slice("atan", to_value(atan));
	math_ptr.set_field_slice("atan2", to_value(atan2));
	math_ptr.set_field_slice("cbrt", to_value(cbrt));
	math_ptr.set_field_slice("ceil", to_value(ceil));
	math_ptr.set_field_slice("cos", to_value(cos));
	math_ptr.set_field_slice("exp", to_value(exp));
	math_ptr.set_field_slice("floor", to_value(floor));
	math_ptr.set_field_slice("log", to_value(log));
	math_ptr.set_field_slice("max", to_value(max));
	math_ptr.set_field_slice("min", to_value(min));
	math_ptr.set_field_slice("pow", to_value(pow));
	math_ptr.set_field_slice("random", to_value(_random));
	math_ptr.set_field_slice("round", to_value(round));
	math_ptr.set_field_slice("sin", to_value(sin));
	math_ptr.set_field_slice("sqrt", to_value(sqrt));
	math_ptr.set_field_slice("tan", to_value(tan));
	math
}
/// Initialise the `Math` object on the global object
pub fn init(global:Value) {
	global.borrow().set_field_slice("Math", _create(global));
}