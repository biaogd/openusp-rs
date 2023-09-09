extern crate prost_build;

fn main() {
    println!("cargo:rerun-if-changed=src/proto/usp-msg.proto");
    println!("cargo:rerun-if-changed=src/proto/usp-record.proto");
    prost_build::compile_protos(&["src/proto/usp-msg.proto", "src/proto/usp-record.proto"], &["src/"]).unwrap();
}