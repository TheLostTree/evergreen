use std::{thread::JoinHandle, collections::HashMap};

use common::cmdids::CmdIds;
use protobuf::reflect::FileDescriptor;

pub struct DynProtoHandler{
    descriptors: Option<HashMap<CmdIds, FileDescriptor>>,
    descriptor_load: Option<JoinHandle<HashMap<CmdIds, FileDescriptor>>>
    
}
impl DynProtoHandler{
    pub fn new()->Self{
        DynProtoHandler { 
            descriptors: None,
            descriptor_load: Some(std::thread::spawn(||{
                common::protos::load_dyn_protos()
            }))
        }
    }


    pub fn get_descriptor(&mut self, cmdid: CmdIds)->Option<FileDescriptor>{
        //these clones are probably ok
        // "The object is refcounted: clone is shallow.""
        // "The equality performs pointer comparison: two clones of the same FileDescriptor objects are equal"

        if let Some(fdesc) = self.descriptors.as_ref().and_then(|x|x.get(&cmdid)){
            Some(fdesc.clone())
        } else{
            // oh god....
            if let Some(descriptor_results) = self.descriptor_load.take().map(|f|JoinHandle::join(f)){
                if let Ok(descriptors) = descriptor_results{
                    _ = self.descriptors.insert(descriptors);
                }else{
                    println!("failed to load descriptors :(");
                    return None;
                }
            }else{
                println!("no descriptors loaded, thread something something join idk");
                return None;
            }

            // the thread is finished loading stuff
            // now we can get the descriptor
            if let Some(fdesc) = self.descriptors.as_ref().and_then(|x|x.get(&cmdid)){
                Some(fdesc.clone())
            } else{
                println!("descriptor not found");
                None
            }
        }
    }

    

}