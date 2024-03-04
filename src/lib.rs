use rl_model::model::*;

pub mod skillset;
pub use skillset::*;

pub mod resource;
pub use resource::*;

pub mod event;
pub use event::*;

pub fn to_tatam(model: &Model) -> String {
    let mut out = "".to_string();

    out += &skillsets_to_tatam(model);

    out
}
