use std::path::Path;

fn main() {
    tonic_build::configure()
        .out_dir(Path::new("src"))
        .compile(&["api/proto/base.proto", "api/proto/message_a.proto",  "api/proto/message_b.proto"], &["api/proto"])
        .unwrap()
}