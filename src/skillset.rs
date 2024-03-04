use super::*;

pub fn skillsets_to_tatam(model: &Model) -> String {
    let mut out = "".to_string();

    for skillset in model.skillsets() {
        out += &skillset_to_tatam(skillset);
    }

    out
}

pub fn skillset_to_tatam(skillset: &Skillset) -> String {
    let mut out = "".to_string();

    out += &resources_to_tatam(skillset);

    out
}
