use ast::{Expr, ConstExpr, BlockExpr, TypeOfExpr, LocalExpr, GetConstFieldExpr, GetFieldExpr, CallExpr, WhileLoopExpr, IfExpr, SwitchExpr, ObjectDeclExpr, ArrayDeclExpr, FunctionDeclExpr, ArrowFunctionDeclExpr, NumOpExpr, BitOpExpr, LogOpExpr, CompOpExpr, ConstructExpr, ReturnExpr, ThrowExpr, AssignExpr};
use ast::{CNum, CInt, CString, CBool, CRegExp, CNull, CUndefined};
use ast::{OpSub, OpAdd, OpMul, OpDiv, OpMod};
use ast::{BitAnd, BitOr, BitXor, BitShl, BitShr};
use ast::{LogAnd, LogOr};
use ast::{CompEqual, CompNotEqual, CompStrictEqual, CompStrictNotEqual, CompGreaterThan, CompGreaterThanOrEqual, CompLessThan, CompLessThanOrEqual};
use js::value::{Value, ValueData, VNull, VUndefined, VString, VNumber, VInteger, VObject, VBoolean, VFunction, ResultValue, to_value, from_value};
use js::object::{INSTANCE_PROTOTYPE, PROTOTYPE, ObjectData};
use js::function::{RegularFunc, RegularFunction};
use js::{console, math, object, array, function, json, number, error, uri, string};
use collections::treemap::TreeMap;
use llvm::{ContextRef, ValueRef};
use llvm::llvm::{LLVMContextCreate};
use std::vec::Vec;
use std::gc::Gc;
use std::cell::RefCell;
use std::str::MaybeOwned;
/// JIT
pub struct Executor {
	context: ContextRef,
	global: Value,
	scopes: Vec<ObjectData>
}
impl Executor {
	fn new() -> Executor {
		let global = ValueData::new_obj(None);
		object::init(global);
		console::init(global);
		math::init(global);
		array::init(global);
		function::init(global);
		json::init(global);
		number::init(global);
		error::init(global);
		number::init(global);
		string::init(global);
		uri::init(global);
		Executor {
			global: global,
			scopes: Vec::new(),
			context: LLVMContextCreate()
		}
	}
	fn set_global(&mut self, name:MaybeOwned, val:Value) -> Value {
		self.global.borrow().set_field(name, val)
	}
	fn get_global(&self, name:MaybeOwned) -> Value {
		self.global.borrow().get_field(name)
	}
	fn make_scope(&mut self) -> ObjectData {
		let value = TreeMap::new();
		self.scopes.push(value);
		value
	}
	fn destroy_scope(&mut self) -> () {
		self.scopes.pop();
	}
	fn compile(&self, e:Expr) -> ValueRef {
		
	}
}