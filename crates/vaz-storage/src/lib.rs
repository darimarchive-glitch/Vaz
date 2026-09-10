use anyhow::{anyhow, Result};
use reed_solomon_erasure::galois_8::ReedSolomon;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shard { pub index: usize, pub hash: [u8; 32], pub bytes: Vec<u8> }

pub fn encode(data: &[u8], data_shards: usize, parity_shards: usize) -> Result<Vec<Shard>> {
    if data_shards == 0 || parity_shards == 0 { return Err(anyhow!("shard counts must be non-zero")); }
    let rs = ReedSolomon::new(data_shards, parity_shards)?;
    let mut framed = (data.len() as u64).to_le_bytes().to_vec(); framed.extend_from_slice(data);
    let shard_len = (framed.len() + data_shards - 1) / data_shards;
    framed.resize(shard_len * data_shards, 0);
    let mut raw: Vec<Vec<u8>> = framed.chunks(shard_len).map(|x| x.to_vec()).collect();
    raw.extend((0..parity_shards).map(|_| vec![0u8; shard_len]));
    rs.encode(&mut raw)?;
    Ok(raw.into_iter().enumerate().map(|(index, bytes)| Shard { index, hash: *blake3::hash(&bytes).as_bytes(), bytes }).collect())
}

pub fn reconstruct(shards: Vec<Option<Shard>>, data_shards: usize, parity_shards: usize) -> Result<Vec<u8>> {
    if shards.len() != data_shards + parity_shards { return Err(anyhow!("wrong shard count")); }
    let mut raw: Vec<Option<Vec<u8>>> = shards.into_iter().map(|s| s.and_then(|s| if *blake3::hash(&s.bytes).as_bytes() == s.hash { Some(s.bytes) } else { None })).collect();
    ReedSolomon::new(data_shards, parity_shards)?.reconstruct(&mut raw)?;
    let mut framed = Vec::new();
    for shard in raw.into_iter().take(data_shards) { framed.extend_from_slice(&shard.ok_or_else(|| anyhow!("missing data shard"))?); }
    if framed.len() < 8 { return Err(anyhow!("bad frame")); }
    let len = u64::from_le_bytes(framed[..8].try_into().unwrap()) as usize;
    if len > framed.len() - 8 { return Err(anyhow!("bad length")); }
    Ok(framed[8..8 + len].to_vec())
}

#[cfg(test)]
mod tests { use super::*; #[test] fn survives_loss() { let s=encode(b"VAZ distribuida",4,2).unwrap(); let mut o:Vec<_>=s.into_iter().map(Some).collect(); o[1]=None; o[5]=None; assert_eq!(reconstruct(o,4,2).unwrap(),b"VAZ distribuida"); } }