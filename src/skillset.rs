use super::*;

pub fn skillset_to_tatam(skillset: &Skillset) -> String {
    let mut out = "".to_string();

    out += "enum SkillsetState = { Free, Lock }\n";
    out += "enum SkillState = { Idle, Running, Interrupting, InvariantFailure, Success, Failure, Interrupted}\n";

    out += &resources_to_tatam(skillset);
    out += &events_to_tatam(skillset);

    out += "// ==================== Skillset ====================\n";
    out += &format!("var {}: SkillsetState\n", skillset_var(skillset));
    out += &format!("\ninit init_{} {{\n", skillset.name());
    out += &format!("\t{} = Free\n", skillset_var(skillset),);
    out += "}\n";

    // Invariant
    out += &skillset_invariant_propagation(skillset);

    // Skills
    out += "\n// ==================== Skill ====================\n";

    for skill in skillset.skills().iter() {
        out += &skill_to_tatam(skillset, skill);
    }

    out
}

fn skillset_invariant_propagation(skillset: &Skillset) -> String {
    let invariants = skillset
        .skills()
        .iter()
        .flat_map(|s| s.invariants())
        .collect::<Vec<_>>();

    let mut out = String::new();

    if let Some((first, others)) = invariants.split_first() {
        out += &format!("\ntrans {}_invariants_propagation {{\n", skillset.name());
        out += &format!("\t{} = Lock and\n", skillset_var(skillset),);
        // first
        let skill = skillset.get(first.id().0).unwrap();
        let guard = first.guard();
        out += &format!(
            "\tif {} = Running and not {} then\n",
            skill_var(skillset, skill),
            expr_to_tatam(skillset, guard)
        );
        out += &format!(
            "\t\t|{}|(\n",
            effects_resources(first.effects())
                .iter()
                .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
                .fold(skill_var(skillset, skill), |acc, res| format!(
                    "{}, {}",
                    acc, res
                ))
        );
        out += &format!("\t\t\t{}' = InvariantFailure\n", skill_var(skillset, skill),);
        for effect in first.effects().iter() {
            let resource = skillset.get(effect.resource().resolved()).unwrap();
            let state = skillset.get(effect.state().resolved()).unwrap();
            out += &format!(
                "\t\t\tand {}' = {}\n",
                resource_var(skillset, resource),
                resource_state(skillset, state)
            );
        }
        out += "\t\t)\n";
        // Others
        for invariant in others.iter() {
            let skill = skillset.get(invariant.id().0).unwrap();
            let guard = invariant.guard();
            out += &format!(
                "\telif {} = Running and not {} then\n",
                skill_var(skillset, skill),
                expr_to_tatam(skillset, guard)
            );
            out += &format!(
                "\t\t|{}|(\n",
                effects_resources(invariant.effects())
                    .iter()
                    .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
                    .fold(skill_var(skillset, skill), |acc, res| format!(
                        "{}, {}",
                        acc, res
                    ))
            );
            out += &format!("\t\t\t{}' = InvariantFailure\n", skill_var(skillset, skill),);
            for effect in invariant.effects().iter() {
                let resource = skillset.get(effect.resource().resolved()).unwrap();
                let state = skillset.get(effect.state().resolved()).unwrap();
                out += &format!(
                    "\t\t\tand {}' = {}\n",
                    resource_var(skillset, resource),
                    resource_state(skillset, state)
                );
            }
            out += "\t\t)\n"
        }
        // Else
        out += "\telse\n";
        out += &format!(
            "\t\t|{}|(\n",
            skillset
                .skills()
                .iter()
                .fold(skillset_var(skillset), |acc, skill| format!(
                    "{}, {}",
                    acc,
                    skill_var(skillset, skill)
                ))
        );
        for skill in skillset.skills().iter() {
            out += &format!("\t\t\tif {} = InvariantFailure or {} = Success or {} = Failure or {} = Interrupted then\n", skill_var(skillset, skill), skill_var(skillset, skill), skill_var(skillset, skill), skill_var(skillset, skill));
            out += &format!("\t\t\t\t{}' = Idle\n", skill_var(skillset, skill));
            out += "\t\t\telse\n";
            out += &format!(
                "\t\t\t\t{}' = {}\n",
                skill_var(skillset, skill),
                skill_var(skillset, skill)
            );
            out += "\t\t\tend and\n";
        }
        out += &format!("\t\t\t{}' = Free\n", skillset_var(skillset));
        out += "\t\t)\n";
        out += "\tend\n";
        out += "}\n";
    }
    out
}
