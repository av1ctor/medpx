use candid::{ser::IDLBuilder, CandidType, utils::ArgumentEncoder};
use ic_cdk::api::stable::{StableWriter, StableReader};
use serde::Deserialize;

pub fn serialize<T: CandidType> (
    value: &T,
    version: f32,
    writer: &mut StableWriter
) -> Result<(), String> {
    let mut ser = IDLBuilder::new();
    (value, ).encode(&mut ser).map_err(|e| format!("{:?}", e))?;
    let arr = ser.serialize_to_vec().unwrap();
    // store version
    writer.write(&f32::to_le_bytes(version)).map_err(|e| format!("{:?}", e))?;
    // store size
    writer.write(&u64::to_le_bytes(arr.len() as u64)).map_err(|e| format!("{:?}", e))?;
    // store value
    writer.write(&arr).map_err(|e| format!("{:?}", e))?;
    Ok(())
}

pub fn deserialize<T: CandidType + for<'a> Deserialize<'a>>(
    version: f32,
    reader: &mut StableReader
) -> Result<T, String> {
    // load version
    let mut version_buf = [0u8; 4];
    reader.read(&mut version_buf).map_err(|e| format!("{:?}", e))?;
    let stored_version = f32::from_le_bytes(version_buf);
    if stored_version != version {
        return Err("Invalid version".to_string());
    }
    // load size
    let mut size_buf = [0u8; 8];
    reader.read(&mut size_buf).map_err(|e| format!("{:?}", e))?;
    let size = u64::from_le_bytes(size_buf);
    // load value
    let mut table_buf = vec![0u8; size as usize];
    reader.read(&mut table_buf).map_err(|e| format!("{:?}", e))?;
    // decode value
    let res = candid::decode_args::<'_, (T, )>(&table_buf)
        .map_err(|e| format!("{:?}", e))?;
    Ok(res.0)
}