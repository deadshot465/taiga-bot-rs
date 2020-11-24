fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protos/discord_bot_service.proto")?;
    tonic_build::compile_protos("protos/jwt_token_service.proto")?;
    Ok(())
}
