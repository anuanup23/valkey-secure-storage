use lazy_static::lazy_static;
use redis_module::redis_command;
use redis_module::redis_module;
use redis_module::{Context, NextArg, REDIS_OK, RedisError, RedisResult, RedisString, RedisValue};
use std::collections::HashMap;
use std::sync::RwLock;
// Use a RwLock-protected HashMap to store the secure key-value pairs
lazy_static! {
    static ref SECURE_STORE: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

/// Command to store a key-value pair in the secure storage
fn secure_set(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 3 {
        return Err(RedisError::WrongArity);
    }

    // Always use next_arg() pattern to consume arguments
    let mut args = args.into_iter().skip(1);
    let key: String = args.next_arg()?.try_into()?;
    let value: String = args.next_arg()?.try_into()?;

    // Acquire write lock and store the key-value
    match SECURE_STORE.write() {
        Ok(mut store) => {
            store.insert(key, value);
            ctx.replicate_verbatim();
            Ok(RedisValue::SimpleString("OK".to_string()))
        }
        Err(_) => Err(RedisError::String("Failed to acquire write lock".into())),
    }
}

/// Command to retrieve a value from the secure storage
fn secure_get(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }

    // Use next_arg() pattern here too
    let mut args = args.into_iter().skip(1);
    let key: String = args.next_arg()?.try_into()?;

    // Acquire read lock and get the value
    match SECURE_STORE.read() {
        Ok(store) => match store.get(&key) {
            Some(value) => Ok(RedisValue::BulkString(value.clone().into())),
            None => Ok(RedisValue::Null),
        },
        Err(_) => Err(RedisError::String("Failed to acquire read lock".into())),
    }
}

/// Command to delete a key from the secure storage
fn secure_del(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }

    // Use next_arg() pattern
    let mut args = args.into_iter().skip(1);
    let key: String = args.next_arg()?.try_into()?;

    // Acquire write lock and remove the key
    match SECURE_STORE.write() {
        Ok(mut store) => {
            let removed = store.remove(&key).is_some() as i64;
            ctx.replicate_verbatim();
            Ok(RedisValue::Integer(removed))
        }
        Err(_) => Err(RedisError::String("Failed to acquire write lock".into())),
    }
}

/// Return a list of all keys in the secure storage
fn secure_keys(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }

    // Acquire read lock and collect all keys
    match SECURE_STORE.read() {
        Ok(store) => {
            let keys: Vec<RedisValue> = store
                .keys()
                .map(|k| RedisValue::BulkString(k.clone().into()))
                .collect();
            Ok(RedisValue::Array(keys))
        }
        Err(_) => Err(RedisError::String("Failed to acquire read lock".into())),
    }
}

// Redis module initialization
redis_module! {
    name: "securestorage",
    version: 1,
    data_types: [],
    commands: [
        ["secure.set", secure_set, "write", 1, 1, 1],
        ["secure.get", secure_get, "readonly", 1, 1, 1],
        ["secure.del", secure_del, "write", 1, 1, 1],
        ["secure.keys", secure_keys, "readonly", 0, 0, 0],
    ],
}
