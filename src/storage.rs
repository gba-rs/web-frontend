use idb::{Database, DatabaseEvent, Error as IdbError, ObjectStoreParams, TransactionMode};
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;

const DB_NAME: &str = "gba-web-frontend";
const DB_VERSION: u32 = 1;
const STORE_NAME: &str = "blobs";

pub async fn open_database() -> Result<Database, IdbError> {
    let factory = idb::Factory::new()?;
    let mut open_request = factory.open(DB_NAME, Some(DB_VERSION))?;

    open_request.on_upgrade_needed(|event| {
        if let Ok(database) = event.database() {
            let _ = database.create_object_store(STORE_NAME, ObjectStoreParams::new());
        }
    });

    open_request.await
}

pub async fn put_bytes(db: &Database, key: &str, bytes: &[u8]) -> Result<(), IdbError> {
    let tx = db.transaction(&[STORE_NAME], TransactionMode::ReadWrite)?;
    let store = tx.object_store(STORE_NAME)?;
    let array = Uint8Array::from(bytes);
    store.put(&JsValue::from(array), Some(&JsValue::from_str(key)))?.await?;
    tx.commit()?;
    Ok(())
}

pub async fn get_bytes(db: &Database, key: &str) -> Result<Option<Vec<u8>>, IdbError> {
    let tx = db.transaction(&[STORE_NAME], TransactionMode::ReadOnly)?;
    let store = tx.object_store(STORE_NAME)?;
    let result: Option<JsValue> = store.get(JsValue::from_str(key))?.await?;
    Ok(result.map(|value| Uint8Array::new(&value).to_vec()))
}
