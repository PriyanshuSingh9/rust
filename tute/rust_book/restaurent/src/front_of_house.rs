// Module separation (Chapter 7.5):
// In modern Rust (2018/2024 edition), declaring submodules inside `front_of_house.rs`
// loads `front_of_house/hosting.rs` and `front_of_house/serving.rs`.

pub mod hosting;
pub mod serving;
