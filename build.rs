use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = Path::new("./proto").canonicalize()?;
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true)
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_protos(&[proto_dir.join("chatting.proto")], &[proto_dir])?;
    Ok(())
}
