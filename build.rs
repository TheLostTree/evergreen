use std::{path::Path, env, fs::{self, File}, io::Write, str::Lines};

use protobuf_codegen;


fn main(){
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let gen_dir = Path::new(&out_dir).join("cmdids_target");
    if !gen_dir.exists() {
        fs::create_dir(&gen_dir).unwrap();
    }
    

    let dest_path = gen_dir.join("cmdids.rs");
    // println!("cargo:warning=dest_path: {}", dest_path.display());

    generate_cmdid_file(dest_path);
    
    
    generate_protobufs();

    println!("cargo:rustc-env=GENERATED_ENV={}", gen_dir.display());
    println!("cargo:rustc-cfg=has_generated_feature");
}


fn generate_cmdid_file<P: AsRef<Path>>(path: P){

    //yes i know this is a mess
    
    let mut contents = String::new();
    // contents.push_str("pub mod cmdids {\n");
    contents.push_str("#[allow(non_camel_case_types)]\n");
    contents.push_str("#[derive(Debug,Hash,Eq,PartialEq)]\n");
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
    lines.clone().for_each(|line|{
        let mut split = line.split(",");
        let name = split.next().unwrap();
        let cmdid = split.next().unwrap();
        contents.push_str(&format!("\t\t\t{} => Some(CmdIds::{}),\n", cmdid, name));
    });
    contents.push_str("\t\t\t_ => None,\n");
    contents.push_str("\t\t}\n");
    contents.push_str("\t}\n");
    contents.push_str("}\n");

    contents.push_str("
#[derive(Debug, PartialEq, Eq)]
pub struct ParseCmdErr;
impl std::str::FromStr for CmdIds{
    ");
    contents.push_str("
type Err = ParseCmdErr;
fn from_str(s:&str) -> Result<Self, Self::Err> {

");
    contents.push_str("\tmatch s {\n");
    lines.clone().for_each(|line|{
        let mut split = line.split(",");
        let name = split.next().unwrap();
        // let cmdid = split.next().unwrap();
        contents.push_str(&format!("\t\t\t\"{}\" => Ok(CmdIds::{}),\n", name, name));
    });
    contents.push_str("\t\t\t_ => Err(ParseCmdErr),\n");
    contents.push_str("\t\t}\n");
    contents.push_str("\t}\n");
    contents.push_str("}\n");

    contents.push_str("
use std::fmt;
impl std::fmt::Display for CmdIds{
    ");
    contents.push_str("
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {

");
    contents.push_str("\tmatch self {\n");
    lines.clone().for_each(|line|{
        let mut split = line.split(",");
        let name = split.next().unwrap();
        // let cmdid = split.next().unwrap();
        contents.push_str(&format!("\t\t\tCmdIds::{} => write!(f, \"{}\"),\n", name, name));
    });
    contents.push_str("\t\t\t_ => Err(std::fmt::Error),\n");
    contents.push_str("\t\t}\n");
    contents.push_str("\t}\n");
    contents.push_str("}\n");


    generate_file(&path, contents.as_bytes());

    
    // generate_proto_decodes(lines)
}

fn generate_protobufs(){

    let protodir = "protos";
    let files = match std::fs::read_dir(format!("./{}", protodir)){
        Ok(f) => f,
        Err(_) => {
            panic!("Please create a ./protos folder and place your proto files inside.");  
        },
    };
    let paths = files.map(|f| format!("{}/{}",protodir,f.unwrap().file_name().to_str().unwrap()));

    protobuf_codegen::Codegen::new()
    .pure()

    // All inputs and imports from the inputs must reside in `includes` directories.
    .includes(&[protodir])
    .inputs(paths)
    // Specify output directory relative to Cargo output directory.
    .cargo_out_dir("protos_target")
    .run_from_script();
}


#[allow(dead_code)]
fn generate_proto_decodes(cmds: Lines){
    let mut contents = String::new();
    contents.push_str("use crate::protos::*;\n");
    contents.push_str("use crate::cmdids::CmdIds;\n");
    contents.push_str("use crate::client_server_pair::Packet;\n");
    contents.push_str("use protobuf_json_mapping::print_to_string_with_options;\n");
    contents.push_str("use protobuf::Message;\n\n");






    contents.push_str("pub fn default_decode_proto(p: &mut Packet, cmd: CmdIds)->Option<String>{\n");
    contents.push_str("
    
    let options = protobuf_json_mapping::PrintOptions{
        enum_values_int :false,
        proto_field_name: false,
        always_output_default_values: true,
        _future_options: (),
    };
");
    contents.push_str("\tmatch cmd {\n");
    let dir = Path::new("./protos");
    for line in cmds{
        let mut split = line.split(",");
        let name = split.next().unwrap();
        // let cmdid = split.next().unwrap();
        //check to make sure it has a corresponding .proto file
        if fs::metadata(dir.join(&format!("{}.proto", name))).is_err(){
            continue;
        }

        contents.push_str(&format!("
        CmdIds::{}=>{{
            let x = {}::{}::parse_from_bytes(&p.data);
            return match x{{
                Ok(v) => {{
                    Some(print_to_string_with_options(&v,&options).unwrap())
                }},
                Err(err) => {{
                    println!(\"{{}}\", err);
                    None
                }},
            }};
        }}
            ", name,name,name));
    }
    contents.push_str("
        _ => {None}
    }
}
    
    ");

    let dir = env::var("OUT_DIR").unwrap();

    let gen_dir = Path::new(&dir).join("./proto_decode.rs");
    generate_file(gen_dir, contents.as_bytes());

}


fn generate_file<P: AsRef<Path>>(path: P, text: &[u8]) {
    let mut f = File::create(path).unwrap();
    f.write_all(text).unwrap()
}