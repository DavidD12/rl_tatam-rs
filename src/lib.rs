use rl_model::model::*;

pub mod skillset;
pub use skillset::*;

pub mod resource;
pub use resource::*;

pub mod event;
pub use event::*;

pub mod skill;
pub use skill::*;

pub mod expr;
pub use expr::*;

pub mod naming;
pub use naming::*;

pub fn to_tatam(model: &Model) -> String {
    let mut out = "".to_string();

    out += &skillsets_to_tatam(model);

    out += "prop = G(F(custom_robot_goto_state = Success))\n";
    out += "\nsearch infinite + complete solve\n";

    out
}

pub fn used_resources(effects: &Vec<Effect>) -> Vec<ResourceId> {
    effects.iter().map(|e| e.resource().resolved()).collect()
}
