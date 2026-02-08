fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .compile_protos(
            &["proto/user.proto", "proto/health.proto"],
            &["proto", "/usr/include"],
        )
        .expect("TODO: tonic panic message");
    Ok(())
}

