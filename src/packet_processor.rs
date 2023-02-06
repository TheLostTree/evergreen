// this code is intended to use the data from parsed packets to calculate dps or whatnot

use std::collections::HashMap;

use crate::cmdids::CmdIds;

pub fn get_handlers(){
    let mut handlers = HashMap::new();
    handlers.insert(crate::cmdids::CmdIds::SceneEntityAppearNotify, scene_entity_appear);
}

fn scene_entity_appear(bytes: &u8, cmd: CmdIds){

}

