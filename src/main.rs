#![allow(unused_doc_comments)]
#![allow(unused_attributes)]

mod dna_wasm;

use anyhow::anyhow;
use dna_wasm::{DnaWasm, DnaWasmHashed};

/// Output to stdout the hash of the file given as input
#[tokio::main]
async fn main() -> anyhow::Result<()> {
   let mut args = std::env::args();
   args.next(); // skip exe
   let zome_wasm_path = if let Some(arg) = args.next() {
      arg
   } else {
      return Err(anyhow!("Missing input argument"));
   };
   /// Load Wasm file
   let zome_wasm = &std::fs::read(zome_wasm_path)?;
   /// Create vanilla Dna out of zome
   let dna_wasm = DnaWasm::from(zome_wasm.to_vec());
   let (_, zome_hash) = DnaWasmHashed::from_content(dna_wasm.clone())
      .await
      .into_inner();
   /// print hash
   print!("{}", zome_hash);
   Ok(())
}
