fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/proto/echo.proto")?;
    tonic_build::compile_protos("src/proto/fulcrum.proto")?;
    
    Ok(())
}