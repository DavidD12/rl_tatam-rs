use std::collections::HashSet;

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

pub mod comp_skill_parser;
pub use comp_skill_parser::*;

pub mod skill_interface;
pub use skill_interface::*;



pub struct ModelTransNames {
    pub model: String,
    pub transition_names: Vec<String>,
}



pub fn to_tatam(skillset: &Skillset, composite_skill_names: &Vec<String>, label: bool, event: bool, interrupt: bool) -> String {
    let mut out = String::new();

    out += &skillset_to_tatam(skillset, composite_skill_names, label, event, interrupt);

    out += "\nprop = true\n";
    out += "\nsearch infinite + complete solve\n";

    out
}

pub fn effects_resources(effects: &Vec<Effect>) -> HashSet<ResourceId> {
    effects.iter().map(|e| e.resource().resolved()).collect()
}

pub fn postconsitions_resources(postconditions: &Vec<Postcondition>) -> HashSet<ResourceId> {
    let mut resources = HashSet::default();
    for post in postconditions {
        resources.extend(expr_resources(post.expr()));
    }
    resources
}
