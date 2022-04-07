fn main() {
    let proto_root = "proto";
    let protos: Vec<_> = ["internal.proto"]
        .iter()
        .map(|file| format!("{proto_root}/{file}"))
        .collect();

    let includes = [proto_root];
    tonic_build::configure()
        .include_file("mod.rs")
        .build_server(true)
        .compile_well_known_types(true)
        .compile(&protos, &includes)
        .expect("compilable proto buf definition");
}
