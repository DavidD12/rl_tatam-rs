use super::*;

pub fn skill_interface_to_tatam(skillset: &Skillset, composite_skill_names: &Vec<String>) -> String {
    let mut out = String::new();

    out += "\n// -------------------- Skill Interface --------------------\n";

    for compo_name in composite_skill_names {
        for skill in skillset.skills().iter() {
            out += &format!("\nvar {}: SkillInterfaceState\n", &interface_var(skillset, skill, &compo_name));
            out += &format!("var {}: SkillCallResults\n", &interface_result_var(skillset, skill, &compo_name));
        }
    }
    
    out += &format!("\ninit init_{}_skill_interface {{\n", skillset.name());

    for compo_name in composite_skill_names {
        for skill in skillset.skills().iter() {
            out += &format!("\t{} = NoCall\n", &interface_var(skillset, skill, &compo_name));
            out += &format!("\tand {} = NoneRes and\n", &interface_result_var(skillset, skill, &compo_name));
        }
    }
    out.truncate(out.len()-5);
    out += "\n}\n";

    out
}
