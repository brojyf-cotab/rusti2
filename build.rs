fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto = "proto/rusti2/v1/object_storage.proto";
    println!("cargo:rerun-if-changed={proto}");
    tonic_build::configure().compile_protos(&[proto], &["proto"])?;
    Ok(())
}
