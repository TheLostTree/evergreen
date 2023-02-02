use protobuf_codegen;


fn main(){
    let files_result = std::fs::read_dir("./protos");
    let files = match files_result{
        Ok(f) => f,
        Err(_) => {
            panic!("Please create a ./protos folder and place your proto files inside.");  
        },
    };
    
    let paths = files.map(|f| format!("protos/{}",f.unwrap().file_name().to_str().unwrap()));

    protobuf_codegen::Codegen::new()
    .pure()

    // All inputs and imports from the inputs must reside in `includes` directories.
    .includes(&["protos"])
    // Inputs must reside in some of include paths.
    // .input("protos/GetPlayerTokenReq.proto")
    // .input("protos/GetPlayerTokenRsp.proto")
    // .input("protos/StopServer.proto")
    // .input("protos/bytes.proto")
    .inputs(paths)
    // Specify output directory relative to Cargo output directory.
    .cargo_out_dir("protos_target")
    .run_from_script();
}