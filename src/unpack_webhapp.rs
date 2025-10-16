
use anyhow::anyhow;
use holochain_types::web_app::WebAppBundle;
use sha2::*;
use std::fs;


pub fn hash_bytes_sha256(bytes: Vec<u8>) -> String {
   let mut hasher = Sha256::new();
   hasher.update(bytes);
   hex::encode(hasher.finalize())
}

/// Output to stdout the hash of the file given as input
#[tokio::main]
async fn main() -> anyhow::Result<()> {
   let mut args = std::env::args();
   args.next(); // skip exe
   let Some(file_path) = args.next() else {
      return Err(anyhow!("Missing input argument"));
   };
   let Some(out_file_path) = args.next() else {
      return Err(anyhow!("Missing output argument"));
   };
   // Load webhapp file
   let webhapp_bytes = fs::read(file_path)?;
   let web_happ_hash = hash_bytes_sha256(webhapp_bytes.clone());
   println!("webhapp hash: {}", web_happ_hash);
   // Decode webhapp file
   let Ok(web_app_bundle) = WebAppBundle::decode(&webhapp_bytes)
      else { return Err(anyhow!("Invalid webhapp file")); };

   // extracting happ bundle
   let app_bundle = web_app_bundle.happ_bundle().await?;

   let app_bundle_bytes = app_bundle.encode()?;

   let happ_hash = hash_bytes_sha256(app_bundle_bytes.clone());
   println!("   happ hash: {}", happ_hash);

   fs::write(out_file_path, app_bundle_bytes)?;

   Ok(())
}