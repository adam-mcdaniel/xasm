extern crate xmachine;
use xmachine::{Machine, Value};


pub fn xasm_list(xasm: &mut Machine) {
    let mut result = vec![];
    let val = xasm.get_arg();
    let count = i32::from(val.clone());
    
    for _ in 0..count {
        result.push(xasm.pop());
    }

    xasm.return_value(Value::List(result));
}

pub fn xasm_range(xasm: &mut Machine) {
    let mut result = vec![];
    let lower = i32::from(xasm.get_arg());
    let upper = i32::from(xasm.get_arg());
    
    for i in lower..upper {
        result.push(Value::number(i));
    }
    xasm.return_value(Value::List(result));
}

pub fn xasm_filter(xasm: &mut Machine) {
    let mut result = vec![];
    let list = xasm.get_arg();
    let function = xasm.pop();

    if let Value::List(mut l) = list {
        for item in l {
            xasm.push(item.clone());
            xasm.push(function.clone());
            xasm.call();
            if bool::from(xasm.get_arg()) {
                result.push(item);
            }
        }
    }

    xasm.return_value(Value::List(result));
}

pub fn xasm_reverse(xasm: &mut Machine) {
    let mut list = xasm.get_arg();

    if let Value::List(mut l) = list {
        l.reverse();
        list = Value::List(l);
    }

    xasm.return_value(list);
}

pub fn xasm_reduce(xasm: &mut Machine) {
    let list = xasm.get_arg();
    let function = xasm.pop();
    let mut accumulator = xasm.pop();

    if let Value::List(mut l) = list {
        for item in l {
            xasm.push(accumulator.clone());
            xasm.push(item.clone());
            xasm.push(function.clone());
            xasm.call();
            accumulator = xasm.pop();
        }
    }

    xasm.push(accumulator);
}

pub fn xasm_map(xasm: &mut Machine) {
    let mut result = vec![];

    let list = xasm.get_arg();
    let function = xasm.pop();

    if let Value::List(mut l) = list {
        for item in l {
            xasm.push(item.clone());
            xasm.push(function.clone());
            xasm.call();
            result.push(xasm.pop());
        }
    }

    xasm.return_value(Value::List(result));
}

pub fn xasm_push(xasm: &mut Machine) {
    let list_value = xasm.pop();
    if let Value::List(mut l) = (*list_value).clone() {
        l.push(xasm.pop());
        xasm.return_value(Value::List(l));
    }
}

pub fn xasm_pop(xasm: &mut Machine) {
    let value = xasm.pop();
    if let Value::List(mut l) = (*value).clone() {
        let last_value = l[l.len() - 1].clone();
        l.pop();
        xasm.push(last_value.copy());
        xasm.return_value(Value::List(l));
    }
}

pub fn xasm_len(xasm: &mut Machine) {
    let value = xasm.pop();
    xasm.push(
        Value::number(
            if let Value::List(l) = (*value).clone() {
                l.len() as f64
            } else if let Value::String(s) = (*value).clone() {
                s.len() as f64
            } else {
                0.0
            }
        )
    );
}

pub fn xasm_format(xasm: &mut Machine) {
    let s = Value::string(format!("{}", xasm.pop()));
    xasm.push(s);
}

pub fn xasm_debug(xasm: &mut Machine) {
    let machine = Value::string(format!("{}", xasm));
    xasm.push(machine);
}

pub fn xasm_new(xasm: &mut Machine) {
    let class = xasm.pop();
    class.call(xasm);
    xasm.push(Value::string("new"));
    xasm.method_call();
}

pub fn xasm_add(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first + second);
}

pub fn xasm_sub(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first - second);
}

pub fn xasm_mul(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first * second);
}

pub fn xasm_div(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first / second);
}

pub fn xasm_rem(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(first % second);
}

pub fn xasm_not(xasm: &mut Machine) {
    let value = xasm.get_arg();
    xasm.return_value(!value);
}

pub fn xasm_eq(xasm: &mut Machine) {
    let first = xasm.get_arg();
    let second = xasm.get_arg();
    xasm.return_value(Value::from(first == second));
}