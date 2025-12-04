fn main() {
    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["proto/device.proto"], &["proto/"])
        .unwrap();
}
