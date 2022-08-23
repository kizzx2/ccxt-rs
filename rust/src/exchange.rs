#![allow(clippy::absurd_extreme_comparisons)]
#![allow(clippy::if_same_then_else)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_assignments)]
#![allow(unused_comparisons)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::cmp::{max, Ordering};
use serde::{Serialize, Deserialize};
use std::ops::{Add, Div, Mul, Not, Sub};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};
use num_traits::sign::Signed;
use num_integer::Integer;
use serde_json::json;

const PRECISE_BASE: usize = 10;

// rounding mode
const TRUNCATE: usize = 0;
const ROUND: usize = 1;
const ROUND_UP: usize = 2;
const ROUND_DOWN: usize = 3;

// digits counting mode
const DECIMAL_PLACES: usize = 2;
const SIGNIFICANT_DIGITS: usize = 3;
const TICK_SIZE: usize = 4;

// padding mode
const NO_PADDING: usize = 5;
const PAD_WITH_ZERO: usize = 6;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Value {
    Json(serde_json::Value),
    Precise(Precise),
    Undefined,
}

pub trait ValueTrait {
    fn is_undefined(&self) -> bool;
    fn is_truthy(&self) -> bool;
    fn or_default(&self, default: Value) -> Value;
    fn is_number(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_object(&self) -> bool;
    fn is_falsy(&self) -> bool;
    fn to_upper_case(&self) -> Value;
    fn unwrap_str(&self) -> &str;
    fn unwrap_bool(&self) -> bool;
    fn unwrap_precise(&self) -> &Precise;
    fn unwrap_json(&self) -> &serde_json::Value;
    fn unwrap_json_mut(&mut self) -> &mut serde_json::Value;
    fn unwrap_precise_mut(&mut self) -> &mut Precise;
    fn len(&self) -> usize;
    fn get(&self, key: Value) -> Value;
    fn set(&mut self, key: Value, value: Value);
    fn push(&mut self, value: Value);
    fn split(&self, separator: Value) -> Value;
    fn contains_key(&self, key: Value) -> bool;
    fn keys(&self) -> Vec<Value>;
    fn values(&self) -> Vec<Value>;
    fn to_array(&self, x: Value) -> Value;
    fn index_of(&self, x: Value) -> Value;
    fn typeof_(&self) -> Value;
}

impl Value {
    fn new_array() -> Self {
        Value::Json(serde_json::Value::Array(vec![]))
    }

    fn new_object() -> Self {
        Value::Json(serde_json::Value::Object(serde_json::Map::new()))
    }
}

impl ValueTrait for Value {
    fn is_undefined(&self) -> bool {
        match self {
            Value::Undefined => true,
            _ => false
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Json(json) => {
                match json {
                    serde_json::Value::Bool(b) => *b,
                    serde_json::Value::Null => false,
                    serde_json::Value::Number(n) => {
                        if n.is_f64() {
                            n.as_f64().unwrap() != 0.0
                        } else if n.is_u64() {
                            n.as_i64().unwrap() != 0
                        } else if n.is_i64() {
                            n.as_i64().unwrap() != 0
                        } else {
                            unreachable!()
                        }
                    }
                    serde_json::Value::String(s) => !s.is_empty(),
                    serde_json::Value::Array(a) => !a.is_empty(),
                    serde_json::Value::Object(o) => !o.is_empty(),
                    // _ => false,
                }
            }
            Value::Precise(precise) => !precise.is_zero(),
            _ => false,
        }
    }

    fn or_default(&self, default: Value) -> Value {
        match self {
            Value::Undefined => default,
            _ => self.clone()
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Value::Json(j) => j.is_number(),
            _ => false
        }
    }

    fn is_string(&self) -> bool {
        match self {
            Value::Json(j) => j.is_string(),
            _ => false
        }
    }

    fn is_object(&self) -> bool {
        match self {
            Value::Json(j) => j.is_object(),
            _ => false
        }
    }

    fn is_falsy(&self) -> bool {
        !self.is_truthy()
        // match self {
        //     Value::Json(v) => v.is_null(),
        //     Value::Undefined => true,
        //     _ => false
        // }
    }

    fn to_upper_case(&self) -> Value {
        match self {
            Value::Json(v) => Value::Json(v.to_string().to_uppercase().parse().unwrap()),
            _ => panic!("unexpected value")
        }
    }

    fn unwrap_str(&self) -> &str {
        match self {
            Value::Json(v) => v.as_str().unwrap(),
            _ => panic!("unexpected value")
        }
    }

    fn unwrap_bool(&self) -> bool {
        match self {
            Value::Json(v) => v.as_bool().unwrap(),
            _ => panic!("unexpected value")
        }
    }

    fn unwrap_precise(&self) -> &Precise {
        match self {
            Value::Precise(v) => v,
            _ => panic!("unexpected value")
        }
    }

    fn unwrap_json(&self) -> &serde_json::Value {
        match self {
            Value::Json(v) => v,
            _ => panic!("unexpected value")
        }
    }

    fn unwrap_json_mut(&mut self) -> &mut serde_json::Value {
        match self {
            Value::Json(v) => v,
            _ => panic!("unexpected value")
        }
    }

    fn unwrap_precise_mut(&mut self) -> &mut Precise {
        match self {
            Value::Precise(v) => v,
            _ => panic!("unexpected value")
        }
    }

    fn len(&self) -> usize {
        match self {
            Value::Json(v) => v.as_array().unwrap().len(),
            _ => panic!("unexpected value")
        }
    }

    fn get(&self, key: Value) -> Value {
        match self {
            Value::Json(v) => {
                match v.get(key.unwrap_str()) {
                    Some(v) => Value::Json(v.clone()),
                    None => Value::Undefined
                }
            }
            _ => panic!("unexpected value")
        }
    }

    fn set(&mut self, key: Value, value: Value) {
        match self {
            Value::Json(v) => {
                v.as_object_mut().unwrap().insert(key.unwrap_str().to_string(), value.unwrap_json().clone());
            }
            _ => panic!("unexpected value")
        }
    }

    fn push(&mut self, value: Value) {
        match self {
            Value::Json(v) => {
                v.as_array_mut().unwrap().push(value.unwrap_json().clone());
            }
            _ => panic!("unexpected value")
        }
    }

    fn split(&self, separator: Value) -> Value {
        Value::Json(serde_json::Value::Array(
            self.unwrap_str().split(separator.unwrap_str()).into_iter().map(Into::into).collect()))
    }

    fn contains_key(&self, key: Value) -> bool {
        match self {
            Value::Json(v) => v.as_object().unwrap().contains_key(key.unwrap_str()),
            _ => panic!("unexpected value")
        }
    }

    fn keys(&self) -> Vec<Value> {
        match self {
            Value::Json(v) => {
                v.as_object().unwrap().keys().map(|x| Value::Json(serde_json::Value::String(x.to_string()))).collect()
            }
            _ => panic!("unexpected value")
        }
    }

    fn values(&self) -> Vec<Value> {
        match self {
            Value::Json(v) => {
                v.as_object().unwrap().values().map(|x| Value::Json(x.clone())).collect()
            }
            _ => panic!("unexpected value")
        }
    }

    fn to_array(&self, x: Value) -> Value {
        match self {
            Value::Json(v) if v.is_object() => {
                Value::Json(serde_json::Value::Array(v.as_object().unwrap().values().into_iter().map(|x| x.clone()).collect()))
            }
            _ => x
        }
    }

    fn index_of(&self, x: Value) -> Value {
        match self {
            Value::Json(v) if v.is_string() => {
                let i: i64 = v.as_str().unwrap().find(x.unwrap_str()).unwrap().try_into().unwrap_or(-1);
                Value::Json(serde_json::Value::Number(i.try_into().unwrap()))
            }
            _ => Value::Undefined
        }
    }

    fn typeof_(&self) -> Value {
        match self {
            Value::Json(v) => match v {
                serde_json::Value::Null => Value::Json(serde_json::Value::String("object".to_string())),
                serde_json::Value::Bool(_) => Value::Json(serde_json::Value::String("boolean".to_string())),
                serde_json::Value::Number(_) => Value::Json(serde_json::Value::String("number".to_string())),
                serde_json::Value::String(_) => Value::Json(serde_json::Value::String("string".to_string())),
                serde_json::Value::Array(_) => Value::Json(serde_json::Value::String("object".to_string())),
                serde_json::Value::Object(_) => Value::Json(serde_json::Value::String("object".to_string())),
                // _ => Value::Json(serde_json::Value::String("undefined".to_string()))
            }
            _ => Value::Undefined
        }
    }
}

fn parse_int(x: Value) -> Value {
    match x {
        Value::Json(v) if v.is_number() => {
            let w: u64 = if v.is_i64() {
                v.as_i64().unwrap().try_into().unwrap()
            } else if v.is_f64() {
                v.as_f64().unwrap().round() as u64
            } else if v.is_u64() {
                v.as_u64().unwrap()
            } else {
                panic!("unexpected value")
            };
            w.into()
        }
        _ => Value::Undefined
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Precise {
    value: BigInt,
    decimals: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Object {}

impl Object {
    pub fn keys(x: Value) -> Value {
        Value::Json(serde_json::Value::Array(x.keys().into_iter().map(|x| x.unwrap_json().clone()).collect()))
    }

    pub fn values(x: Value) -> Value {
        Value::Json(serde_json::Value::Array(x.values().into_iter().map(|x| x.unwrap_json().clone()).collect()))
    }
}

struct Math {}

impl Math {
    fn max(x: Value, y: Value) -> Value {
        match (x, y) {
            (Value::Json(v1), Value::Json(v2)) if v1.is_number() && v2.is_number() => {
                Value::Json(if v1.as_f64().unwrap() > v2.as_f64().unwrap() {
                    v1.clone()
                } else {
                    v2.clone()
                })
            }
            _ => Value::Undefined
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Array {}

impl Array {
    pub fn is_array(x: Value) -> Value {
        matches!(x, Value::Json(v) if v.is_array()).into()
    }
}


impl Precise {
    pub fn new(val: Value) -> Value {
        let x = val.unwrap_str();
        let mut modifier = 0;
        let mut number = x.to_lowercase();
        if number.contains('e') {
            let splits = number.split('e').map(|x| x.to_owned()).collect::<Vec<_>>();
            number = splits.get(0).unwrap().to_string();
            modifier = splits.get(1).unwrap().parse::<i32>().unwrap();
        }
        let decimals = match number.find('.') {
            Some(i) => number.len() - i - 1,
            None => 0,
        };
        let integer_string = number.replace(".", "");
        Self::new_with_decimals(integer_string.into(), (decimals - modifier as usize).try_into().unwrap())
    }

    pub fn new_with_decimals(val: Value, decimals: u32) -> Value {
        Value::Precise(Self {
            value: BigInt::from_str(val.unwrap_str().try_into().unwrap()).unwrap(),
            decimals,
        })
    }

    pub fn mul(&self, other: &Value) -> Value {
        Value::Precise(Self {
            value: self.value.checked_mul(&other.unwrap_precise().value).unwrap(),
            decimals: self.decimals + other.unwrap_precise().decimals,
        })
    }

    pub fn div(&self, other: &Value, precision: Option<u32>) -> Value {
        let precision1 = precision.unwrap_or(18);
        let distance: i32 = (precision1 - self.decimals + other.unwrap_precise().decimals).try_into().unwrap();
        let numerator = if distance == 0 {
            self.value.clone()
        } else if distance < 0 {
            self.value.checked_div(&BigInt::from(PRECISE_BASE).pow(distance.abs().try_into().unwrap())).unwrap()
        } else {
            self.value.checked_mul(&BigInt::from(PRECISE_BASE).pow(distance.abs().try_into().unwrap())).unwrap()
        };
        Value::Precise(Self {
            value: numerator.div(&other.unwrap_precise().value),
            decimals: precision1,
        })
    }

    pub fn add(&self, other: &Value) -> Value {
        let other = other.unwrap_precise();
        Value::Precise(if self.decimals == other.decimals {
            Self {
                value: self.value.checked_add(&other.value).unwrap(),
                decimals: self.decimals,
            }
        } else {
            let (smaller, bigger) = if self.decimals > other.decimals {
                (other, self)
            } else {
                (self, other)
            };
            let exponent = bigger.decimals - smaller.decimals;
            let normalised = smaller.value.checked_mul(&BigInt::from(PRECISE_BASE).pow(exponent)).unwrap();
            let result = normalised.add(&bigger.value);
            Self {
                value: result,
                decimals: bigger.decimals,
            }
        })
    }

    pub fn r#mod(&self, other: &Value) -> Value {
        let other = other.unwrap_precise();
        // XXX
        let rationizer_numerator: u32 = max(-(self.decimals as i32) + other.decimals as i32, 0).try_into().unwrap();
        let numerator = self.value.checked_mul(&BigInt::from(PRECISE_BASE).pow(rationizer_numerator)).unwrap();
        let rationizer_denominator: u32 = max(-(other.decimals as i32) + self.decimals as i32, 0).try_into().unwrap();
        let denominator = other.value.checked_mul(&BigInt::from(PRECISE_BASE).pow(rationizer_denominator)).unwrap();
        let result = numerator.mod_floor(&denominator);
        Value::Precise(Self {
            value: result,
            decimals: rationizer_denominator + other.decimals,
        })
    }

    pub fn sub(&self, other: &Value) -> Value {
        let other = other.unwrap_precise();
        self.add(&other.neg())
    }

    pub fn abs(&self) -> Value {
        Value::Precise(Self {
            value: self.value.abs(),
            decimals: self.decimals,
        })
    }

    pub fn neg(&self) -> Value {
        Value::Precise(Self {
            value: self.value.checked_mul(&BigInt::from(-1)).unwrap(),
            decimals: self.decimals,
        })
    }

    pub fn min(&self, other: &Value) -> Value {
        todo!()
    }

    pub fn max(&self, other: &Value) -> &Value {
        todo!()
        // if self.gt(other) { self } else { other }
    }

    pub fn gt(&self, other: &Value) -> bool {
        self.sub(other).unwrap_precise().value.is_positive()
    }

    pub fn ge(&self, other: &Value) -> bool {
        self.gt(other) || self.eq(other.unwrap_precise())
    }

    pub fn lt(&self, other: &Value) -> bool {
        self.sub(other).unwrap_precise().value.is_negative()
    }

    pub fn le(&self, other: &Value) -> bool {
        self.lt(other) || self.eq(other.unwrap_precise())
    }

    pub fn reduce(&mut self) {
        let string = self.value.to_string();
        let start = string.len() - 1;
        if start == 0 {
            if start == 0 {
                self.decimals = 0;
            }
            return;
        }
        let mut i = 0;
        let chars = string.chars().collect::<Vec<_>>();
        for i in (0..=start).rev() {
            if chars[i] != '0' {
                break;
            }
        }
        let difference = start - i;
        if difference == 0 {
            return;
        }

        self.decimals -= difference as u32;
        self.value = BigInt::from_str(&string[0..=i]).unwrap()
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    pub fn equals(&mut self, other: &mut Value) -> bool {
        let other = other.unwrap_precise_mut();
        self.reduce();
        other.reduce();
        self.value == other.value && self.decimals == other.decimals
    }

    pub fn string_mul(x: Value, y: Value) -> Value {
        Precise::new(x).unwrap_precise().mul(&Precise::new(y))
    }

    pub fn string_div(x: Value, y: Value, precision: Value) -> Value {
        Precise::new(x).unwrap_precise().div(&Precise::new(y), if precision.is_undefined() {
            None
        } else {
            Some(precision.unwrap_precise().value.clone().try_into().unwrap())
        })
    }

    pub fn string_add(x: Value, y: Value) -> Value {
        Precise::new(x).unwrap_precise().add(&Precise::new(y))
    }

    pub fn string_sub(x: Value, y: Value) -> Value {
        Precise::new(x).unwrap_precise().sub(&Precise::new(y))
    }

    pub fn string_abs(x: Value) -> Value {
        Precise::new(x).unwrap_precise().abs()
    }

    pub fn string_neg(x: Value) -> Value {
        Precise::new(x).unwrap_precise().neg()
    }

    pub fn string_mod(x: Value, y: Value) -> Value {
        Precise::new(x).unwrap_precise().r#mod(&Precise::new(y))
    }

    pub fn string_equals(x: Value, y: Value) -> bool {
        Precise::new(x).unwrap_precise().eq(&Precise::new(y).unwrap_precise())
    }

    pub fn string_eq(x: Value, y: Value) -> bool {
        Precise::new(x).unwrap_precise().eq(&Precise::new(y).unwrap_precise())
    }

    pub fn string_min(x: Value, y: Value) -> Value {
        let x1 = Precise::new(x);
        let y1 = Precise::new(y);
        if x1.lt(&y1) { x1 } else { y1 }
    }

    pub fn string_max(x: Value, y: Value) -> Value {
        let x1 = Precise::new(x);
        let y1 = Precise::new(y);
        if x1.gt(&y1) { x1 } else { y1 }
    }

    pub fn string_gt(x: Value, y: Value) -> bool {
        Precise::new(x).gt(&Precise::new(y))
    }

    pub fn string_ge(x: Value, y: Value) -> bool {
        Precise::new(x).ge(&Precise::new(y))
    }

    pub fn string_lt(x: Value, y: Value) -> bool {
        Precise::new(x).lt(&Precise::new(y))
    }

    pub fn string_le(x: Value, y: Value) -> bool {
        Precise::new(x).le(&Precise::new(y))
    }
}

impl ToString for Precise {
    fn to_string(&self) -> String {
        // self.reduce(); // XXX
        let (sign, abs) = if self.value.is_negative() {
            ("-", self.value.abs())
        } else {
            ("", self.value.clone())
        };
        let abs_string = abs.to_string();
        let integer_array: Vec<&str> = if abs_string.len() < self.decimals as usize {
            let mut array = vec!["0"; (self.decimals - abs_string.len() as u32) as usize];
            todo!()
            // array.extend(abs_string.chars().collect());
            // array
        } else {
            todo!()
            // abs_string.chars().collect()
        };
        let index = integer_array.len() as u32 - self.decimals;
        let item = if index == 0 {
            "0."
        } else if self.decimals < 0 {
            todo!()
            // "0".repeat(self.decimals as usize)
        } else if self.decimals == 0 {
            ""
        } else {
            "."
        };
        todo!()
        // format!("{}{}{}{}", sign, &integer_array[0..index].iter().collect(), item, &integer_array[index..].iter().collect())
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Json(serde_json::Value::String(s.to_string()))
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Json(serde_json::Value::String(s))
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Value::Json(serde_json::Value::String(s.to_owned()))
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Json(serde_json::Value::Number(serde_json::Number::from(i)))
    }
}

impl From<u64> for Value {
    fn from(i: u64) -> Self {
        Value::Json(serde_json::Value::Number(serde_json::Number::from(i)))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Json(serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap()))
    }
}

impl From<usize> for Value {
    fn from(i: usize) -> Self {
        Value::Json(serde_json::Value::Number(serde_json::Number::from(i)))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Json(serde_json::Value::Bool(b))
    }
}

impl<T: Into<serde_json::Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        Value::Json(serde_json::Value::Array(v.into_iter().map(|x| x.into()).collect()))
    }
}

impl From<serde_json::Value> for Value {
    fn from(v: serde_json::Value) -> Self {
        Value::Json(v)
    }
}

impl From<&serde_json::Value> for Value {
    fn from(v: &serde_json::Value) -> Self {
        Value::Json(v.clone())
    }
}

impl Into<serde_json::Value> for Value {
    fn into(self) -> serde_json::Value {
        self.unwrap_json().clone()
    }
}

impl Not for Value {
    type Output = Value;
    fn not(self) -> Self::Output {
        match self {
            Value::Json(v) => Value::Json(serde_json::Value::Bool(!v.as_bool().unwrap())),
            Value::Undefined => Value::Json(serde_json::Value::Bool(true)),
            _ => panic!("Not not implemented for {:?}", self),
        }
    }
}

impl Add for Value {
    type Output = Value;
    fn add(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Json(v1), Value::Json(v2)) if v1.is_number() && v2.is_number() => {
                Value::Json(serde_json::Value::Number(serde_json::Number::from_f64(v1.as_f64().unwrap() + v2.as_f64().unwrap()).unwrap()))
            }
            (Value::Json(v1), Value::Undefined) => Value::Undefined,
            (Value::Undefined, Value::Json(v2)) => Value::Undefined,
            (Value::Undefined, Value::Undefined) => Value::Undefined,
            _ => Value::Undefined
        }
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Json(x), Value::Json(y)) if x.is_number() && y.is_number() => (x.as_f64().unwrap() * y.as_f64().unwrap()).into(),
            _ => panic!("type error"),
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Json(x), Value::Json(y)) if x.is_number() && y.is_number() => (x.as_f64().unwrap() - y.as_f64().unwrap()).into(),
            _ => panic!("type error"),
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Json(x), Value::Json(y)) if x.is_number() && y.is_number() => (x.as_f64().unwrap() / y.as_f64().unwrap()).into(),
            _ => panic!("type error"),
        }
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        self.unwrap_json().as_bool().unwrap()
    }
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.unwrap_json(), other.unwrap_json()) {
            (serde_json::Value::Number(x), serde_json::Value::Number(y)) => {
                if x.is_f64() {
                    if y.is_f64() {
                        x.as_f64().unwrap().partial_cmp(&y.as_f64().unwrap())
                    } else if y.is_u64() {
                        x.as_f64().unwrap().partial_cmp(&(y.as_u64().unwrap() as u32).try_into().unwrap())
                    } else if y.is_i64() {
                        x.as_f64().unwrap().partial_cmp(&(y.as_i64().unwrap() as i32).try_into().unwrap())
                    } else {
                        panic!("unexpected type")
                    }
                } else if y.is_f64() {
                    if x.is_u64() {
                        x.as_u64().unwrap().partial_cmp(&(y.as_u64().unwrap() as u32).try_into().unwrap())
                    } else if x.is_i64() {
                        x.as_i64().unwrap().partial_cmp(&(y.as_i64().unwrap() as i32).try_into().unwrap())
                    } else {
                        panic!("unexpected type")
                    }
                } else {
                    x.as_i64().unwrap().partial_cmp(&y.as_i64().unwrap())
                }
            }
            (serde_json::Value::String(x), serde_json::Value::String(y)) => x.partial_cmp(&y),
            (serde_json::Value::Bool(x), serde_json::Value::Bool(y)) => x.partial_cmp(&y),
            (serde_json::Value::Null, serde_json::Value::Null) => Some(Ordering::Equal),
            (serde_json::Value::Null, _) => Some(Ordering::Less),
            (_, serde_json::Value::Null) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub trait Exchange: ValueTrait {
    fn safe_ledger_entry(&mut self, entry: Value, currency_option: Value) -> Value;
}

impl dyn Exchange {
    // pub fn safe_string(&self, x: JsonValue, key: JsonValue, default_value: Option<JsonValue>) -> Option<JsonValue> {
    //     x.get(key.as_str().expect("given key is not a string").map(|x| x.to_owned()).or(default_value))
    // }

    fn set_number_mode(&mut self, mode: Value) {
        self.set("___number_mode".into(), mode);
    }

    pub fn parse_number(&self, value: Value, default: Value) -> Value {
        if value.is_undefined() {
            return default;
        }

        match value.clone() {
            Value::Json(x) => {
                if x.is_number() {
                    value
                } else if x.is_string() {
                    Value::Json(serde_json::Value::Number(serde_json::Number::from_f64(x.as_str().unwrap().parse::<f64>().unwrap()).unwrap()))
                } else {
                    default
                }
            }
            _ => return default,
        }
    }

    pub fn extend_2(&self, x: Value, y: Value) -> Value {
        let mut x1 = x.unwrap_json().clone();
        let mut y1 = y.unwrap_json().clone();
        let x = x1.as_object_mut().unwrap();
        let y = y1.as_object_mut().unwrap();
        for (k, v) in y {
            x.insert(k.to_owned(), v.clone());
        }
        serde_json::Value::Object(x.clone()).into()
    }

    pub fn deep_extend_2(&self, x1: Value, x2: Value) -> Value {
        let mut result = Value::Undefined;
        for arg in [&x1, &x2] {
            if arg.unwrap_json().is_object() {
                if result.is_undefined() || !result.unwrap_json().is_object() {
                    result = Value::Json(json!({}));
                }
                let result1 = result.clone();
                for key in arg.unwrap_json().as_object().unwrap().keys() {
                    result.unwrap_json_mut().as_object_mut().unwrap().insert(
                        key.to_owned(), self.deep_extend_2(
                            if result1.contains_key(key.into()) { result1.get(key.into()) } else { Value::Undefined },
                            arg.get(key.into()),
                        ).unwrap_json().clone(),
                    );
                }
            }
        }
        result
    }

    pub fn deep_extend_4(&self, x1: Value, x2: Value, x3: Value, x4: Value) -> Value {
        let mut result = Value::Undefined;
        for arg in [&x1, &x2, &x3, &x4] {
            if arg.unwrap_json().is_object() {
                if result.is_undefined() || !result.unwrap_json().is_object() {
                    result = Value::Json(json!({}));
                }
                for key in arg.unwrap_json().as_object().unwrap().keys() {
                    let result1 = result.clone();
                    result.unwrap_json_mut().as_object_mut().unwrap().insert(
                        key.to_owned(), self.deep_extend_2(
                            if result1.contains_key(key.into()) { result1.get(key.into()) } else { Value::Undefined },
                            arg.get(key.into()),
                        ).unwrap_json().clone(),
                    );
                }
            }
        }
        result
    }

    pub fn in_array(&self, needle: Value, haystack: Value) -> Value {
        match haystack {
            Value::Json(x) if x.is_array() => x.as_array().unwrap().contains(&needle.unwrap_json()).into(),
            _ => panic!("haystack is not an array"),
        }
    }

    pub fn omit_zero(&self, string_number: Value) -> Value {
        if string_number.is_falsy() { Value::Undefined } else { string_number }
    }

    pub fn omit(&self, x: Value, keys: Value) -> Value {
        match x {
            Value::Json(x1) => {
                match x1 {
                    serde_json::Value::Object(x2) => {
                        let mut result = serde_json::Map::new();
                        for key in x2.keys() {
                            if !keys.contains_key(key.into()) {
                                result.insert(key.to_owned(), x2.get(key.into()).unwrap().clone());
                            }
                        }
                        Value::Json(serde_json::Value::Object(result))
                    }
                    _ => x1.clone().into()
                }
            }
            _ => panic!("x is not Json"),
        }
    }

    pub fn group_by(&self, array: Value, key: Value) -> Value {
        let mut result = serde_json::Map::new();
        for entry in self.to_array(array).unwrap_json().as_array().unwrap() {
            if let Some(item) = entry.as_object().unwrap().get(key.unwrap_str()) {
                if !item.is_null() {
                    let item_as_str = item.as_str().unwrap();
                    if !result.contains_key(item_as_str) {
                        result.insert(item_as_str.to_owned(), json!([]));
                    }
                    result.get_mut(item_as_str).unwrap().as_array_mut().unwrap().push(entry.clone());
                }
            }
        }
        serde_json::Value::Object(result).into()
    }

    pub fn safe_string(&self, x: Value, key: Value, default_value: Value) -> Value {
        let rv = match x {
            Value::Json(j) => match j {
                serde_json::Value::Object(o) => {
                    match o.get(key.unwrap_str()) {
                        Some(v) if v.is_string() => Value::Json(v.clone()),
                        _ => Value::Undefined
                    }
                }
                _ => Value::Undefined
            },
            _ => Value::Undefined
        };

        return if rv.is_undefined() {
            default_value
        } else {
            rv
        };
    }

    pub fn msec(&self) -> Value {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_u64().unwrap().into()
    }

    pub fn usec(&self) -> Value {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_u64().unwrap().into()
    }

    pub fn seconds(&self) -> Value {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_u64().unwrap().into()
    }

    pub fn milliseconds(&self) -> Value {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_u64().unwrap().into()
    }

    pub fn microseconds(&self) -> Value {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_u64().unwrap().into()
    }

    pub fn safe_string_lower(&self, x: Value, key: Value, default_value: Value) -> Value {
        let rv = match x {
            Value::Json(j) => match j {
                serde_json::Value::Object(o) => {
                    match o.get(key.unwrap_str()) {
                        Some(v) if v.is_string() => v.to_string().to_lowercase().into(),
                        _ => Value::Undefined
                    }
                }
                _ => Value::Undefined
            },
            _ => Value::Undefined
        };

        return if rv.is_undefined() {
            default_value
        } else {
            rv
        };
    }

    pub fn safe_string_upper(&self, x: Value, key: Value, default_value: Value) -> Value {
        let rv = match x {
            Value::Json(j) => match j {
                serde_json::Value::Object(o) => {
                    match o.get(key.unwrap_str()) {
                        Some(v) if v.is_string() => v.to_string().to_uppercase().into(),
                        _ => Value::Undefined
                    }
                }
                _ => Value::Undefined
            },
            _ => Value::Undefined
        };

        return if rv.is_undefined() {
            default_value
        } else {
            rv
        };
    }

    pub fn safe_integer(&self, x: Value, key: Value, default_value: Value) -> Value {
        let rv = match self.safe_value(x, key, Value::Undefined) {
            Value::Json(j) => match j {
                serde_json::Value::Number(o) => {
                    Value::Json(serde_json::Value::from(o.to_string().parse::<i64>().unwrap()))
                }
                _ => Value::Undefined
            },
            _ => Value::Undefined
        };

        if rv.is_undefined() {
            match default_value {
                Value::Json(j) => Value::Json(j),
                _ => Value::Undefined
            }
        } else {
            rv
        }
    }

    pub fn safe_value(&self, x: Value, key: Value, default_value: Value) -> Value {
        let rv = match x {
            Value::Json(j) => match j {
                serde_json::Value::Object(o) => {
                    match o.get(key.unwrap_str()) {
                        Some(v) => Value::Json(v.clone()),
                        _ => Value::Undefined
                    }
                }
                _ => Value::Undefined
            },
            _ => Value::Undefined
        };

        if rv.is_undefined() {
            match default_value {
                Value::Json(j) => Value::Json(j),
                _ => Value::Undefined
            }
        } else {
            rv
        }
    }

    // pub fn common_currency_code(&self, currency: Value) -> Value {
    //     if self.get("substituteCommonCurrencyCodes".into()).is_undefined() {
    //         return currency;
    //     }
    //     self.safe_string(self.get("commonCurrencies".into()), currency, currency.clone())
    // }

    // pub fn safe_currency(&self, currency_id: Value, mut currency: Value) -> Value {
    //     if currency_id.is_undefined() && !currency.is_undefined() {
    //         return currency;
    //     }
    //
    //     let currencies_by_id = self.get("currencies_by_id".into());
    //     if !currencies_by_id.is_undefined() {
    //         currency = self.safe_value(currencies_by_id, currency_id.clone(), Value::Undefined);
    //         if !currency.is_undefined() {
    //             return currency;
    //         }
    //     }
    //
    //     let mut code = currency_id.clone();
    //     if !currency_id.is_undefined() {
    //         code = self.common_currency_code(currency_id.unwrap_str().to_uppercase().into());
    //     }
    //
    //     Value::Json(json!({
    //         "id": currency_id.unwrap_json(),
    //         "code": code.unwrap_json(),
    //     }))
    // }

    // pub fn safe_currency_code(&self, currency_id: Value, mut currency: Value) -> Value {
    //     currency = self.safe_currency(currency_id, currency);
    //     currency.get("code".into())
    // }

    // pub fn safe_market(&self, market_id: Value, mut market: Value, delimiter: Value) -> Value {
    //     let mut result = json!({
    //         "id": market_id.unwrap_json(),
    //         "symbol": market_id.unwrap_json(),
    //         // "base": undefined,
    //         // "quote": undefined,
    //         // "baseId": undefined,
    //         // "quoteId": undefined,
    //         // "active": undefined,
    //         // "type": undefined,
    //         // "linear": undefined,
    //         // "inverse": undefined,
    //         "spot": false,
    //         "swap": false,
    //         "future": false,
    //         "option": false,
    //         "margin": false,
    //         "contract": false,
    //         // "contractSize": undefined,
    //         // "expiry": undefined,
    //         // "expiryDatetime": undefined,
    //         // "optionType": undefined,
    //         // "strike": undefined,
    //         // "settle": undefined,
    //         // "settleId": undefined,
    //         "precision": {
    //             // "amount": undefined,
    //             // "price": undefined,
    //         },
    //         "limits": {
    //             "amount": {
    //                 // "min": undefined,
    //                 // "max": undefined,
    //             },
    //             "price": {
    //                 // "min": undefined,
    //                 // "max": undefined,
    //             },
    //             "cost": {
    //                 // "min": undefined,
    //                 // "max": undefined,
    //             },
    //         },
    //         // "info": undefined,
    //     }).as_object().unwrap();
    //
    //     if !market_id.is_undefined() {
    //         let markets_by_id = self.get("markets_by_id".into());
    //         if !markets_by_id.is_undefined() {
    //             market = self.safe_value(markets_by_id, market_id, Value::Undefined);
    //         } else if !delimiter.is_undefined() {
    //             let parts: Value = market_id.unwrap_str().split(delimiter.unwrap_str()).collect::<Vec<&str>>().into();
    //             if parts.len() == 2 {
    //                 let base_id = self.safe_string(parts.clone(), 0.into(), Value::Undefined).unwrap_json().clone();
    //                 let quote_id = self.safe_string(parts.clone(), 1.into(), Value::Undefined).unwrap_json().clone();
    //                 let base = self.safe_currency_code(base_id.into(), Value::Undefined).unwrap_json().clone();
    //                 let quote = self.safe_currency_code(quote_id.into(), Value::Undefined).unwrap_json().clone();
    //                 let symbol = format!("{}/{}", base, quote);
    //                 result.insert("baseId".into(), base_id.clone());
    //                 result.insert("quoteId".into(), quote_id.clone());
    //                 result.insert("base".into(), base.clone());
    //                 result.insert("quote".into(), quote.clone());
    //                 result.insert("symbol".into(), symbol.into());
    //             }
    //             return Value::Json(result.to_owned().into());
    //         }
    //     }
    //
    //     if !market.is_undefined() {
    //         return market;
    //     }
    //
    //     Value::Json(result.to_owned().into())
    // }

    pub fn safe_value_2(&self, x: Value, key1: Value, key2: Value, default_value: Value) -> Value {
        self.safe_value(x.clone(), key1, Value::Undefined).or_default(
            self.safe_value(x, key2, default_value))
    }

    pub fn safe_string_2(&self, x: Value, key1: Value, key2: Value, default_value: Value) -> Value {
        self.safe_string(x.clone(), key1, Value::Undefined).or_default(
            self.safe_string(x, key2, default_value))
    }

    pub fn safe_string_lower_2(&self, x: Value, key1: Value, key2: Value, default_value: Value) -> Value {
        self.safe_string_lower(x.clone(), key1, Value::Undefined).or_default(
            self.safe_string_lower(x, key2, default_value))
    }

    pub fn safe_string_upper_2(&self, x: Value, key1: Value, key2: Value, default_value: Value) -> Value {
        self.safe_string_upper(x.clone(), key1, Value::Undefined).or_default(
            self.safe_string_upper(x, key2, default_value))
    }

    pub fn keysort(&self, dictionary: Value) -> Value {
        let obj = dictionary.unwrap_json().as_object().unwrap();
        let mut keys = obj.keys().into_iter().collect::<Vec<_>>();
        keys.sort();
        let mut result = serde_json::Map::new();
        for k in keys {
            result.insert(k.clone(), obj.get(k).unwrap().clone());
        }
        Value::Json(result.into())
    }

    pub fn index_by(&self, array: Value, key: Value) -> Value {
        let mut result: serde_json::Map<String, serde_json::Value> = Default::default();
        let mut array = array.unwrap_json();
        let mut temp = serde_json::Value::Array(vec![]);
        if array.is_object() {
            let sorted = self.keysort(array.clone().into());
            let values = sorted.unwrap_json().as_object().unwrap().values().into_iter().collect::<Vec<_>>();
            temp = serde_json::Value::Array(values.into_iter().map(|x| x.to_owned()).collect());
            array = &temp;
        }
        let is_int_key = key.unwrap_json().is_u64();
        for element in array.as_array().unwrap() {
            if (is_int_key && element.is_array() && (key < element.as_array().unwrap().len().into())) || (element.is_object() && element.as_object().unwrap().contains_key(key.unwrap_str())) {
                let k = if element.is_array() {
                    element.as_array().unwrap()[key.unwrap_json().as_u64().unwrap() as usize].clone()
                } else {
                    element.as_object().unwrap().get(key.unwrap_str()).unwrap().clone()
                };

                if !k.is_null() {
                    result.insert(k.as_str().unwrap().to_owned(), element.clone());
                }
            }
        }
        Value::Json(serde_json::Value::Object(result))
    }

    pub fn sort_by(&self, array: Value, key: Value, descending: Value) -> Value {
        let descending = descending.or_default(false.into());
        let mut array = array.unwrap_json().as_array().unwrap().clone();
        array.sort_by_key(|x| x.get(key.unwrap_str()).map(|x| {
            let y: Value = x.clone().into();
            y
        }).unwrap_or("".into()));
        if descending.unwrap_bool() {
            array.reverse();
        }
        Value::Json(serde_json::Value::Array(array))
    }

    pub fn sort_by_2(&self, array: Value, key1: Value, key2: Value, descending: Value) -> Value {
        let descending = descending.or_default(false.into());
        let mut array = array.unwrap_json().as_array().unwrap().clone();
        array.sort_by_key(|x| x.get(key1.unwrap_str()).map(Into::<Value>::into).unwrap_or(
            x.get(key2.unwrap_str()).map(Into::<Value>::into).unwrap_or("".into())).clone());
        if descending.unwrap_bool() {
            array.reverse();
        }
        Value::Json(serde_json::Value::Array(array))
    }

    pub fn array_concat(&self, a: Value, b: Value) -> Value {
        let mut array = a.unwrap_json().as_array().unwrap().clone();
        array.extend(b.unwrap_json().as_array().unwrap().clone());
        Value::Json(serde_json::Value::Array(array))
    }

    pub fn is_empty(&self, object: Value) -> Value {
        let object = object.unwrap_json();
        if object.is_object() {
            Value::Json(serde_json::Value::Bool(object.as_object().unwrap().is_empty()))
        } else if object.is_array() {
            Value::Json(serde_json::Value::Bool(object.as_array().unwrap().is_empty()))
        } else {
            Value::Json(serde_json::Value::Bool(false))
        }
    }

    fn parse_transaction(&self, mut transaction: Value, mut currency: Value) -> Value { todo!() }
    fn parse_transfer(&self, mut transfer: Value, mut currency: Value) -> Value { todo!() }
    fn parse_market_leverage_tiers(&self, info: Value, market: Value) -> Value { todo!() }
    fn sign(&self, path: Value, api: Value, method: Value, params: Value, headers: Value, body: Value) -> Value { todo!() }
    async fn fetch_accounts(&self, parmas: Value) -> Value { todo!() }

    fn decimal_to_precision(&self, n: Value, rounding_mode: Value, precision: Value, counting_mode: Value, padding_mode: Value) -> Value { todo!() }
    fn number_to_string(&self, x: Value) -> Value { todo!() }
    async fn fetch_trades(&self, symbol: Value, since: Value, limit: Value, params: Value) -> Value { todo!() }

    fn parse_ticker(&self, ticker: Value, market: Value) -> Value { todo!() }
    // TODO
    fn filter_by_value_since_limit(&self, array: Value, field: Value, value: Value, since: Value, limit: Value, key: Value, tail: Value) -> Value { todo!() }
    fn parse_deposit_address(&self, deposit_address: Value, currency: Value) -> Value { todo!() }
    fn parse_borrow_interest(&self, info: Value, market: Value) -> Value { todo!() }
    fn parse_funding_rate_history(&self, info: Value, market: Value) -> Value { todo!() }
    // TODO
    fn totp(&self, key: Value) -> Value { todo!() }
    fn parse_trading_limits(&self, limits: Value, symbol: Value, params: Value) -> Value { todo!() }
    fn parse_trade(&self, trade: Value, market: Value) -> Value { todo!() }
    fn parse_ledger_entry(&self, item: Value, currency: Value) -> Value { todo!() }
    fn parse_position(&self, position: Value, market: Value) -> Value { todo!() }
    // TODO
    fn implode_params(&self, string: Value, params: Value) -> Value { todo!() }
    // TODO
    fn extract_params(&self, string: Value, params: Value) -> Value { todo!() }
    async fn fetch_trading_limits_by_id(&self, id: Value, params: Value) -> Value { todo!() }
    // TODO
    fn filter_by_since_limit(&self, array: Value, since: Value, limit: Value, key: Value, tail: Value) -> Value { todo!() }
    // TODO
    fn aggregate(&self, bidasks: Value) -> Value { todo!() }
    fn parse_order(&self, order: Value, market: Value) -> Value { todo!() }

    fn iso8601(&self, timestamp: Value) -> Value { todo!() }
    // fn fetch_borrow_rate(&self, code: Value, params: Value) -> Value { todo!() }
    async fn load_markets(&self, reload: Value, params: Value) -> Value { todo!() }
    async fn fetch_time(&self, params: Value) -> Value { todo!() }
    fn safe_string_n(&self, dictionary: Value, key_list: Value, default_value: Value) -> Value { todo!() }
    async fn fetch_funding_rates(&self, symbols: Value, params: Value) -> Value { todo!() }
    async fn fetch_leverage_tiers(&self, symbols: Value, params: Value) -> Value { todo!() }
    fn build_ohlcvc(&self, trades: Value, timeframe: Value, since: Value, limit: Value) -> Value { todo!() }
    async fn fetch(&self, url: Value, method: Value, headers: Value, body: Value) -> Value { todo!() }
    async fn throttle(&self, cost: Value) -> Value { todo!() }
    fn safe_timestamp(&self, dictionary: Value, key: Value, default_value: Value) -> Value { todo!() }

    async fn fetch_deposit_addresses(&self, codes: Value, params: Value) -> Value { todo!() }
    async fn fetch_borrow_rates(&self, params: Value) -> Value { todo!() }
    async fn fetch_order_book(&self, symbol: Value, limit: Value, params: Value) -> Value { todo!() }
    async fn fetch_trading_limits(&self, symbols: Value, params: Value) -> Value { todo!() }

    // METHODS BELOW THIS LINE ARE TRANSPILED FROM JAVASCRIPT TO PYTHON AND PHP
    fn safe_ledger_entry(&self, mut entry: Value, mut currency: Value) -> Value {
        currency = self.safe_currency(Value::Undefined, currency.clone());
        let mut direction: Value = self.safe_string(entry.clone(), Value::from("direction"), Value::Undefined);
        let mut before: Value = self.safe_string(entry.clone(), Value::from("before"), Value::Undefined);
        let mut after: Value = self.safe_string(entry.clone(), Value::from("after"), Value::Undefined);
        let mut amount: Value = self.safe_string(entry.clone(), Value::from("amount"), Value::Undefined);
        if amount.clone() != Value::Undefined {
            if before.clone() == Value::Undefined && after.clone() != Value::Undefined {
                before = Precise::string_sub(after.clone(), amount.clone());
            } else if before.clone() != Value::Undefined && after.clone() == Value::Undefined {
                after = Precise::string_add(before.clone(), amount.clone());
            };
        };
        if before.clone() != Value::Undefined && after.clone() != Value::Undefined {
            if direction.clone() == Value::Undefined {
                if Precise::string_gt(before.clone(), after.clone()) {
                    direction = Value::from("out");
                };
                if Precise::string_gt(after.clone(), before.clone()) {
                    direction = Value::from("in");
                };
            };
        };
        let mut fee: Value = self.safe_value(entry.clone(), Value::from("fee"), Value::Undefined);
        if fee.clone() != Value::Undefined {
            fee.set("cost".into(), self.safe_number(fee.clone(), Value::from("cost"), Value::Undefined));
        };
        let mut timestamp: Value = self.safe_integer(entry.clone(), Value::from("timestamp"), Value::Undefined);
        return Value::Json(json!({
            "id": self.safe_string(entry.clone(), Value::from("id"), Value::Undefined),
            "timestamp": timestamp,
            "datetime": self.iso8601(timestamp.clone()),
            "direction": direction,
            "account": self.safe_string(entry.clone(), Value::from("account"), Value::Undefined),
            "referenceId": self.safe_string(entry.clone(), Value::from("referenceId"), Value::Undefined),
            "referenceAccount": self.safe_string(entry.clone(), Value::from("referenceAccount"), Value::Undefined),
            "type": self.safe_string(entry.clone(), Value::from("type"), Value::Undefined),
            "currency": currency.get(Value::from("code")),
            "amount": self.parse_number(amount.clone(), Value::Undefined),
            "before": self.parse_number(before.clone(), Value::Undefined),
            "after": self.parse_number(after.clone(), Value::Undefined),
            "status": self.safe_string(entry.clone(), Value::from("status"), Value::Undefined),
            "fee": fee,
            "info": entry
        }));
    }

    fn set_markets(&mut self, mut markets: Value, mut currencies: Value) -> Value {
        let mut values: Value = Value::new_array();
        let mut market_values: Value = self.to_array(markets.clone());
        let mut i: usize = 0;
        while Value::from(i) < market_values.len().into() {
            let mut market: Value = self.deep_extend_4(self.safe_market(Value::Undefined, Value::Undefined, Value::Undefined), Value::Json(json!({
                "precision": self.get("precision".into()),
                "limits": self.get("limits".into())
            })), self.get("fees".into()).get(Value::from("trading")), market_values.get(i.into()));
            values.push(market.clone());
            i += 1;
        };
        self.set("markets".into(), self.index_by(values.clone(), Value::from("symbol")));
        self.set("markets_by_id".into(), self.index_by(markets.clone(), Value::from("id")));
        let mut markets_sorted_by_symbol: Value = self.keysort(self.get("markets".into()));
        let mut markets_sorted_by_id: Value = self.keysort(self.get("markets_by_id".into()));
        self.set("symbols".into(), Object::keys(markets_sorted_by_symbol.clone()));
        self.set("ids".into(), Object::keys(markets_sorted_by_id.clone()));
        if currencies.clone() != Value::Undefined {
            self.set("currencies".into(), self.deep_extend_2(self.get("currencies".into()), currencies.clone()));
        } else {
            let mut base_currencies: Value = Value::new_array();
            let mut quote_currencies: Value = Value::new_array();
            let mut i: usize = 0;
            while Value::from(i) < values.len().into() {
                let mut market: Value = values.get(i.into());
                let mut default_currency_precision: Value = if self.get("precision_mode".into()) == DECIMAL_PLACES.into() { 8.into() } else { self.parse_number(Value::from("0.00000001"), Value::Undefined) };
                let mut market_precision: Value = self.safe_value(market.clone(), Value::from("precision"), Value::new_object());
                if market.contains_key(Value::from("base")) {
                    let mut currency_precision: Value = self.safe_value_2(market_precision.clone(), Value::from("base"), Value::from("amount"), default_currency_precision.clone());
                    let mut currency: Value = Value::Json(json!({
                        "id": self.safe_string_2(market.clone(), Value::from("baseId"), Value::from("base"), Value::Undefined),
                        "numericId": self.safe_string(market.clone(), Value::from("baseNumericId"), Value::Undefined),
                        "code": self.safe_string(market.clone(), Value::from("base"), Value::Undefined),
                        "precision": currency_precision
                    }));
                    base_currencies.push(currency.clone());
                };
                if market.contains_key(Value::from("quote")) {
                    let mut currency_precision: Value = self.safe_value_2(market_precision.clone(), Value::from("quote"), Value::from("amount"), default_currency_precision.clone());
                    let mut currency: Value = Value::Json(json!({
                        "id": self.safe_string_2(market.clone(), Value::from("quoteId"), Value::from("quote"), Value::Undefined),
                        "numericId": self.safe_string(market.clone(), Value::from("quoteNumericId"), Value::Undefined),
                        "code": self.safe_string(market.clone(), Value::from("quote"), Value::Undefined),
                        "precision": currency_precision
                    }));
                    quote_currencies.push(currency.clone());
                };
                i += 1;
            };
            base_currencies = self.sort_by(base_currencies.clone(), Value::from("code"), Value::Undefined);
            quote_currencies = self.sort_by(quote_currencies.clone(), Value::from("code"), Value::Undefined);
            self.set("base_currencies".into(), self.index_by(base_currencies.clone(), Value::from("code")));
            self.set("quote_currencies".into(), self.index_by(quote_currencies.clone(), Value::from("code")));
            let mut all_currencies: Value = self.array_concat(base_currencies.clone(), quote_currencies.clone());
            let mut grouped_currencies: Value = self.group_by(all_currencies.clone(), Value::from("code"));
            let mut codes: Value = Object::keys(grouped_currencies.clone());
            let mut resulting_currencies: Value = Value::new_array();
            let mut i: usize = 0;
            while Value::from(i) < codes.len().into() {
                let mut code: Value = codes.get(i.into());
                let mut grouped_currencies_code: Value = self.safe_value(grouped_currencies.clone(), code.clone(), Value::new_array());
                let mut highest_precision_currency: Value = self.safe_value(grouped_currencies_code.clone(), 0.into(), Value::Undefined);
                let mut j: usize = 1;
                while Value::from(j) < grouped_currencies_code.len().into() {
                    let mut current_currency: Value = grouped_currencies_code.get(j.into());
                    if self.get("precision_mode".into()) == TICK_SIZE.into() {
                        highest_precision_currency = if current_currency.get(Value::from("precision")) < highest_precision_currency.get(Value::from("precision")) { current_currency.clone() } else { highest_precision_currency.clone() };
                    } else {
                        highest_precision_currency = if current_currency.get(Value::from("precision")) > highest_precision_currency.get(Value::from("precision")) { current_currency.clone() } else { highest_precision_currency.clone() };
                    };
                    j += 1;
                };
                resulting_currencies.push(highest_precision_currency.clone());
                i += 1;
            };
            let mut sorted_currencies: Value = self.sort_by(resulting_currencies.clone(), Value::from("code"), Value::Undefined);
            self.set("currencies".into(), self.deep_extend_2(self.get("currencies".into()), self.index_by(sorted_currencies.clone(), Value::from("code"))));
        };
        self.set("currencies_by_id".into(), self.index_by(self.get("currencies".into()), Value::from("id")));
        let mut currencies_sorted_by_code: Value = self.keysort(self.get("currencies".into()));
        self.set("codes".into(), Object::keys(currencies_sorted_by_code.clone()));
        return self.get("markets".into());
    }

    fn safe_balance(&self, mut balance: Value) -> Value {
        let mut balances: Value = self.omit(balance.clone(), Value::Json(serde_json::Value::Array(vec![Value::from("info").into(), Value::from("timestamp").into(), Value::from("datetime").into(), Value::from("free").into(), Value::from("used").into(), Value::from("total").into()])));
        let mut codes: Value = Object::keys(balances.clone());
        balance.set("free".into(), Value::new_object());
        balance.set("used".into(), Value::new_object());
        balance.set("total".into(), Value::new_object());
        let mut i: usize = 0;
        while Value::from(i) < codes.len().into() {
            let mut code: Value = codes.get(i.into());
            let mut total: Value = self.safe_string(balance.get(code.clone()), Value::from("total"), Value::Undefined);
            let mut free: Value = self.safe_string(balance.get(code.clone()), Value::from("free"), Value::Undefined);
            let mut used: Value = self.safe_string(balance.get(code.clone()), Value::from("used"), Value::Undefined);
            if total.clone() == Value::Undefined && free.clone() != Value::Undefined && used.clone() != Value::Undefined {
                total = Precise::string_add(free.clone(), used.clone());
            };
            if free.clone() == Value::Undefined && total.clone() != Value::Undefined && used.clone() != Value::Undefined {
                free = Precise::string_sub(total.clone(), used.clone());
            };
            if used.clone() == Value::Undefined && total.clone() != Value::Undefined && free.clone() != Value::Undefined {
                used = Precise::string_sub(total.clone(), free.clone());
            };
            balance.get(code.clone()).set("free".into(), self.parse_number(free.clone(), Value::Undefined));
            balance.get(code.clone()).set("used".into(), self.parse_number(used.clone(), Value::Undefined));
            balance.get(code.clone()).set("total".into(), self.parse_number(total.clone(), Value::Undefined));
            balance.get(Value::from("free")).set(code.clone(), balance.get(code.clone()).get(Value::from("free")));
            balance.get(Value::from("used")).set(code.clone(), balance.get(code.clone()).get(Value::from("used")));
            balance.get(Value::from("total")).set(code.clone(), balance.get(code.clone()).get(Value::from("total")));
            i += 1;
        };
        return balance.clone();
    }

    fn safe_order(&mut self, mut order: Value, mut market: Value) -> Value {
        let mut amount: Value = self.omit_zero(self.safe_string(order.clone(), Value::from("amount"), Value::Undefined));
        let mut remaining: Value = self.safe_string(order.clone(), Value::from("remaining"), Value::Undefined);
        let mut filled: Value = self.safe_string(order.clone(), Value::from("filled"), Value::Undefined);
        let mut cost: Value = self.safe_string(order.clone(), Value::from("cost"), Value::Undefined);
        let mut average: Value = self.omit_zero(self.safe_string(order.clone(), Value::from("average"), Value::Undefined));
        let mut price: Value = self.omit_zero(self.safe_string(order.clone(), Value::from("price"), Value::Undefined));
        let mut last_trade_time_timestamp: Value = self.safe_integer(order.clone(), Value::from("lastTradeTimestamp"), Value::Undefined);
        let mut parse_filled: Value = (filled.clone() == Value::Undefined).into();
        let mut parse_cost: Value = (cost.clone() == Value::Undefined).into();
        let mut parse_last_trade_time_timestamp: Value = (last_trade_time_timestamp.clone() == Value::Undefined).into();
        let mut fee: Value = self.safe_value(order.clone(), Value::from("fee"), Value::Undefined);
        let mut parse_fee: Value = (fee.clone() == Value::Undefined).into();
        let mut parse_fees: Value = (self.safe_value(order.clone(), Value::from("fees"), Value::Undefined) == Value::Undefined).into();
        let mut should_parse_fees: Value = (parse_fee.is_truthy() || parse_fees.is_truthy()).into();
        let mut fees: Value = self.safe_value(order.clone(), Value::from("fees"), Value::new_array());
        let mut trades: Value = Value::new_array();
        if parse_filled.is_truthy() || parse_cost.is_truthy() || should_parse_fees.is_truthy() {
            let mut raw_trades: Value = self.safe_value(order.clone(), Value::from("trades"), trades.clone());
            let mut old_number: Value = self.get("number".into());
            self.set_number_mode("String".into());
            trades = self.parse_trades(raw_trades.clone(), market.clone(), Value::Undefined, Value::Undefined, Value::Json(json!({
                "symbol": order.get(Value::from("symbol")),
                "side": order.get(Value::from("side")),
                "type": order.get(Value::from("type")),
                "order": order.get(Value::from("id"))
            })));
            self.set("number".into(), old_number.clone());
            let mut trades_length: Value = 0.into();
            let mut is_array: Value = Array::is_array(trades.clone());
            if is_array.is_truthy() {
                trades_length = trades.len().into();
            };
            if is_array.is_truthy() && trades_length.clone() > 0.into() {
                if order.get(Value::from("symbol")) == Value::Undefined {
                    order.set("symbol".into(), trades.get(0.into()).get(Value::from("symbol")));
                };
                if order.get(Value::from("side")) == Value::Undefined {
                    order.set("side".into(), trades.get(0.into()).get(Value::from("side")));
                };
                if order.get(Value::from("type")) == Value::Undefined {
                    order.set("type".into(), trades.get(0.into()).get(Value::from("type")));
                };
                if order.get(Value::from("id")) == Value::Undefined {
                    order.set("id".into(), trades.get(0.into()).get(Value::from("order")));
                };
                if parse_filled.is_truthy() {
                    filled = Value::from("0");
                };
                if parse_cost.is_truthy() {
                    cost = Value::from("0");
                };
                let mut i: usize = 0;
                while Value::from(i) < trades.len().into() {
                    let mut trade: Value = trades.get(i.into());
                    let mut trade_amount: Value = self.safe_string(trade.clone(), Value::from("amount"), Value::Undefined);
                    if parse_filled.is_truthy() && trade_amount.clone() != Value::Undefined {
                        filled = Precise::string_add(filled.clone(), trade_amount.clone());
                    };
                    let mut trade_cost: Value = self.safe_string(trade.clone(), Value::from("cost"), Value::Undefined);
                    if parse_cost.is_truthy() && trade_cost.clone() != Value::Undefined {
                        cost = Precise::string_add(cost.clone(), trade_cost.clone());
                    };
                    let mut trade_timestamp: Value = self.safe_value(trade.clone(), Value::from("timestamp"), Value::Undefined);
                    if parse_last_trade_time_timestamp.is_truthy() && trade_timestamp.clone() != Value::Undefined {
                        if last_trade_time_timestamp.clone() == Value::Undefined {
                            last_trade_time_timestamp = trade_timestamp.clone();
                        } else {
                            last_trade_time_timestamp = Math::max(last_trade_time_timestamp.clone(), trade_timestamp.clone());
                        };
                    };
                    if should_parse_fees.is_truthy() {
                        let mut trade_fees: Value = self.safe_value(trade.clone(), Value::from("fees"), Value::Undefined);
                        if trade_fees.clone() != Value::Undefined {
                            let mut j: usize = 0;
                            while Value::from(j) < trade_fees.len().into() {
                                let mut trade_fee: Value = trade_fees.get(j.into());
                                fees.push(self.extend_2(Value::new_object(), trade_fee.clone()));
                                j += 1;
                            };
                        } else {
                            let mut trade_fee: Value = self.safe_value(trade.clone(), Value::from("fee"), Value::Undefined);
                            if trade_fee.clone() != Value::Undefined {
                                fees.push(self.extend_2(Value::new_object(), trade_fee.clone()));
                            };
                        };
                    };
                    i += 1;
                };
            };
        };
        if should_parse_fees.is_truthy() {
            let mut reduced_fees: Value = if self.get("reduce_fees".into()).is_truthy() { self.reduce_fees_by_currency(fees.clone()) } else { fees.clone() };
            let mut reduced_length: Value = reduced_fees.len().into();
            let mut i: usize = 0;
            while Value::from(i) < reduced_length.clone() {
                reduced_fees.get(i.into()).set("cost".into(), self.safe_number(reduced_fees.get(i.into()), Value::from("cost"), Value::Undefined));
                if reduced_fees.get(i.into()).contains_key(Value::from("rate")) {
                    reduced_fees.get(i.into()).set("rate".into(), self.safe_number(reduced_fees.get(i.into()), Value::from("rate"), Value::Undefined));
                };
                i += 1;
            };
            if !parse_fee.is_truthy() && reduced_length.clone() == 0.into() {
                fee.set("cost".into(), self.safe_number(fee.clone(), Value::from("cost"), Value::Undefined));
                if fee.contains_key(Value::from("rate")) {
                    fee.set("rate".into(), self.safe_number(fee.clone(), Value::from("rate"), Value::Undefined));
                };
                reduced_fees.push(fee.clone());
            };
            order.set("fees".into(), reduced_fees.clone());
            if parse_fee.is_truthy() && reduced_length.clone() == 1.into() {
                order.set("fee".into(), reduced_fees.get(0.into()));
            };
        };
        if amount.clone() == Value::Undefined {
            if filled.clone() != Value::Undefined && remaining.clone() != Value::Undefined {
                amount = Precise::string_add(filled.clone(), remaining.clone());
            } else if self.safe_string(order.clone(), Value::from("status"), Value::Undefined) == Value::from("closed") {
                amount = filled.clone();
            };
        };
        if filled.clone() == Value::Undefined {
            if amount.clone() != Value::Undefined && remaining.clone() != Value::Undefined {
                filled = Precise::string_sub(amount.clone(), remaining.clone());
            };
        };
        if remaining.clone() == Value::Undefined {
            if amount.clone() != Value::Undefined && filled.clone() != Value::Undefined {
                remaining = Precise::string_sub(amount.clone(), filled.clone());
            };
        };
        if average.clone() == Value::Undefined {
            if filled.clone() != Value::Undefined && cost.clone() != Value::Undefined && Precise::string_gt(filled.clone(), Value::from("0")) {
                average = Precise::string_div(cost.clone(), filled.clone(), Value::Undefined);
            };
        };
        let mut cost_price_exists: Value = (average.clone() != Value::Undefined || price.clone() != Value::Undefined).into();
        if parse_cost.is_truthy() && filled.clone() != Value::Undefined && cost_price_exists.is_truthy() {
            let mut multiply_price: Value = Value::Undefined;
            if average.clone() == Value::Undefined {
                multiply_price = price.clone();
            } else {
                multiply_price = average.clone();
            };
            let mut contract_size: Value = self.safe_string(market.clone(), Value::from("contractSize"), Value::Undefined);
            if contract_size.clone() != Value::Undefined {
                let mut inverse: Value = self.safe_value(market.clone(), Value::from("inverse"), false.into());
                if inverse.is_truthy() {
                    multiply_price = Precise::string_div(Value::from("1"), multiply_price.clone(), Value::Undefined);
                };
                multiply_price = Precise::string_mul(multiply_price.clone(), contract_size.clone());
            };
            cost = Precise::string_mul(multiply_price.clone(), filled.clone());
        };
        let mut order_type: Value = self.safe_value(order.clone(), Value::from("type"), Value::Undefined);
        let mut empty_price: Value = (price.clone() == Value::Undefined || Precise::string_equals(price.clone(), Value::from("0"))).into();
        if empty_price.is_truthy() && order_type.clone() == Value::from("market") {
            price = average.clone();
        };
        let mut i: usize = 0;
        while Value::from(i) < trades.len().into() {
            let mut entry: Value = trades.get(i.into());
            entry.set("amount".into(), self.safe_number(entry.clone(), Value::from("amount"), Value::Undefined));
            entry.set("price".into(), self.safe_number(entry.clone(), Value::from("price"), Value::Undefined));
            entry.set("cost".into(), self.safe_number(entry.clone(), Value::from("cost"), Value::Undefined));
            let mut fee: Value = self.safe_value(entry.clone(), Value::from("fee"), Value::new_object());
            fee.set("cost".into(), self.safe_number(fee.clone(), Value::from("cost"), Value::Undefined));
            if fee.contains_key(Value::from("rate")) {
                fee.set("rate".into(), self.safe_number(fee.clone(), Value::from("rate"), Value::Undefined));
            };
            entry.set("fee".into(), fee.clone());
            i += 1;
        };
        let mut time_in_force: Value = self.safe_string(order.clone(), Value::from("timeInForce"), Value::Undefined);
        if time_in_force.clone() == Value::Undefined {
            if self.safe_string(order.clone(), Value::from("type"), Value::Undefined) == Value::from("market") {
                time_in_force = Value::from("IOC");
            };
            if self.safe_value(order.clone(), Value::from("postOnly"), false.into()).is_truthy() {
                time_in_force = Value::from("PO");
            };
        };
        return self.extend_2(order.clone(), Value::Json(json!({
            "lastTradeTimestamp": last_trade_time_timestamp,
            "price": self.parse_number(price.clone(), Value::Undefined),
            "amount": self.parse_number(amount.clone(), Value::Undefined),
            "cost": self.parse_number(cost.clone(), Value::Undefined),
            "average": self.parse_number(average.clone(), Value::Undefined),
            "filled": self.parse_number(filled.clone(), Value::Undefined),
            "remaining": self.parse_number(remaining.clone(), Value::Undefined),
            "timeInForce": time_in_force,
            "trades": trades
        })));
    }

    fn parse_orders(&mut self, mut orders: Value, mut market: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        let mut results: Value = Value::new_array();
        if Array::is_array(orders.clone()).is_truthy() {
            let mut i: usize = 0;
            while Value::from(i) < orders.len().into() {
                let mut order: Value = self.extend_2(self.parse_order(orders.get(i.into()), market.clone()), params.clone());
                results.push(order.clone());
                i += 1;
            };
        } else {
            let mut ids: Value = Object::keys(orders.clone());
            let mut i: usize = 0;
            while Value::from(i) < ids.len().into() {
                let mut id: Value = ids.get(i.into());
                let mut order: Value = self.extend_2(self.parse_order(self.extend_2(Value::Json(json!({
                    "id": id
                })), orders.get(id.clone())), market.clone()), params.clone());
                results.push(order.clone());
                i += 1;
            };
        };
        results = self.sort_by(results.clone(), Value::from("timestamp"), Value::Undefined);
        let mut symbol: Value = if market.clone() != Value::Undefined { market.get(Value::from("symbol")) } else { Value::Undefined };
        let mut tail: Value = (since.clone() == Value::Undefined).into();
        return self.filter_by_symbol_since_limit(results.clone(), symbol.clone(), since.clone(), limit.clone(), tail.clone());
    }

    fn calculate_fee(&mut self, mut symbol: Value, mut r#type: Value, mut side: Value, mut amount: Value, mut price: Value, mut taker_or_maker: Value, mut params: Value) -> Value {
        let mut market: Value = self.get("markets".into()).get(symbol.clone());
        let mut fee_side: Value = self.safe_string(market.clone(), Value::from("feeSide"), Value::from("quote"));
        let mut key: Value = Value::from("quote");
        let mut cost: Value = Value::Undefined;
        if fee_side.clone() == Value::from("quote") {
            cost = amount.clone() * price.clone();
        } else if fee_side.clone() == Value::from("base") {
            cost = amount.clone();
        } else if fee_side.clone() == Value::from("get") {
            cost = amount.clone();
            if side.clone() == Value::from("sell") {
                cost = cost * price.clone();
            } else {
                key = Value::from("base");
            };
        } else if fee_side.clone() == Value::from("give") {
            cost = amount.clone();
            if side.clone() == Value::from("buy") {
                cost = cost * price.clone();
            } else {
                key = Value::from("base");
            };
        };
        let mut rate: Value = market.get(taker_or_maker.clone());
        if cost.clone() != Value::Undefined {
            cost = cost * rate.clone();
        };
        return Value::Json(json!({
            "type": taker_or_maker,
            "currency": market.get(key.clone()),
            "rate": rate,
            "cost": cost
        }));
    }

    fn safe_trade(&mut self, mut trade: Value, mut market: Value) -> Value {
        let mut amount: Value = self.safe_string(trade.clone(), Value::from("amount"), Value::Undefined);
        let mut price: Value = self.safe_string(trade.clone(), Value::from("price"), Value::Undefined);
        let mut cost: Value = self.safe_string(trade.clone(), Value::from("cost"), Value::Undefined);
        if cost.clone() == Value::Undefined {
            let mut contract_size: Value = self.safe_string(market.clone(), Value::from("contractSize"), Value::Undefined);
            let mut multiply_price: Value = price.clone();
            if contract_size.clone() != Value::Undefined {
                let mut inverse: Value = self.safe_value(market.clone(), Value::from("inverse"), false.into());
                if inverse.is_truthy() {
                    multiply_price = Precise::string_div(Value::from("1"), price.clone(), Value::Undefined);
                };
                multiply_price = Precise::string_mul(multiply_price.clone(), contract_size.clone());
            };
            cost = Precise::string_mul(multiply_price.clone(), amount.clone());
        };
        let mut parse_fee: Value = (self.safe_value(trade.clone(), Value::from("fee"), Value::Undefined) == Value::Undefined).into();
        let mut parse_fees: Value = (self.safe_value(trade.clone(), Value::from("fees"), Value::Undefined) == Value::Undefined).into();
        let mut should_parse_fees: Value = (parse_fee.is_truthy() || parse_fees.is_truthy()).into();
        let mut fees: Value = Value::new_array();
        if should_parse_fees.is_truthy() {
            let mut trade_fees: Value = self.safe_value(trade.clone(), Value::from("fees"), Value::Undefined);
            if trade_fees.clone() != Value::Undefined {
                let mut j: usize = 0;
                while Value::from(j) < trade_fees.len().into() {
                    let mut trade_fee: Value = trade_fees.get(j.into());
                    fees.push(self.extend_2(Value::new_object(), trade_fee.clone()));
                    j += 1;
                };
            } else {
                let mut trade_fee: Value = self.safe_value(trade.clone(), Value::from("fee"), Value::Undefined);
                if trade_fee.clone() != Value::Undefined {
                    fees.push(self.extend_2(Value::new_object(), trade_fee.clone()));
                };
            };
        };
        let mut fee: Value = self.safe_value(trade.clone(), Value::from("fee"), Value::Undefined);
        if should_parse_fees.is_truthy() {
            let mut reduced_fees: Value = if self.get("reduce_fees".into()).is_truthy() { self.reduce_fees_by_currency(fees.clone()) } else { fees.clone() };
            let mut reduced_length: Value = reduced_fees.len().into();
            let mut i: usize = 0;
            while Value::from(i) < reduced_length.clone() {
                reduced_fees.get(i.into()).set("cost".into(), self.safe_number(reduced_fees.get(i.into()), Value::from("cost"), Value::Undefined));
                if reduced_fees.get(i.into()).contains_key(Value::from("rate")) {
                    reduced_fees.get(i.into()).set("rate".into(), self.safe_number(reduced_fees.get(i.into()), Value::from("rate"), Value::Undefined));
                };
                i += 1;
            };
            if !parse_fee.is_truthy() && reduced_length.clone() == 0.into() {
                fee.set("cost".into(), self.safe_number(fee.clone(), Value::from("cost"), Value::Undefined));
                if fee.contains_key(Value::from("rate")) {
                    fee.set("rate".into(), self.safe_number(fee.clone(), Value::from("rate"), Value::Undefined));
                };
                reduced_fees.push(fee.clone());
            };
            if parse_fees.is_truthy() {
                trade.set("fees".into(), reduced_fees.clone());
            };
            if parse_fee.is_truthy() && reduced_length.clone() == 1.into() {
                trade.set("fee".into(), reduced_fees.get(0.into()));
            };
            let mut trade_fee: Value = self.safe_value(trade.clone(), Value::from("fee"), Value::Undefined);
            if trade_fee.clone() != Value::Undefined {
                trade_fee.set("cost".into(), self.safe_number(trade_fee.clone(), Value::from("cost"), Value::Undefined));
                if trade_fee.contains_key(Value::from("rate")) {
                    trade_fee.set("rate".into(), self.safe_number(trade_fee.clone(), Value::from("rate"), Value::Undefined));
                };
                trade.set("fee".into(), trade_fee.clone());
            };
        };
        trade.set("amount".into(), self.parse_number(amount.clone(), Value::Undefined));
        trade.set("price".into(), self.parse_number(price.clone(), Value::Undefined));
        trade.set("cost".into(), self.parse_number(cost.clone(), Value::Undefined));
        return trade.clone();
    }

    fn reduce_fees_by_currency(&mut self, mut fees: Value) -> Value {
        let mut reduced: Value = Value::new_object();
        let mut i: usize = 0;
        while Value::from(i) < fees.len().into() {
            let mut fee: Value = fees.get(i.into());
            let mut fee_currency_code: Value = self.safe_string(fee.clone(), Value::from("currency"), Value::Undefined);
            if fee_currency_code.clone() != Value::Undefined {
                let mut rate: Value = self.safe_string(fee.clone(), Value::from("rate"), Value::Undefined);
                let mut cost: Value = self.safe_value(fee.clone(), Value::from("cost"), Value::Undefined);
                if Precise::string_eq(cost.clone(), Value::from("0")) {
                    continue;
                };
                if !reduced.contains_key(fee_currency_code.clone()) {
                    reduced.set(fee_currency_code.clone(), Value::new_object());
                };
                let mut rate_key: Value = if rate.clone() == Value::Undefined { Value::from("") } else { rate.clone() };
                if reduced.get(fee_currency_code.clone()).contains_key(rate_key.clone()) {
                    reduced.get(fee_currency_code.clone()).get(rate_key.clone()).set("cost".into(), Precise::string_add(reduced.get(fee_currency_code.clone()).get(rate_key.clone()).get(Value::from("cost")), cost.clone()));
                } else {
                    reduced.get(fee_currency_code.clone()).set(rate_key.clone(), Value::Json(json!({
                        "currency": fee_currency_code,
                        "cost": cost
                    })));
                    if rate.clone() != Value::Undefined {
                        reduced.get(fee_currency_code.clone()).get(rate_key.clone()).set("rate".into(), rate.clone());
                    };
                };
            };
            i += 1;
        };
        let mut result: Value = Value::new_array();
        let mut fee_values: Value = Object::values(reduced.clone());
        let mut i: usize = 0;
        while Value::from(i) < fee_values.len().into() {
            let mut reduced_fee_values: Value = Object::values(fee_values.get(i.into()));
            result = self.array_concat(result.clone(), reduced_fee_values.clone());
            i += 1;
        };
        return result.clone();
    }

    fn safe_ticker(&self, mut ticker: Value, mut market: Value) -> Value {
        let mut open: Value = self.safe_value(ticker.clone(), Value::from("open"), Value::Undefined);
        let mut close: Value = self.safe_value(ticker.clone(), Value::from("close"), Value::Undefined);
        let mut last: Value = self.safe_value(ticker.clone(), Value::from("last"), Value::Undefined);
        let mut change: Value = self.safe_value(ticker.clone(), Value::from("change"), Value::Undefined);
        let mut percentage: Value = self.safe_value(ticker.clone(), Value::from("percentage"), Value::Undefined);
        let mut average: Value = self.safe_value(ticker.clone(), Value::from("average"), Value::Undefined);
        let mut vwap: Value = self.safe_value(ticker.clone(), Value::from("vwap"), Value::Undefined);
        let mut base_volume: Value = self.safe_value(ticker.clone(), Value::from("baseVolume"), Value::Undefined);
        let mut quote_volume: Value = self.safe_value(ticker.clone(), Value::from("quoteVolume"), Value::Undefined);
        if vwap.clone() == Value::Undefined {
            vwap = Precise::string_div(quote_volume.clone(), base_volume.clone(), Value::Undefined);
        };
        if last.clone() != Value::Undefined && close.clone() == Value::Undefined {
            close = last.clone();
        } else if last.clone() == Value::Undefined && close.clone() != Value::Undefined {
            last = close.clone();
        };
        if last.clone() != Value::Undefined && open.clone() != Value::Undefined {
            if change.clone() == Value::Undefined {
                change = Precise::string_sub(last.clone(), open.clone());
            };
            if average.clone() == Value::Undefined {
                average = Precise::string_div(Precise::string_add(last.clone(), open.clone()), Value::from("2"), Value::Undefined);
            };
        };
        if percentage.clone() == Value::Undefined && change.clone() != Value::Undefined && open.clone() != Value::Undefined && Precise::string_gt(open.clone(), Value::from("0")) {
            percentage = Precise::string_mul(Precise::string_div(change.clone(), open.clone(), Value::Undefined), Value::from("100"));
        };
        if change.clone() == Value::Undefined && percentage.clone() != Value::Undefined && open.clone() != Value::Undefined {
            change = Precise::string_div(Precise::string_mul(percentage.clone(), open.clone()), Value::from("100"), Value::Undefined);
        };
        if open.clone() == Value::Undefined && last.clone() != Value::Undefined && change.clone() != Value::Undefined {
            open = Precise::string_sub(last.clone(), change.clone());
        };
        return self.extend_2(ticker.clone(), Value::Json(json!({
            "bid": self.safe_number(ticker.clone(), Value::from("bid"), Value::Undefined),
            "bidVolume": self.safe_number(ticker.clone(), Value::from("bidVolume"), Value::Undefined),
            "ask": self.safe_number(ticker.clone(), Value::from("ask"), Value::Undefined),
            "askVolume": self.safe_number(ticker.clone(), Value::from("askVolume"), Value::Undefined),
            "high": self.safe_number(ticker.clone(), Value::from("high"), Value::Undefined),
            "low": self.safe_number(ticker.clone(), Value::from("low"), Value::Undefined),
            "open": self.parse_number(open.clone(), Value::Undefined),
            "close": self.parse_number(close.clone(), Value::Undefined),
            "last": self.parse_number(last.clone(), Value::Undefined),
            "change": self.parse_number(change.clone(), Value::Undefined),
            "percentage": self.parse_number(percentage.clone(), Value::Undefined),
            "average": self.parse_number(average.clone(), Value::Undefined),
            "vwap": self.parse_number(vwap.clone(), Value::Undefined),
            "baseVolume": self.parse_number(base_volume.clone(), Value::Undefined),
            "quoteVolume": self.parse_number(quote_volume.clone(), Value::Undefined),
            "previousClose": self.safe_number(ticker.clone(), Value::from("previousClose"), Value::Undefined)
        })));
    }

    async fn fetch_ohlcv(&mut self, mut symbol: Value, mut timeframe: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("fetchTrades")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchOHLCV() is not supported yet"))"###);
        };
        self.load_markets(Value::Undefined, Value::Undefined).await;
        let mut trades: Value = self.fetch_trades(symbol.clone(), since.clone(), limit.clone(), params.clone()).await;
        let mut ohlcvc: Value = self.build_ohlcvc(trades.clone(), timeframe.clone(), since.clone(), limit.clone());
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < ohlcvc.len().into() {
            result.push(Value::Json(serde_json::Value::Array(vec![self.safe_integer(ohlcvc.get(i.into()), 0.into(), Value::Undefined).into(), self.safe_number(ohlcvc.get(i.into()), 1.into(), Value::Undefined).into(), self.safe_number(ohlcvc.get(i.into()), 2.into(), Value::Undefined).into(), self.safe_number(ohlcvc.get(i.into()), 3.into(), Value::Undefined).into(), self.safe_number(ohlcvc.get(i.into()), 4.into(), Value::Undefined).into(), self.safe_number(ohlcvc.get(i.into()), 5.into(), Value::Undefined).into()])));
            i += 1;
        };
        return result.clone();
    }

    fn convert_trading_view_to_ohlcv(&mut self, mut ohlcvs: Value, mut timestamp: Value, mut open: Value, mut high: Value, mut low: Value, mut close: Value, mut volume: Value, mut ms: Value) -> Value {
        let mut result: Value = Value::new_array();
        let mut timestamps: Value = self.safe_value(ohlcvs.clone(), timestamp.clone(), Value::new_array());
        let mut opens: Value = self.safe_value(ohlcvs.clone(), open.clone(), Value::new_array());
        let mut highs: Value = self.safe_value(ohlcvs.clone(), high.clone(), Value::new_array());
        let mut lows: Value = self.safe_value(ohlcvs.clone(), low.clone(), Value::new_array());
        let mut closes: Value = self.safe_value(ohlcvs.clone(), close.clone(), Value::new_array());
        let mut volumes: Value = self.safe_value(ohlcvs.clone(), volume.clone(), Value::new_array());
        let mut i: usize = 0;
        while Value::from(i) < timestamps.len().into() {
            result.push(Value::Json(serde_json::Value::Array(vec![if ms.is_truthy() { self.safe_integer(timestamps.clone(), Value::from(i), Value::Undefined) } else { self.safe_timestamp(timestamps.clone(), Value::from(i), Value::Undefined) }.into(), self.safe_value(opens.clone(), Value::from(i), Value::Undefined).into(), self.safe_value(highs.clone(), Value::from(i), Value::Undefined).into(), self.safe_value(lows.clone(), Value::from(i), Value::Undefined).into(), self.safe_value(closes.clone(), Value::from(i), Value::Undefined).into(), self.safe_value(volumes.clone(), Value::from(i), Value::Undefined).into()])));
            i += 1;
        };
        return result.clone();
    }

    fn convert_ohlcv_to_trading_view(&mut self, mut ohlcvs: Value, mut timestamp: Value, mut open: Value, mut high: Value, mut low: Value, mut close: Value, mut volume: Value, mut ms: Value) -> Value {
        let mut result: Value = Value::new_object();
        result.set(timestamp.clone(), Value::new_array());
        result.set(open.clone(), Value::new_array());
        result.set(high.clone(), Value::new_array());
        result.set(low.clone(), Value::new_array());
        result.set(close.clone(), Value::new_array());
        result.set(volume.clone(), Value::new_array());
        let mut i: usize = 0;
        while Value::from(i) < ohlcvs.len().into() {
            let mut ts: Value = if ms.is_truthy() { ohlcvs.get(i.into()).get(0.into()) } else { parse_int(ohlcvs.get(i.into()).get(0.into()) / 1000.into()) };
            result.get(timestamp.clone()).push(ts.clone());
            result.get(open.clone()).push(ohlcvs.get(i.into()).get(1.into()));
            result.get(high.clone()).push(ohlcvs.get(i.into()).get(2.into()));
            result.get(low.clone()).push(ohlcvs.get(i.into()).get(3.into()));
            result.get(close.clone()).push(ohlcvs.get(i.into()).get(4.into()));
            result.get(volume.clone()).push(ohlcvs.get(i.into()).get(5.into()));
            i += 1;
        };
        return result.clone();
    }

    fn market_ids(&mut self, mut symbols: Value) -> Value {
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < symbols.len().into() {
            result.push(self.market_id(symbols.get(i.into())));
            i += 1;
        };
        return result.clone();
    }

    fn market_symbols(&mut self, mut symbols: Value) -> Value {
        if symbols.clone() == Value::Undefined {
            return symbols.clone();
        };
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < symbols.len().into() {
            result.push(self.symbol(symbols.get(i.into())));
            i += 1;
        };
        return result.clone();
    }

    fn parse_bids_asks(&mut self, mut bidasks: Value, mut price_key: Value, mut amount_key: Value) -> Value {
        bidasks = self.to_array(bidasks.clone());
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < bidasks.len().into() {
            result.push(self.parse_bid_ask(bidasks.get(i.into()), price_key.clone(), amount_key.clone()));
            i += 1;
        };
        return result.clone();
    }

    async fn fetch_l2_order_book(&mut self, mut symbol: Value, mut limit: Value, mut params: Value) -> Value {
        let mut orderbook: Value = self.fetch_order_book(symbol.clone(), limit.clone(), params.clone()).await;
        return self.extend_2(orderbook.clone(), Value::Json(json!({
            "asks": self.sort_by(self.aggregate(orderbook.get(Value::from("asks"))), 0.into(), Value::Undefined),
            "bids": self.sort_by(self.aggregate(orderbook.get(Value::from("bids"))), 0.into(), true.into())
        })));
    }

    fn filter_by_symbol(&mut self, mut objects: Value, mut symbol: Value) -> Value {
        if symbol.clone() == Value::Undefined {
            return objects.clone();
        };
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < objects.len().into() {
            let mut object_symbol: Value = self.safe_string(objects.get(i.into()), Value::from("symbol"), Value::Undefined);
            if object_symbol.clone() == symbol.clone() {
                result.push(objects.get(i.into()));
            };
            i += 1;
        };
        return result.clone();
    }

    fn parse_ohlcv(&mut self, mut ohlcv: Value, mut market: Value) -> Value {
        if Array::is_array(ohlcv.clone()).is_truthy() {
            return Value::Json(serde_json::Value::Array(vec![self.safe_integer(ohlcv.clone(), 0.into(), Value::Undefined).into(), self.safe_number(ohlcv.clone(), 1.into(), Value::Undefined).into(), self.safe_number(ohlcv.clone(), 2.into(), Value::Undefined).into(), self.safe_number(ohlcv.clone(), 3.into(), Value::Undefined).into(), self.safe_number(ohlcv.clone(), 4.into(), Value::Undefined).into(), self.safe_number(ohlcv.clone(), 5.into(), Value::Undefined).into()]));
        };
        return ohlcv.clone();
    }

    fn get_network(&mut self, mut network: Value, mut code: Value) -> Value {
        network = network.to_upper_case();
        let mut aliases: Value = Value::Json(json!({
            "ETHEREUM": "ETH",
            "ETHER": "ETH",
            "ERC20": "ETH",
            "ETH": "ETH",
            "TRC20": "TRX",
            "TRON": "TRX",
            "TRX": "TRX",
            "BEP20": "BSC",
            "BSC": "BSC",
            "HRC20": "HT",
            "HECO": "HT",
            "SPL": "SOL",
            "SOL": "SOL",
            "TERRA": "LUNA",
            "LUNA": "LUNA",
            "POLYGON": "MATIC",
            "MATIC": "MATIC",
            "EOS": "EOS",
            "WAVES": "WAVES",
            "AVALANCHE": "AVAX",
            "AVAX": "AVAX",
            "QTUM": "QTUM",
            "CHZ": "CHZ",
            "NEO": "NEO",
            "ONT": "ONT",
            "RON": "RON"
        }));
        if network.clone() == code.clone() {
            return network.clone();
        } else if aliases.contains_key(network.clone()) {
            return aliases.get(network.clone());
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" network ") + network.clone() + Value::from(" is not yet supported"))"###);
        };
    }

    fn safe_number_2(&self, mut dictionary: Value, mut key1: Value, mut key2: Value, mut d: Value) -> Value {
        let mut value: Value = self.safe_string_2(dictionary.clone(), key1.clone(), key2.clone(), Value::Undefined);
        return self.parse_number(value.clone(), d.clone());
    }

    fn parse_order_book(&mut self, mut orderbook: Value, mut symbol: Value, mut timestamp: Value, mut bids_key: Value, mut asks_key: Value, mut price_key: Value, mut amount_key: Value) -> Value {
        let mut bids: Value = self.parse_bids_asks(self.safe_value(orderbook.clone(), bids_key.clone(), Value::new_array()), price_key.clone(), amount_key.clone());
        let mut asks: Value = self.parse_bids_asks(self.safe_value(orderbook.clone(), asks_key.clone(), Value::new_array()), price_key.clone(), amount_key.clone());
        return Value::Json(json!({
            "symbol": symbol,
            "bids": self.sort_by(bids.clone(), 0.into(), true.into()),
            "asks": self.sort_by(asks.clone(), 0.into(), Value::Undefined),
            "timestamp": timestamp,
            "datetime": self.iso8601(timestamp.clone()),
            "nonce": Value::Undefined
        }));
    }

    fn parse_ohlcvs(&mut self, mut ohlcvs: Value, mut market: Value, mut timeframe: Value, mut since: Value, mut limit: Value) -> Value {
        let mut results: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < ohlcvs.len().into() {
            results.push(self.parse_ohlcv(ohlcvs.get(i.into()), market.clone()));
            i += 1;
        };
        let mut sorted: Value = self.sort_by(results.clone(), 0.into(), Value::Undefined);
        let mut tail: Value = (since.clone() == Value::Undefined).into();
        return self.filter_by_since_limit(sorted.clone(), since.clone(), limit.clone(), 0.into(), tail.clone());
    }

    fn parse_leverage_tiers(&mut self, mut response: Value, mut symbols: Value, mut market_id_key: Value) -> Value {
        symbols = self.market_symbols(symbols.clone());
        let mut tiers: Value = Value::new_object();
        let mut i: usize = 0;
        while Value::from(i) < response.len().into() {
            let mut item: Value = response.get(i.into());
            let mut id: Value = self.safe_string(item.clone(), market_id_key.clone(), Value::Undefined);
            let mut market: Value = self.safe_market(id.clone(), Value::Undefined, Value::Undefined);
            let mut symbol: Value = market.get(Value::from("symbol"));
            let mut contract: Value = self.safe_value(market.clone(), Value::from("contract"), false.into());
            if contract.is_truthy() && symbols.clone() == Value::Undefined || self.in_array(symbol.clone(), symbols.clone()).is_truthy() {
                tiers.set(symbol.clone(), self.parse_market_leverage_tiers(item.clone(), market.clone()));
            };
            i += 1;
        };
        return tiers.clone();
    }

    async fn load_trading_limits(&mut self, mut symbols: Value, mut reload: Value, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchTradingLimits")).is_truthy() {
            if reload.is_truthy() || !self.get("options".into()).contains_key(Value::from("limitsLoaded")) {
                let mut response: Value = self.fetch_trading_limits(symbols.clone(), Value::Undefined).await;
                let mut i: usize = 0;
                while Value::from(i) < symbols.len().into() {
                    let mut symbol: Value = symbols.get(i.into());
                    self.get("markets".into()).set(symbol.clone(), self.deep_extend_2(self.get("markets".into()).get(symbol.clone()), response.get(symbol.clone())));
                    i += 1;
                };
                self.get("options".into()).set("limitsLoaded".into(), self.milliseconds());
            };
        };
        return self.get("markets".into());
    }

    fn parse_positions(&mut self, mut positions: Value, mut symbols: Value, mut params: Value) -> Value {
        symbols = self.market_symbols(symbols.clone());
        positions = self.to_array(positions.clone());
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < positions.len().into() {
            let mut position: Value = self.extend_2(self.parse_position(positions.get(i.into()), Value::Undefined), params.clone());
            result.push(position.clone());
            i += 1;
        };
        return self.filter_by_array(result.clone(), Value::from("symbol"), symbols.clone(), false.into());
    }

    fn parse_accounts(&mut self, mut accounts: Value, mut params: Value) -> Value {
        accounts = self.to_array(accounts.clone());
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < accounts.len().into() {
            let mut account: Value = self.extend_2(self.parse_account(accounts.get(i.into())), params.clone());
            result.push(account.clone());
            i += 1;
        };
        return result.clone();
    }

    fn parse_trades(&mut self, mut trades: Value, mut market: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        trades = self.to_array(trades.clone());
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < trades.len().into() {
            let mut trade: Value = self.extend_2(self.parse_trade(trades.get(i.into()), market.clone()), params.clone());
            result.push(trade.clone());
            i += 1;
        };
        result = self.sort_by_2(result.clone(), Value::from("timestamp"), Value::from("id"), Value::Undefined);
        let mut symbol: Value = if market.clone() != Value::Undefined { market.get(Value::from("symbol")) } else { Value::Undefined };
        let mut tail: Value = (since.clone() == Value::Undefined).into();
        return self.filter_by_symbol_since_limit(result.clone(), symbol.clone(), since.clone(), limit.clone(), tail.clone());
    }

    fn parse_transactions(&mut self, mut transactions: Value, mut currency: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        transactions = self.to_array(transactions.clone());
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < transactions.len().into() {
            let mut transaction: Value = self.extend_2(self.parse_transaction(transactions.get(i.into()), currency.clone()), params.clone());
            result.push(transaction.clone());
            i += 1;
        };
        result = self.sort_by(result.clone(), Value::from("timestamp"), Value::Undefined);
        let mut code: Value = if currency.clone() != Value::Undefined { currency.get(Value::from("code")) } else { Value::Undefined };
        let mut tail: Value = (since.clone() == Value::Undefined).into();
        return self.filter_by_currency_since_limit(result.clone(), code.clone(), since.clone(), limit.clone(), tail.clone());
    }

    fn parse_transfers(&mut self, mut transfers: Value, mut currency: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        transfers = self.to_array(transfers.clone());
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < transfers.len().into() {
            let mut transfer: Value = self.extend_2(self.parse_transfer(transfers.get(i.into()), currency.clone()), params.clone());
            result.push(transfer.clone());
            i += 1;
        };
        result = self.sort_by(result.clone(), Value::from("timestamp"), Value::Undefined);
        let mut code: Value = if currency.clone() != Value::Undefined { currency.get(Value::from("code")) } else { Value::Undefined };
        let mut tail: Value = (since.clone() == Value::Undefined).into();
        return self.filter_by_currency_since_limit(result.clone(), code.clone(), since.clone(), limit.clone(), tail.clone());
    }

    fn parse_ledger(&mut self, mut data: Value, mut currency: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        let mut result: Value = Value::new_array();
        let mut array_data: Value = self.to_array(data.clone());
        let mut i: usize = 0;
        while Value::from(i) < array_data.len().into() {
            let mut item_or_items: Value = self.parse_ledger_entry(array_data.get(i.into()), currency.clone());
            if Array::is_array(item_or_items.clone()).is_truthy() {
                let mut j: usize = 0;
                while Value::from(j) < item_or_items.len().into() {
                    result.push(self.extend_2(item_or_items.get(j.into()), params.clone()));
                    j += 1;
                };
            } else {
                result.push(self.extend_2(item_or_items.clone(), params.clone()));
            };
            i += 1;
        };
        result = self.sort_by(result.clone(), Value::from("timestamp"), Value::Undefined);
        let mut code: Value = if currency.clone() != Value::Undefined { currency.get(Value::from("code")) } else { Value::Undefined };
        let mut tail: Value = (since.clone() == Value::Undefined).into();
        return self.filter_by_currency_since_limit(result.clone(), code.clone(), since.clone(), limit.clone(), tail.clone());
    }

    fn nonce(&mut self) -> Value {
        return self.seconds();
    }

    fn set_headers(&mut self, mut headers: Value) -> Value {
        return headers.clone();
    }

    fn market_id(&mut self, mut symbol: Value) -> Value {
        let mut market: Value = self.market(symbol.clone());
        if market.clone() != Value::Undefined {
            return market.get(Value::from("id"));
        };
        return symbol.clone();
    }

    fn symbol(&mut self, mut symbol: Value) -> Value {
        let mut market: Value = self.market(symbol.clone());
        return self.safe_string(market.clone(), Value::from("symbol"), symbol.clone());
    }

    fn resolve_path(&mut self, mut path: Value, mut params: Value) -> Value {
        return Value::Json(serde_json::Value::Array(vec![self.implode_params(path.clone(), params.clone()).into(), self.omit(params.clone(), self.extract_params(path.clone(), Value::Undefined)).into()]));
    }

    fn filter_by_array(&mut self, mut objects: Value, mut key: Value, mut values: Value, mut indexed: Value) -> Value {
        objects = self.to_array(objects.clone());
        if values.clone() == Value::Undefined || !values.is_truthy() {
            return if indexed.is_truthy() { self.index_by(objects.clone(), key.clone()) } else { objects.clone() };
        };
        let mut results: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < objects.len().into() {
            if self.in_array(objects.get(i.into()).get(key.clone()), values.clone()).is_truthy() {
                results.push(objects.get(i.into()));
            };
            i += 1;
        };
        return if indexed.is_truthy() { self.index_by(results.clone(), key.clone()) } else { results.clone() };
    }

    async fn fetch2(&mut self, mut path: Value, mut api: Value, mut method: Value, mut params: Value, mut headers: Value, mut body: Value, mut config: Value, mut context: Value) -> Value {
        if self.get("enable_rate_limit".into()).is_truthy() {
            let mut cost: Value = self.calculate_rate_limiter_cost(api.clone(), method.clone(), path.clone(), params.clone(), config.clone(), context.clone());
            self.throttle(cost.clone()).await;
        };
        self.set("last_rest_request_timestamp".into(), self.milliseconds());
        let mut request: Value = self.sign(path.clone(), api.clone(), method.clone(), params.clone(), headers.clone(), body.clone());
        return self.fetch(request.get(Value::from("url")), request.get(Value::from("method")), request.get(Value::from("headers")), request.get(Value::from("body"))).await;
    }

    async fn request(&mut self, mut path: Value, mut api: Value, mut method: Value, mut params: Value, mut headers: Value, mut body: Value, mut config: Value, mut context: Value) -> Value {
        return self.fetch2(path.clone(), api.clone(), method.clone(), params.clone(), headers.clone(), body.clone(), config.clone(), context.clone()).await;
    }

    async fn load_accounts(&mut self, mut reload: Value, mut params: Value) -> Value {
        if reload.is_truthy() {
            self.set("accounts".into(), self.fetch_accounts(params.clone()).await);
        } else {
            if self.get("accounts".into()).is_truthy() {
                return self.get("accounts".into());
            } else {
                self.set("accounts".into(), self.fetch_accounts(params.clone()).await);
            };
        };
        self.set("accounts_by_id".into(), self.index_by(self.get("accounts".into()), Value::from("id")));
        return self.get("accounts".into());
    }

    async fn fetch_ohlcvc(&mut self, mut symbol: Value, mut timeframe: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("fetchTrades")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchOHLCV() is not supported yet"))"###);
        };
        self.load_markets(Value::Undefined, Value::Undefined).await;
        let mut trades: Value = self.fetch_trades(symbol.clone(), since.clone(), limit.clone(), params.clone()).await;
        return self.build_ohlcvc(trades.clone(), timeframe.clone(), since.clone(), limit.clone());
    }

    fn parse_trading_view_ohlcv(&mut self, mut ohlcvs: Value, mut market: Value, mut timeframe: Value, mut since: Value, mut limit: Value) -> Value {
        let mut result: Value = self.convert_trading_view_to_ohlcv(ohlcvs.clone(), Value::Undefined, Value::Undefined, Value::Undefined, Value::Undefined, Value::Undefined, Value::Undefined, Value::Undefined);
        return self.parse_ohlcvs(result.clone(), market.clone(), timeframe.clone(), since.clone(), limit.clone());
    }

    async fn edit_limit_buy_order(&mut self, mut id: Value, mut symbol: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        return self.edit_limit_order(id.clone(), symbol.clone(), Value::from("buy"), amount.clone(), price.clone(), params.clone()).await;
    }

    async fn edit_limit_sell_order(&mut self, mut id: Value, mut symbol: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        return self.edit_limit_order(id.clone(), symbol.clone(), Value::from("sell"), amount.clone(), price.clone(), params.clone()).await;
    }

    async fn edit_limit_order(&mut self, mut id: Value, mut symbol: Value, mut side: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        return self.edit_order(id.clone(), symbol.clone(), Value::from("limit"), side.clone(), amount.clone(), price.clone(), params.clone()).await;
    }

    async fn edit_order(&mut self, mut id: Value, mut symbol: Value, mut r#type: Value, mut side: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        self.cancel_order(id.clone(), symbol.clone(), Value::Undefined).await;
        return self.create_order(symbol.clone(), r#type.clone(), side.clone(), amount.clone(), price.clone(), params.clone()).await;
    }

    async fn fetch_permissions(&mut self, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchPermissions() is not supported yet"))"###);
    }

    async fn fetch_bids_asks(&mut self, mut symbols: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchBidsAsks() is not supported yet"))"###);
    }

    fn parse_bid_ask(&mut self, mut bidask: Value, mut price_key: Value, mut amount_key: Value) -> Value {
        let mut price: Value = self.safe_number(bidask.clone(), price_key.clone(), Value::Undefined);
        let mut amount: Value = self.safe_number(bidask.clone(), amount_key.clone(), Value::Undefined);
        return Value::Json(serde_json::Value::Array(vec![price.clone().into(), amount.clone().into()]));
    }

    fn safe_currency(&self, mut currency_id: Value, mut currency: Value) -> Value {
        if currency_id.clone() == Value::Undefined && currency.clone() != Value::Undefined {
            return currency.clone();
        };
        if self.get("currencies_by_id".into()) != Value::Undefined && self.get("currencies_by_id".into()).contains_key(currency_id.clone()) {
            return self.get("currencies_by_id".into()).get(currency_id.clone());
        };
        let mut code: Value = currency_id.clone();
        if currency_id.clone() != Value::Undefined {
            code = self.common_currency_code(currency_id.to_upper_case());
        };
        return Value::Json(json!({
            "id": currency_id,
            "code": code
        }));
    }

    fn safe_market(&self, mut market_id: Value, mut market: Value, mut delimiter: Value) -> Value {
        let mut result: Value = Value::Json(json!({
            "id": market_id,
            "symbol": market_id,
            "base": Value::Undefined,
            "quote": Value::Undefined,
            "baseId": Value::Undefined,
            "quoteId": Value::Undefined,
            "active": Value::Undefined,
            "type": Value::Undefined,
            "linear": Value::Undefined,
            "inverse": Value::Undefined,
            "spot": false,
            "swap": false,
            "future": false,
            "option": false,
            "margin": false,
            "contract": false,
            "contractSize": Value::Undefined,
            "expiry": Value::Undefined,
            "expiryDatetime": Value::Undefined,
            "optionType": Value::Undefined,
            "strike": Value::Undefined,
            "settle": Value::Undefined,
            "settleId": Value::Undefined,
            "precision": Value::Json(json!({
            "amount": Value::Undefined,
            "price": Value::Undefined
        })),
            "limits": Value::Json(json!({
            "amount": Value::Json(json!({
            "min": Value::Undefined,
            "max": Value::Undefined
        })),
            "price": Value::Json(json!({
            "min": Value::Undefined,
            "max": Value::Undefined
        })),
            "cost": Value::Json(json!({
            "min": Value::Undefined,
            "max": Value::Undefined
        }))
        })),
            "info": Value::Undefined
        }));
        if market_id.clone() != Value::Undefined {
            if self.get("markets_by_id".into()) != Value::Undefined && self.get("markets_by_id".into()).contains_key(market_id.clone()) {
                market = self.get("markets_by_id".into()).get(market_id.clone());
            } else if delimiter.clone() != Value::Undefined {
                let mut parts: Value = market_id.split(delimiter.clone());
                let mut parts_length: Value = parts.len().into();
                if parts_length.clone() == 2.into() {
                    result.set("baseId".into(), self.safe_string(parts.clone(), 0.into(), Value::Undefined));
                    result.set("quoteId".into(), self.safe_string(parts.clone(), 1.into(), Value::Undefined));
                    result.set("base".into(), self.safe_currency_code(result.get(Value::from("baseId")), Value::Undefined));
                    result.set("quote".into(), self.safe_currency_code(result.get(Value::from("quoteId")), Value::Undefined));
                    result.set("symbol".into(), result.get(Value::from("base")) + Value::from("/") + result.get(Value::from("quote")));
                    return result.clone();
                } else {
                    return result.clone();
                };
            };
        };
        if market.clone() != Value::Undefined {
            return market.clone();
        };
        return result.clone();
    }

    fn check_required_credentials(&mut self, mut error: Value) -> Value {
        let mut keys: Value = Object::keys(self.get("required_credentials".into()));
        let mut i: usize = 0;
        while Value::from(i) < keys.len().into() {
            let mut key: Value = keys.get(i.into());
            if self.get("required_credentials".into()).get(key.clone()).is_truthy() && !self.get("key".into()).is_truthy() {
                if error.is_truthy() {
                    panic!(r###"AuthenticationError::new(self.get("id".into()) + Value::from(r#" requires ""#) + key.clone() + Value::from(r#"" credential"#))"###);
                } else {
                    return error.clone();
                };
            };
            i += 1;
        };
        return true.into();
    }

    fn oath(&mut self) -> Value {
        if self.get("twofa".into()) != Value::Undefined {
            return self.totp(self.get("twofa".into()));
        } else {
            panic!(r###"ExchangeError::new(self.get("id".into()) + Value::from(" exchange.twofa has not been set for 2FA Two-Factor Authentication"))"###);
        };
    }

    async fn fetch_balance(&mut self, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchBalance() is not supported yet"))"###);
    }

    async fn fetch_partial_balance(&mut self, mut part: Value, mut params: Value) -> Value {
        let mut balance: Value = self.fetch_balance(params.clone()).await;
        return balance.get(part.clone());
    }

    async fn fetch_free_balance(&mut self, mut params: Value) -> Value {
        return self.fetch_partial_balance(Value::from("free"), params.clone()).await;
    }

    async fn fetch_used_balance(&mut self, mut params: Value) -> Value {
        return self.fetch_partial_balance(Value::from("used"), params.clone()).await;
    }

    async fn fetch_total_balance(&mut self, mut params: Value) -> Value {
        return self.fetch_partial_balance(Value::from("total"), params.clone()).await;
    }

    async fn fetch_status(&mut self, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchTime")).is_truthy() {
            let mut time: Value = self.fetch_time(params.clone()).await;
            self.set("status".into(), self.extend_2(self.get("status".into()), Value::Json(json!({
                "updated": time
            }))));
        };
        return self.get("status".into());
    }

    async fn fetch_funding_fee(&mut self, mut code: Value, mut params: Value) -> Value {
        let mut warn_on_fetch_funding_fee: Value = self.safe_value(self.get("options".into()), Value::from("warnOnFetchFundingFee"), true.into());
        if warn_on_fetch_funding_fee.is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(r#" fetchFundingFee() method is deprecated, it will be removed in July 2022, please, use fetchTransactionFee() or set exchange.options["warnOnFetchFundingFee"] = false to suppress this warning"#))"###);
        };
        return self.fetch_transaction_fee(code.clone(), params.clone()).await;
    }

    async fn fetch_funding_fees(&mut self, mut codes: Value, mut params: Value) -> Value {
        let mut warn_on_fetch_funding_fees: Value = self.safe_value(self.get("options".into()), Value::from("warnOnFetchFundingFees"), true.into());
        if warn_on_fetch_funding_fees.is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(r#" fetchFundingFees() method is deprecated, it will be removed in July 2022. Please, use fetchTransactionFees() or set exchange.options["warnOnFetchFundingFees"] = false to suppress this warning"#))"###);
        };
        return self.fetch_transaction_fees(codes.clone(), params.clone()).await;
    }

    async fn fetch_transaction_fee(&mut self, mut code: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("fetchTransactionFees")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchTransactionFee() is not supported yet"))"###);
        };
        return self.fetch_transaction_fees(Value::Json(serde_json::Value::Array(vec![code.clone().into()])), params.clone()).await;
    }

    async fn fetch_transaction_fees(&mut self, mut codes: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchTransactionFees() is not supported yet"))"###);
    }

    fn get_supported_mapping(&mut self, mut key: Value, mut mapping: Value) -> Value {
        if mapping.contains_key(key.clone()) {
            return mapping.get(key.clone());
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" ") + key.clone() + Value::from(" does not have a value in mapping"))"###);
        };
    }

    async fn fetch_borrow_rate(&mut self, mut code: Value, mut params: Value) -> Value {
        self.load_markets(Value::Undefined, Value::Undefined).await;
        if !self.get("has".into()).get(Value::from("fetchBorrowRates")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchBorrowRate() is not supported yet"))"###);
        };
        let mut borrow_rates: Value = self.fetch_borrow_rates(params.clone()).await;
        let mut rate: Value = self.safe_value(borrow_rates.clone(), code.clone(), Value::Undefined);
        if rate.clone() == Value::Undefined {
            panic!(r###"ExchangeError::new(self.get("id".into()) + Value::from(" fetchBorrowRate() could not find the borrow rate for currency code ") + code.clone())"###);
        };
        return rate.clone();
    }

    fn handle_market_type_and_params(&mut self, mut method_name: Value, mut market: Value, mut params: Value) -> Value {
        let mut default_type: Value = self.safe_string_2(self.get("options".into()), Value::from("defaultType"), Value::from("type"), Value::from("spot"));
        let mut method_options: Value = self.safe_value(self.get("options".into()), method_name.clone(), Value::Undefined);
        let mut method_type: Value = default_type.clone();
        if method_options.clone() != Value::Undefined {
            if method_options.typeof_() == Value::from("string") {
                method_type = method_options.clone();
            } else {
                method_type = self.safe_string_2(method_options.clone(), Value::from("defaultType"), Value::from("type"), method_type.clone());
            };
        };
        let mut market_type: Value = if market.clone() == Value::Undefined { method_type.clone() } else { market.get(Value::from("type")) };
        let mut r#type: Value = self.safe_string_2(params.clone(), Value::from("defaultType"), Value::from("type"), market_type.clone());
        params = self.omit(params.clone(), Value::Json(serde_json::Value::Array(vec![Value::from("defaultType").into(), Value::from("type").into()])));
        return Value::Json(serde_json::Value::Array(vec![r#type.clone().into(), params.clone().into()]));
    }

    fn handle_sub_type_and_params(&mut self, mut method_name: Value, mut market: Value, mut params: Value) -> Value {
        let mut sub_type: Value = Value::Undefined;
        let mut sub_type_in_params: Value = self.safe_string_2(params.clone(), Value::from("subType"), Value::from("subType"), Value::Undefined);
        if sub_type_in_params.clone() != Value::Undefined {
            sub_type = sub_type_in_params.clone();
            params = self.omit(params.clone(), Value::Json(serde_json::Value::Array(vec![Value::from("defaultSubType").into(), Value::from("subType").into()])));
        } else {
            if market.clone() != Value::Undefined {
                if market.get(Value::from("linear")).is_truthy() {
                    sub_type = Value::from("linear");
                } else if market.get(Value::from("inverse")).is_truthy() {
                    sub_type = Value::from("inverse");
                };
            };
            if sub_type.clone() == Value::Undefined {
                let mut exchange_wide_value: Value = self.safe_string_2(self.get("options".into()), Value::from("defaultSubType"), Value::from("subType"), Value::from("linear"));
                let mut method_options: Value = self.safe_value(self.get("options".into()), method_name.clone(), Value::new_object());
                sub_type = self.safe_string_2(method_options.clone(), Value::from("defaultSubType"), Value::from("subType"), exchange_wide_value.clone());
            };
        };
        return Value::Json(serde_json::Value::Array(vec![sub_type.clone().into(), params.clone().into()]));
    }

    fn throw_exactly_matched_exception(&mut self, mut exact: Value, mut string: Value, mut message: Value) -> () {
        if exact.contains_key(string.clone()) {
            panic!(r###"exact.get(string.clone())::new(message)"###);
        };
    }

    fn throw_broadly_matched_exception(&mut self, mut broad: Value, mut string: Value, mut message: Value) -> () {
        let mut broad_key: Value = self.find_broadly_matched_key(broad.clone(), string.clone());
        if broad_key.clone() != Value::Undefined {
            panic!(r###"broad.get(broad_key.clone())::new(message)"###);
        };
    }

    fn find_broadly_matched_key(&mut self, mut broad: Value, mut string: Value) -> Value {
        let mut keys: Value = Object::keys(broad.clone());
        let mut i: usize = 0;
        while Value::from(i) < keys.len().into() {
            let mut key: Value = keys.get(i.into());
            if string.index_of(key.clone()) >= 0.into() {
                return key.clone();
            };
            i += 1;
        };
        return Value::Undefined;
    }

    fn handle_errors(&mut self, mut status_code: Value, mut status_text: Value, mut url: Value, mut method: Value, mut response_headers: Value, mut response_body: Value, mut response: Value, mut request_headers: Value, mut request_body: Value) -> Value { Value::Undefined }

    fn calculate_rate_limiter_cost(&mut self, mut api: Value, mut method: Value, mut path: Value, mut params: Value, mut config: Value, mut context: Value) -> Value {
        return self.safe_value(config.clone(), Value::from("cost"), 1.into());
    }

    async fn fetch_ticker(&mut self, mut symbol: Value, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchTickers")).is_truthy() {
            let mut tickers: Value = self.fetch_tickers(Value::Json(serde_json::Value::Array(vec![symbol.clone().into()])), params.clone()).await;
            let mut ticker: Value = self.safe_value(tickers.clone(), symbol.clone(), Value::Undefined);
            if ticker.clone() == Value::Undefined {
                panic!(r###"NullResponse::new(self.get("id".into()) + Value::from(" fetchTickers() could not find a ticker for ") + symbol.clone())"###);
            } else {
                return ticker.clone();
            };
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchTicker() is not supported yet"))"###);
        };
    }

    async fn fetch_tickers(&mut self, mut symbols: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchTickers() is not supported yet"))"###);
    }

    async fn fetch_order(&mut self, mut id: Value, mut symbol: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchOrder() is not supported yet"))"###);
    }

    async fn fetch_order_status(&mut self, mut id: Value, mut symbol: Value, mut params: Value) -> Value {
        let mut order: Value = self.fetch_order(id.clone(), symbol.clone(), params.clone()).await;
        return order.get(Value::from("status"));
    }

    async fn fetch_unified_order(&mut self, mut order: Value, mut params: Value) -> Value {
        return self.fetch_order(self.safe_value(order.clone(), Value::from("id"), Value::Undefined), self.safe_value(order.clone(), Value::from("symbol"), Value::Undefined), params.clone()).await;
    }

    async fn create_order(&mut self, mut symbol: Value, mut r#type: Value, mut side: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" createOrder() is not supported yet"))"###);
    }

    async fn cancel_order(&mut self, mut id: Value, mut symbol: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" cancelOrder() is not supported yet"))"###);
    }

    async fn cancel_unified_order(&mut self, mut order: Value, mut params: Value) -> Value {
        return self.cancel_order(self.safe_value(order.clone(), Value::from("id"), Value::Undefined), self.safe_value(order.clone(), Value::from("symbol"), Value::Undefined), params.clone()).await;
    }

    async fn fetch_orders(&mut self, mut symbol: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchOrders() is not supported yet"))"###);
    }

    async fn fetch_open_orders(&mut self, mut symbol: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchOpenOrders() is not supported yet"))"###);
    }

    async fn fetch_closed_orders(&mut self, mut symbol: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchClosedOrders() is not supported yet"))"###);
    }

    async fn fetch_my_trades(&mut self, mut symbol: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchMyTrades() is not supported yet"))"###);
    }

    async fn fetch_transactions(&mut self, mut symbol: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchTransactions() is not supported yet"))"###);
    }

    async fn fetch_deposits(&mut self, mut symbol: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchDeposits() is not supported yet"))"###);
    }

    async fn fetch_withdrawals(&mut self, mut symbol: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchWithdrawals() is not supported yet"))"###);
    }

    async fn fetch_deposit_address(&mut self, mut code: Value, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchDepositAddresses")).is_truthy() {
            let mut deposit_addresses: Value = self.fetch_deposit_addresses(Value::Json(serde_json::Value::Array(vec![code.clone().into()])), params.clone()).await;
            let mut deposit_address: Value = self.safe_value(deposit_addresses.clone(), code.clone(), Value::Undefined);
            if deposit_address.clone() == Value::Undefined {
                panic!(r###"InvalidAddress::new(self.get("id".into()) + Value::from(" fetchDepositAddress() could not find a deposit address for ") + code.clone() + Value::from(", make sure you have created a corresponding deposit address in your wallet on the exchange website"))"###);
            } else {
                return deposit_address.clone();
            };
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchDepositAddress() is not supported yet"))"###);
        };
    }

    fn account(&mut self) -> Value {
        return Value::Json(json!({
            "free": Value::Undefined,
            "used": Value::Undefined,
            "total": Value::Undefined
        }));
    }

    fn common_currency_code(&self, mut currency: Value) -> Value {
        if !self.get("substitute_common_currency_codes".into()).is_truthy() {
            return currency.clone();
        };
        return self.safe_string(self.get("common_currencies".into()), currency.clone(), currency.clone());
    }

    fn currency(&mut self, mut code: Value) -> Value {
        if self.get("currencies".into()) == Value::Undefined {
            panic!(r###"ExchangeError::new(self.get("id".into()) + Value::from(" currencies not loaded"))"###);
        };
        if code.typeof_() == Value::from("string") {
            if self.get("currencies".into()).contains_key(code.clone()) {
                return self.get("currencies".into()).get(code.clone());
            } else if self.get("currencies_by_id".into()).contains_key(code.clone()) {
                return self.get("currencies_by_id".into()).get(code.clone());
            };
        };
        panic!(r###"ExchangeError::new(self.get("id".into()) + Value::from(" does not have currency code ") + code.clone())"###);
    }

    fn market(&self, mut symbol: Value) -> Value {
        if self.get("markets".into()) == Value::Undefined {
            panic!(r###"ExchangeError::new(self.get("id".into()) + Value::from(" markets not loaded"))"###);
        };
        if self.get("markets_by_id".into()) == Value::Undefined {
            panic!(r###"ExchangeError::new(self.get("id".into()) + Value::from(" markets not loaded"))"###);
        };
        if symbol.typeof_() == Value::from("string") {
            if self.get("markets".into()).contains_key(symbol.clone()) {
                return self.get("markets".into()).get(symbol.clone());
            } else if self.get("markets_by_id".into()).contains_key(symbol.clone()) {
                return self.get("markets_by_id".into()).get(symbol.clone());
            };
        };
        panic!(r###"BadSymbol::new(self.get("id".into()) + Value::from(" does not have market symbol ") + symbol.clone())"###);
    }

    fn handle_withdraw_tag_and_params(&mut self, mut tag: Value, mut params: Value) -> Value {
        if tag.typeof_() == Value::from("object") {
            params = self.extend_2(tag.clone(), params.clone());
            tag = Value::Undefined;
        };
        if tag.clone() == Value::Undefined {
            tag = self.safe_string(params.clone(), Value::from("tag"), Value::Undefined);
            if tag.clone() != Value::Undefined {
                params = self.omit(params.clone(), Value::from("tag"));
            };
        };
        return Value::Json(serde_json::Value::Array(vec![tag.clone().into(), params.clone().into()]));
    }

    async fn create_limit_order(&mut self, mut symbol: Value, mut side: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        return self.create_order(symbol.clone(), Value::from("limit"), side.clone(), amount.clone(), price.clone(), params.clone()).await;
    }

    async fn create_market_order(&mut self, mut symbol: Value, mut side: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        return self.create_order(symbol.clone(), Value::from("market"), side.clone(), amount.clone(), price.clone(), params.clone()).await;
    }

    async fn create_limit_buy_order(&mut self, mut symbol: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        return self.create_order(symbol.clone(), Value::from("limit"), Value::from("buy"), amount.clone(), price.clone(), params.clone()).await;
    }

    async fn create_limit_sell_order(&mut self, mut symbol: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        return self.create_order(symbol.clone(), Value::from("limit"), Value::from("sell"), amount.clone(), price.clone(), params.clone()).await;
    }

    async fn create_market_buy_order(&mut self, mut symbol: Value, mut amount: Value, mut params: Value) -> Value {
        return self.create_order(symbol.clone(), Value::from("market"), Value::from("buy"), amount.clone(), Value::Undefined, params.clone()).await;
    }

    async fn create_market_sell_order(&mut self, mut symbol: Value, mut amount: Value, mut params: Value) -> Value {
        return self.create_order(symbol.clone(), Value::from("market"), Value::from("sell"), amount.clone(), Value::Undefined, params.clone()).await;
    }

    fn cost_to_precision(&mut self, mut symbol: Value, mut cost: Value) -> Value {
        let mut market: Value = self.market(symbol.clone());
        return self.decimal_to_precision(cost.clone(), TRUNCATE.into(), market.get(Value::from("precision")).get(Value::from("price")), self.get("precision_mode".into()), self.get("padding_mode".into()));
    }

    fn price_to_precision(&mut self, mut symbol: Value, mut price: Value) -> Value {
        let mut market: Value = self.market(symbol.clone());
        return self.decimal_to_precision(price.clone(), ROUND.into(), market.get(Value::from("precision")).get(Value::from("price")), self.get("precision_mode".into()), self.get("padding_mode".into()));
    }

    fn amount_to_precision(&mut self, mut symbol: Value, mut amount: Value) -> Value {
        let mut market: Value = self.market(symbol.clone());
        return self.decimal_to_precision(amount.clone(), TRUNCATE.into(), market.get(Value::from("precision")).get(Value::from("amount")), self.get("precision_mode".into()), self.get("padding_mode".into()));
    }

    fn fee_to_precision(&mut self, mut symbol: Value, mut fee: Value) -> Value {
        let mut market: Value = self.market(symbol.clone());
        return self.decimal_to_precision(fee.clone(), ROUND.into(), market.get(Value::from("precision")).get(Value::from("price")), self.get("precision_mode".into()), self.get("padding_mode".into()));
    }

    fn currency_to_precision(&mut self, mut code: Value, mut fee: Value, mut network_code: Value) -> Value {
        let mut currency: Value = self.get("currencies".into()).get(code.clone());
        let mut precision: Value = self.safe_value(currency.clone(), Value::from("precision"), Value::Undefined);
        if network_code.clone() != Value::Undefined {
            let mut networks: Value = self.safe_value(currency.clone(), Value::from("networks"), Value::new_object());
            let mut network_item: Value = self.safe_value(networks.clone(), network_code.clone(), Value::new_object());
            precision = self.safe_value(network_item.clone(), Value::from("precision"), precision.clone());
        };
        if precision.clone() == Value::Undefined {
            return fee.clone();
        } else {
            return self.decimal_to_precision(fee.clone(), ROUND.into(), precision.clone(), self.get("precision_mode".into()), self.get("padding_mode".into()));
        };
    }

    fn safe_number(&self, mut object: Value, mut key: Value, mut d: Value) -> Value {
        let mut value: Value = self.safe_string(object.clone(), key.clone(), Value::Undefined);
        return self.parse_number(value.clone(), d.clone());
    }

    fn safe_number_n(&self, mut object: Value, mut arr: Value, mut d: Value) -> Value {
        let mut value: Value = self.safe_string_n(object.clone(), arr.clone(), Value::Undefined);
        return self.parse_number(value.clone(), d.clone());
    }

    fn parse_precision(&mut self, mut precision: Value) -> Value {
        if precision.clone() == Value::Undefined {
            return Value::Undefined;
        };
        return Value::from("1e") + Precise::string_neg(precision.clone());
    }

    async fn load_time_difference(&mut self, mut params: Value) -> Value {
        let mut server_time: Value = self.fetch_time(params.clone()).await;
        let mut after: Value = self.milliseconds();
        self.get("options".into()).set("timeDifference".into(), after.clone() - server_time.clone());
        return self.get("options".into()).get(Value::from("timeDifference"));
    }

    fn implode_hostname(&mut self, mut url: Value) -> Value {
        return self.implode_params(url.clone(), Value::Json(json!({
            "hostname": self.get("hostname".into())
        })));
    }

    async fn fetch_market_leverage_tiers(&mut self, mut symbol: Value, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchLeverageTiers")).is_truthy() {
            let mut market: Value = self.market(symbol.clone());
            if !market.get(Value::from("contract")).is_truthy() {
                panic!(r###"BadSymbol::new(self.get("id".into()) + Value::from(" fetchMarketLeverageTiers() supports contract markets only"))"###);
            };
            let mut tiers: Value = self.fetch_leverage_tiers(Value::Json(serde_json::Value::Array(vec![symbol.clone().into()])), Value::Undefined).await;
            return self.safe_value(tiers.clone(), symbol.clone(), Value::Undefined);
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchMarketLeverageTiers() is not supported yet"))"###);
        };
    }

    async fn create_post_only_order(&mut self, mut symbol: Value, mut r#type: Value, mut side: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("createPostOnlyOrder")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from("createPostOnlyOrder() is not supported yet"))"###);
        };
        let mut query: Value = self.extend_2(params.clone(), Value::Json(json!({
            "postOnly": true
        })));
        return self.create_order(symbol.clone(), r#type.clone(), side.clone(), amount.clone(), price.clone(), query.clone()).await;
    }

    async fn create_reduce_only_order(&mut self, mut symbol: Value, mut r#type: Value, mut side: Value, mut amount: Value, mut price: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("createReduceOnlyOrder")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from("createReduceOnlyOrder() is not supported yet"))"###);
        };
        let mut query: Value = self.extend_2(params.clone(), Value::Json(json!({
            "reduceOnly": true
        })));
        return self.create_order(symbol.clone(), r#type.clone(), side.clone(), amount.clone(), price.clone(), query.clone()).await;
    }

    async fn create_stop_order(&mut self, mut symbol: Value, mut r#type: Value, mut side: Value, mut amount: Value, mut price: Value, mut stop_price: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("createStopOrder")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" createStopOrder() is not supported yet"))"###);
        };
        if stop_price.clone() == Value::Undefined {
            panic!(r###"ArgumentsRequired::new(self.get("id".into()) + Value::from(" create_stop_order() requires a stopPrice argument"))"###);
        };
        let mut query: Value = self.extend_2(params.clone(), Value::Json(json!({
            "stopPrice": stop_price
        })));
        return self.create_order(symbol.clone(), r#type.clone(), side.clone(), amount.clone(), price.clone(), query.clone()).await;
    }

    async fn create_stop_limit_order(&mut self, mut symbol: Value, mut side: Value, mut amount: Value, mut price: Value, mut stop_price: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("createStopLimitOrder")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" createStopLimitOrder() is not supported yet"))"###);
        };
        let mut query: Value = self.extend_2(params.clone(), Value::Json(json!({
            "stopPrice": stop_price
        })));
        return self.create_order(symbol.clone(), Value::from("limit"), side.clone(), amount.clone(), price.clone(), query.clone()).await;
    }

    async fn create_stop_market_order(&mut self, mut symbol: Value, mut side: Value, mut amount: Value, mut stop_price: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("createStopMarketOrder")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" createStopMarketOrder() is not supported yet"))"###);
        };
        let mut query: Value = self.extend_2(params.clone(), Value::Json(json!({
            "stopPrice": stop_price
        })));
        return self.create_order(symbol.clone(), Value::from("market"), side.clone(), amount.clone(), Value::Undefined, query.clone()).await;
    }

    fn safe_currency_code(&self, mut currency_id: Value, mut currency: Value) -> Value {
        currency = self.safe_currency(currency_id.clone(), currency.clone());
        return currency.get(Value::from("code"));
    }

    fn filter_by_symbol_since_limit(&mut self, mut array: Value, mut symbol: Value, mut since: Value, mut limit: Value, mut tail: Value) -> Value {
        return self.filter_by_value_since_limit(array.clone(), Value::from("symbol"), symbol.clone(), since.clone(), limit.clone(), Value::from("timestamp"), tail.clone());
    }

    fn filter_by_currency_since_limit(&mut self, mut array: Value, mut code: Value, mut since: Value, mut limit: Value, mut tail: Value) -> Value {
        return self.filter_by_value_since_limit(array.clone(), Value::from("currency"), code.clone(), since.clone(), limit.clone(), Value::from("timestamp"), tail.clone());
    }

    fn parse_tickers(&mut self, mut tickers: Value, mut symbols: Value, mut params: Value) -> Value {
        let mut results: Value = Value::new_array();
        if Array::is_array(tickers.clone()).is_truthy() {
            let mut i: usize = 0;
            while Value::from(i) < tickers.len().into() {
                let mut ticker: Value = self.extend_2(self.parse_ticker(tickers.get(i.into()), Value::Undefined), params.clone());
                results.push(ticker.clone());
                i += 1;
            };
        } else {
            let mut market_ids: Value = Object::keys(tickers.clone());
            let mut i: usize = 0;
            while Value::from(i) < market_ids.len().into() {
                let mut market_id: Value = market_ids.get(i.into());
                let mut market: Value = self.safe_market(market_id.clone(), Value::Undefined, Value::Undefined);
                let mut ticker: Value = self.extend_2(self.parse_ticker(tickers.get(market_id.clone()), market.clone()), params.clone());
                results.push(ticker.clone());
                i += 1;
            };
        };
        symbols = self.market_symbols(symbols.clone());
        return self.filter_by_array(results.clone(), Value::from("symbol"), symbols.clone(), Value::Undefined);
    }

    fn parse_deposit_addresses(&mut self, mut addresses: Value, mut codes: Value, mut indexed: Value, mut params: Value) -> Value {
        let mut result: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < addresses.len().into() {
            let mut address: Value = self.extend_2(self.parse_deposit_address(addresses.get(i.into()), Value::Undefined), params.clone());
            result.push(address.clone());
            i += 1;
        };
        if codes.clone() != Value::Undefined {
            result = self.filter_by_array(result.clone(), Value::from("currency"), codes.clone(), false.into());
        };
        result = if indexed.is_truthy() { self.index_by(result.clone(), Value::from("currency")) } else { result.clone() };
        return result.clone();
    }

    fn parse_borrow_interests(&mut self, mut response: Value, mut market: Value) -> Value {
        let mut interests: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < response.len().into() {
            let mut row: Value = response.get(i.into());
            interests.push(self.parse_borrow_interest(row.clone(), market.clone()));
            i += 1;
        };
        return interests.clone();
    }

    fn parse_funding_rate_histories(&mut self, mut response: Value, mut market: Value, mut since: Value, mut limit: Value) -> Value {
        let mut rates: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < response.len().into() {
            let mut entry: Value = response.get(i.into());
            rates.push(self.parse_funding_rate_history(entry.clone(), market.clone()));
            i += 1;
        };
        let mut sorted: Value = self.sort_by(rates.clone(), Value::from("timestamp"), Value::Undefined);
        let mut symbol: Value = if market.clone() == Value::Undefined { Value::Undefined } else { market.get(Value::from("symbol")) };
        return self.filter_by_symbol_since_limit(sorted.clone(), symbol.clone(), since.clone(), limit.clone(), Value::Undefined);
    }

    fn safe_symbol(&self, mut market_id: Value, mut market: Value, mut delimiter: Value) -> Value {
        market = self.safe_market(market_id.clone(), market.clone(), delimiter.clone());
        return market.get(Value::from("symbol"));
    }

    fn parse_funding_rate(&mut self, mut contract: Value, mut market: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" parseFundingRate() is not supported yet"))"###);
    }

    fn parse_funding_rates(&mut self, mut response: Value, mut market: Value) -> Value {
        let mut result: Value = Value::new_object();
        let mut i: usize = 0;
        while Value::from(i) < response.len().into() {
            let mut parsed: Value = self.parse_funding_rate(response.get(i.into()), market.clone());
            result.set(parsed.get(Value::from("symbol")), parsed.clone());
            i += 1;
        };
        return result.clone();
    }

    fn is_post_only(&mut self, mut is_market_order: Value, mut exchange_specific_param: Value, mut params: Value) -> Value {
        let mut time_in_force: Value = self.safe_string_upper(params.clone(), Value::from("timeInForce"), Value::Undefined);
        let mut post_only: Value = self.safe_value_2(params.clone(), Value::from("postOnly"), Value::from("post_only"), false.into());
        let mut ioc: Value = (time_in_force.clone() == Value::from("IOC")).into();
        let mut fok: Value = (time_in_force.clone() == Value::from("FOK")).into();
        let mut time_in_force_post_only: Value = (time_in_force.clone() == Value::from("PO")).into();
        post_only = (post_only.is_truthy() || time_in_force_post_only.is_truthy() || exchange_specific_param.is_truthy()).into();
        if post_only.is_truthy() {
            if ioc.is_truthy() || fok.is_truthy() {
                panic!(r###"InvalidOrder::new(self.get("id".into()) + Value::from(" postOnly orders cannot have timeInForce equal to ") + time_in_force.clone())"###);
            } else if is_market_order.is_truthy() {
                panic!(r###"InvalidOrder::new(self.get("id".into()) + Value::from(" market orders cannot be postOnly"))"###);
            } else {
                return true.into();
            };
        } else {
            return false.into();
        };
    }

    async fn fetch_trading_fees(&mut self, mut params: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchTradingFees() is not supported yet"))"###);
    }

    async fn fetch_trading_fee(&mut self, mut symbol: Value, mut params: Value) -> Value {
        if !self.get("has".into()).get(Value::from("fetchTradingFees")).is_truthy() {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchTradingFee() is not supported yet"))"###);
        };
        return self.fetch_trading_fees(params.clone()).await;
    }

    fn parse_open_interest(&mut self, mut interest: Value, mut market: Value) -> Value {
        panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" parseOpenInterest () is not supported yet"))"###);
    }

    fn parse_open_interests(&mut self, mut response: Value, mut market: Value, mut since: Value, mut limit: Value) -> Value {
        let mut interests: Value = Value::new_array();
        let mut i: usize = 0;
        while Value::from(i) < response.len().into() {
            let mut entry: Value = response.get(i.into());
            let mut interest: Value = self.parse_open_interest(entry.clone(), market.clone());
            interests.push(interest.clone());
            i += 1;
        };
        let mut sorted: Value = self.sort_by(interests.clone(), Value::from("timestamp"), Value::Undefined);
        let mut symbol: Value = self.safe_string(market.clone(), Value::from("symbol"), Value::Undefined);
        return self.filter_by_symbol_since_limit(sorted.clone(), symbol.clone(), since.clone(), limit.clone(), Value::Undefined);
    }

    async fn fetch_funding_rate(&mut self, mut symbol: Value, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchFundingRates")).is_truthy() {
            self.load_markets(Value::Undefined, Value::Undefined).await;
            let mut market: Value = self.market(symbol.clone());
            if !market.get(Value::from("contract")).is_truthy() {
                panic!(r###"BadSymbol::new(self.get("id".into()) + Value::from(" fetchFundingRate() supports contract markets only"))"###);
            };
            let mut rates: Value = self.fetch_funding_rates(Value::Json(serde_json::Value::Array(vec![symbol.clone().into()])), params.clone()).await;
            let mut rate: Value = self.safe_value(rates.clone(), symbol.clone(), Value::Undefined);
            if rate.clone() == Value::Undefined {
                panic!(r###"NullResponse::new(self.get("id".into()) + Value::from(" fetchFundingRate () returned no data for ") + symbol.clone())"###);
            } else {
                return rate.clone();
            };
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchFundingRate () is not supported yet"))"###);
        };
    }

    async fn fetch_mark_ohlcv(&mut self, mut symbol: Value, mut timeframe: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchMarkOHLCV")).is_truthy() {
            let mut request: Value = Value::Json(json!({
                "price": "mark"
            }));
            return self.fetch_ohlcv(symbol.clone(), timeframe.clone(), since.clone(), limit.clone(), self.extend_2(request.clone(), params.clone())).await;
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchMarkOHLCV () is not supported yet"))"###);
        };
    }

    async fn fetch_index_ohlcv(&mut self, mut symbol: Value, mut timeframe: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchIndexOHLCV")).is_truthy() {
            let mut request: Value = Value::Json(json!({
                "price": "index"
            }));
            return self.fetch_ohlcv(symbol.clone(), timeframe.clone(), since.clone(), limit.clone(), self.extend_2(request.clone(), params.clone())).await;
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchIndexOHLCV () is not supported yet"))"###);
        };
    }

    async fn fetch_premium_index_ohlcv(&mut self, mut symbol: Value, mut timeframe: Value, mut since: Value, mut limit: Value, mut params: Value) -> Value {
        if self.get("has".into()).get(Value::from("fetchPremiumIndexOHLCV")).is_truthy() {
            let mut request: Value = Value::Json(json!({
                "price": "premiumIndex"
            }));
            return self.fetch_ohlcv(symbol.clone(), timeframe.clone(), since.clone(), limit.clone(), self.extend_2(request.clone(), params.clone())).await;
        } else {
            panic!(r###"NotSupported::new(self.get("id".into()) + Value::from(" fetchPremiumIndexOHLCV () is not supported yet"))"###);
        };
    }

    fn handle_time_in_force(&mut self, mut params: Value) -> Value {
        let mut time_in_force: Value = self.safe_string_upper(params.clone(), Value::from("timeInForce"), Value::Undefined);
        if time_in_force.clone() != Value::Undefined {
            let mut exchange_value: Value = self.safe_string(self.get("options".into()).get(Value::from("timeInForce")), time_in_force.clone(), Value::Undefined);
            if exchange_value.clone() == Value::Undefined {
                panic!(r###"ExchangeError::new(self.get("id".into()) + Value::from(r#" does not support timeInForce ""#) + time_in_force.clone() + Value::from(r#"""#))"###);
            };
            return exchange_value.clone();
        };
        return Value::Undefined;
    }

    fn parse_account(&self, mut account: Value) -> Value {
        let mut accounts_by_type: Value = self.safe_value(self.get("options".into()), Value::from("accountsByType"), Value::new_object());
        let mut symbols: Value = self.get("symbols".into());
        if accounts_by_type.contains_key(account.clone()) {
            return accounts_by_type.get(account.clone());
        } else if self.in_array(account.clone(), symbols.clone()).is_truthy() {
            let mut market: Value = self.market(account.clone());
            return market.get(Value::from("id"));
        } else {
            return account.clone();
        };
    }

    fn handle_margin_mode_and_params(&mut self, mut method_name: Value, mut params: Value) -> Value {
        let mut default_margin_mode: Value = self.safe_string_2(self.get("options".into()), Value::from("marginMode"), Value::from("defaultMarginMode"), Value::Undefined);
        let mut method_options: Value = self.safe_value(self.get("options".into()), method_name.clone(), Value::new_object());
        let mut method_margin_mode: Value = self.safe_string_2(method_options.clone(), Value::from("marginMode"), Value::from("defaultMarginMode"), default_margin_mode.clone());
        let mut margin_mode: Value = self.safe_string_lower_2(params.clone(), Value::from("marginMode"), Value::from("defaultMarginMode"), method_margin_mode.clone());
        if margin_mode.clone() != Value::Undefined {
            params = self.omit(params.clone(), Value::Json(serde_json::Value::Array(vec![Value::from("marginMode").into(), Value::from("defaultMarginMode").into()])));
        };
        return Value::Json(serde_json::Value::Array(vec![margin_mode.clone().into(), params.clone().into()]));
    }
}