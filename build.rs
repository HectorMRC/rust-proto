fn main()->Result<(),Box<dyn std::error::Error>>{
    // compiling protos using path on build time
    tonic_build::compile_protos("proto/endpoint/request.proto")?;
    tonic_build::compile_protos("proto/endpoint/service.proto")?;
    Ok(())
}