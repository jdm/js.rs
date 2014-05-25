#![allow(missing_doc)]

use ast::{ConstExpr, CNull, Expr};
use js::value::{Value, ValueData};
use libjit::{Context, Function, Type, Types, CDECL};

use std::gc::Gc;

fn with_builder<'a, R>(context: &'a Context, cb: || -> R) -> R {
    context.build_start();
    let rv = cb();
    context.build_end();
    rv
}

pub enum JITCompileError {
    Unimplemented,
}

fn create_value_from_data(arg: ValueData) -> Value {
    Gc::new(arg)
}

pub fn compile_expr(context: &Context, expr: &Expr) -> Result<Box<Function>, JITCompileError> {
    with_builder(context, || {
        let enum_of_bare_enums_t = Type::create_union(&[&*Types::get_int()]);
        let binop_t = Type::create_union(&[&*enum_of_bare_enums_t]);
        let binop_expr_t = Type::create_struct(&[&*binop_t, &*Types::get_void_ptr(),
                                                 &*Types::get_void_ptr()]);
        let const_t = Type::create_union(&[&*Types::get_float64()]);
        let constexpr_t = Type::create_struct(&[&*const_t]);
        let exprdef_t = Type::create_union(&[&*binop_expr_t, &*constexpr_t]);
        let expr_t = Type::create_struct(&[&*exprdef_t]);
        let expr_ptr_t = Type::create_pointer(&*expr_t);

        let valuedata_t = Type::create_union(&[&*Types::get_int(), &*Types::get_float64()]); //XXXjdm only numeric for now
        let valuedata_ptr_t = Type::create_pointer(valuedata_t);
        let value_t = Type::create_struct(&[&*valuedata_ptr_t]); // Gc<ValueData>

        let value_ptr_t = Type::create_pointer(&*value_t);

        // fn(&Expr, &mut Gc<ValueData>) -> ()
        let sig = Type::create_signature(CDECL, Types::get_void(), &[&*expr_ptr_t,
                                                                     &*value_ptr_t]);
        let func = context.create_function(sig);

        let result = func.get_param(1);

        match expr.def {
            ConstExpr(CNull) => {
                // fn(ValueData) -> Gc<ValueData>
                let create_val_sig = Type::create_signature(CDECL, value_t, &[&*valuedata_t]);
                let vnull = func.insn_alloca(func.constant_int32(valuedata_t.get_size() as i32));
                func.insn_store_relative(&*vnull, 0, func.constant_int32(0));
                let t = func.create_value(valuedata_t);
                func.insn_store(t, vnull);
                let res = func.insn_call_native1("create_value_from_data",
                                                 create_value_from_data,
                                                 create_val_sig,
                                                 &[&*t]);
                func.insn_store_relative(&result, 0, &*res);
                                                
            },
            _ => return Err(Unimplemented),
        }

        //func.dump("foo");
        func.compile();
        //func.dump("foo");
        Ok(func)
    })
}