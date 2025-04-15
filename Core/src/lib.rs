#![feature(string_remove_matches)]

mod data;
mod gen_kana;
mod gen_table;
mod types;

pub use data::*;
pub use gen_kana::*;
pub use gen_table::*;
pub use types::*;
use types::{config::AzikConfig, tokens::Assignable};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn gen_google_ime_runner() -> String {
    gen_google_ime_table::gen_google_ime_table(AzikConfig {
        Hatsuon: "c".to_string(),
        Sokuon: "v".to_string(),
        Sequence: vec![Assignable {
            Token: "c".to_string(),
            Sequence: "ou".to_string(),
        }],
    })
}
