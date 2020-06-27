use js_sys::Date;
use js_sys::Math;
use sha2::Sha256;
use sha2::Digest;
use crate::kv;

// helper functions
// crypto

pub fn hash(string: &str) -> String {
    Sha256::digest(string.as_bytes())
        .iter()
        .map(|x| format!("{:x?}", x))
        .collect::<String>()
}

pub fn stamp() -> Result<String, String> {
    let stream = (0..32).map(|_| Math::random().to_string()).collect::<Vec<String>>().join("");
    let pre_stamp = Date::now().to_string() + &stream;
    Ok(hash(&pre_stamp))
}

// key-value

pub async fn write(value: &str) -> Result<String, String> {
    let key = hash(value);
    kv::value(kv::AddressedNS::put(&key, value))
        .await.ok_or("Could not write to kv")?;
    return Ok(key);
}

pub async fn read(key: &str) -> Result<String, String> {
    let x = kv::AddressedNS::get(key, "text");
    kv::value(x)
        .await.ok_or("Could not read from kv")?
        .as_string().ok_or("Could not convert kv-read value to String".to_owned())
}

pub async fn mutate(key: &str, value: &str) -> Result<(), String> {
    kv::value(kv::AddressedNS::put(key, value))
        .await.ok_or("Could not mutate kv")?;
    return Ok(());
}

pub async fn append(key: &str, value: Vec<String>) -> Result<(), String> {
    let contents = read(key).await? + "\n" + &value.join("\n");
    kv::value(kv::AddressedNS::put(&key, &contents))
        .await.ok_or("Could not push to kv")?;
    return Ok(());
}

pub async fn push(key: &str, value: String) -> Result<(), String> {
    return Ok(append(key, vec![value]).await?);
}

pub async fn list(key: &str) -> Result<Vec<String>, String> {
    Ok(
        read(key)
            .await?
            .split("\n")
            .filter(|x| x != &"")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
    )
}

pub async fn ensure(key: &str) -> Result<(), String> {
    if let Ok(_) = read(key).await {
        return Ok(());
    }
    kv::value(kv::AddressedNS::put(&key, ""))
        .await.ok_or("Could not kv-create new empty list")?;
    return Ok(());
}
