//! Файл компилирует код Rust из описания API на Protobuf.

fn main() {
  tonic_build::compile_protos("proto/users.proto")
    .unwrap_or_else(|e| panic!("Ошибка при компиляции proto: {:?}", e));
}
