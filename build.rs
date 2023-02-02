use std::{path::Path, env, fs::{self, File}, io::Write};

use protobuf_codegen;


fn main(){
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let gen_dir = Path::new(&out_dir).join("cmdids_target");
    if !gen_dir.exists() {
        fs::create_dir(&gen_dir).unwrap();
    }
    

    let dest_path = gen_dir.join("cmdids.rs");
    println!("cargo:warning=dest_path: {}", dest_path.display());

    generate_cmdid_file(dest_path);
    
    
    generate_protobufs();

    println!("cargo:rustc-env=GENERATED_ENV={}", gen_dir.display());
    println!("cargo:rustc-cfg=has_generated_feature");
}


fn generate_cmdid_file<P: AsRef<Path>>(path: P){
    let mut contents = String::new();
    // contents.push_str("pub mod cmdids {\n");
    contents.push_str("pub enum CmdIds {\n");
    let binding = std::fs::read_to_string("./CmdIds.csv").expect("place ur cmdids pls");
    let lines = binding.lines();
    _ = lines.clone().for_each(|line|{
        let mut split = line.split(",");
        let name = split.next().unwrap();
        let cmdid = split.next().unwrap();
        contents.push_str(&format!("\t{} = {},\n", name, cmdid));
    });
    contents.push_str("}\n");

    // impl from_u32
    contents.push_str("impl CmdIds {\n");
    contents.push_str("\tpub fn from_u16(id: u16) -> Option<CmdIds> {\n");
    contents.push_str("\t\tmatch id {\n");
    lines.for_each(|line|{
        let mut split = line.split(",");
        let name = split.next().unwrap();
        let cmdid = split.next().unwrap();
        contents.push_str(&format!("\t\t\t{} => Some(CmdIds::{}),\n", cmdid, name));
    });
    contents.push_str("\t\t\t_ => None,\n");
    contents.push_str("\t\t}\n");
    contents.push_str("\t}\n");
    contents.push_str("}\n");

    generate_file(&path, contents.as_bytes());
}

fn generate_protobufs(){
    let files = match std::fs::read_dir("./protos"){
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
    .inputs(paths)
    // Specify output directory relative to Cargo output directory.
    .cargo_out_dir("protos_target")
    .run_from_script();
}


fn generate_file<P: AsRef<Path>>(path: P, text: &[u8]) {
    let mut f = File::create(path).unwrap();
    f.write_all(text).unwrap()
}