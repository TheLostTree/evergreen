pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos_target/mod.rs"));

    use crate::cmdids::CmdIds;
    use protobuf::reflect::FileDescriptor;
    use protobuf_parse::Parser;
    use std::{collections::HashMap, str::FromStr};

    pub fn load_dyn_protos() -> HashMap<CmdIds, FileDescriptor> {
        let x = Parser::new()
            .pure()
            .inputs(
                std::fs::read_dir("./all_protos")
                    .unwrap()
                    .map(|v| v.unwrap().path())
                    .filter(|v| v.extension().map_or(false, |u| u == "proto"))
                    .collect::<Vec<_>>(),
            )
            .include("./all_protos")
            .parse_and_typecheck()
            .unwrap();
        // x.file_descriptors
        //haha.... clone....
        let mut map = HashMap::new();
        for descriptor in FileDescriptor::new_dynamic_fds(x.file_descriptors, &[]).expect("oopsie!")
        {
            // println!("{}", descriptor.name());
            let cmd = CmdIds::from_str(descriptor.name().split(".").next().expect("hmm.."));
            if let Ok(c) = cmd {
                map.insert(c, descriptor);
            }
        }
        map
    }
}
pub mod cmdids {
    include!(concat!(env!("OUT_DIR"), "/cmdids_target/cmdids.rs"));
}
