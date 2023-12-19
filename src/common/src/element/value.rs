use std::{collections::HashMap, time::Duration};

pub enum Value {
    Null,
    Boolean(bool),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),

    // Decimal128(Decimal128),
    String(String),

    // Binary(Bytes),
    // Date(Date),
    // DateTime(DateTime),
    // Timestamp(Timestamp),
    // Time(Time),
    Duration(Duration),
    // Interval(Interval),
    List(Vec<Value>),
    Object(HashMap<Value, Value>),
}
