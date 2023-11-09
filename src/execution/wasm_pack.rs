use wasm_pack::build_project;

pub fn compile_contract(contract_rs: &str) -> Vec<u8> {
  let wasm_bytes = build_project(contract_rs).unwrap();
  wasm_bytes.as_ref().to_vec()
}