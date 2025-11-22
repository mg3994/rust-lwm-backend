fn main() {
    tonic_prost_build::compile_protos("proto/service.proto").unwrap();
    println!("cargo:rerun-if-changed=proto/service.proto");
}
