use candid::{ser::IDLBuilder, CandidType, utils::ArgumentEncoder};
use ic_cdk::api::stable::{StableWriter, StableReader};
use serde::Deserialize;

pub fn serialize<T: CandidType> (
    value: &T,
    writer: &mut StableWriter
) -> Result<(), String> {
    let mut ser = IDLBuilder::new();
    (value, ).encode(&mut ser).map_err(|e| format!("{:?}", e))?;
    let arr = ser.serialize_to_vec().unwrap();
    // store size
    writer.write(&u64::to_le_bytes(arr.len() as u64)).map_err(|e| format!("{:?}", e))?;
    // store value
    writer.write(&arr).map_err(|e| format!("{:?}", e))?;
    Ok(())
}

pub fn deserialize<T: CandidType + for<'a> Deserialize<'a>>(
    reader: &mut StableReader
) -> Result<T, String> {
    // load size
    let mut size_buf = [0u8; 8];
    reader.read(&mut size_buf).map_err(|e| format!("{:?}", e))?;
    let size = u64::from_le_bytes(size_buf);
    // load table
    let mut table_buf = vec![0u8; size as usize];
    reader.read(&mut table_buf).map_err(|e| format!("{:?}", e))?;
    // decode table
    let res = candid::decode_args::<'_, (T, )>(&table_buf)
        .map_err(|e| format!("{:?}", e))?;
    Ok(res.0)
}