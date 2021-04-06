use rand::{self, seq::SliceRandom};
use self_encryption::{
    test_helpers::{new_test_rng, random_bytes, SimpleStorage},
    DataMap, SelfEncryptionError, SelfEncryptor,
};

const DATA_SIZE: usize = 20 * 1024 * 1024;

#[tokio::main]
async fn main() {
    let res = real_main().await;
}

async fn real_main() -> Result<(), SelfEncryptionError> {
    let storage = SimpleStorage::new();
    let se = SelfEncryptor::new(storage, DataMap::None)?;
    let content = vec![0u8; DATA_SIZE];
    se.write(&content, 0).await?;
    let (dm, st) = se.close().await?;
    println!("{:?}", dm);
    Ok(())
}
