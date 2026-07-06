use std::sync::Arc;

use serde_json::{Number, Value};
use tauri::{AppHandle, Wry};
use tauri_plugin_store::{Store, StoreExt};

const STORE_PATH: &str = "store.json";

pub fn load(app: AppHandle) -> Result<Arc<Store<Wry>>, String> {
    let store = app
        .store(STORE_PATH)
        .map_err(|e| format!("error while loading store: {}", e))?;

    Ok(store)
}

pub fn get(app: AppHandle, key: &str, default: Value) -> Result<Value, String> {
    let store = load(app.clone())?;
    let keys = keys(app.clone(), None)?;

    if !keys.contains(&key.to_string()) {
        Ok(default)
    } else {
        store
            .get(&key)
            .map(|v| v.clone())
            .ok_or_else(|| format!("error while loading data: key={}", key))
    }
}

pub fn set(app: AppHandle, key: &str, value: Value) -> Result<(), String> {
    let store = load(app)?;

    store.set(key, value);

    store
        .save()
        .map_err(|e| format!("error while saving data: {}", e))
}

pub fn keys(app: AppHandle, store: Option<Arc<Store<Wry>>>) -> Result<Vec<String>, String> {
    let _store = store.unwrap_or(load(app)?);

    Ok(_store.keys())
}

pub fn delete(app: AppHandle, key: &str) -> Result<(), String> {
    let store = load(app)?;

    if !store.delete(&key) {
        return Err(format!(
            "error while deleting because the key '{}' not found",
            key
        ));
    }

    store
        .save()
        .map_err(|e| format!("error while deleting data: {}", e))
}

pub fn clear(app: AppHandle) -> Result<(), String> {
    let store = load(app.clone())?;

    let keys = keys(app.clone(), None)?;
    for key in keys {
        store.delete(&key);
    }

    store
        .save()
        .map_err(|e| format!("error while clearing store: {}", e))
}

#[tauri::command]
pub fn is_minimum(app: AppHandle) -> Result<bool, String> {
    let value = get(app, "is_minimum", Value::Bool(false))?;

    Ok(value.as_bool().unwrap_or(false))
}

pub fn set_minimum(app: AppHandle, value: bool) -> Result<(), String> {
    set(app, &"is_minimum".to_string(), Value::Bool(value))?;

    Ok(())
}

pub fn get_height(app: AppHandle) -> Result<f64, String> {
    let number = Number::from_f64(app.config().app.windows.first().unwrap().height).unwrap();

    let value = get(app, "height", Value::Number(number))?;

    Ok(value.as_f64().unwrap())
}

pub fn set_height(app: AppHandle, value: f64) -> Result<(), String> {
    set(
        app,
        &"height".to_string(),
        Value::Number(Number::from_f64(value).unwrap()),
    )?;

    Ok(())
}
