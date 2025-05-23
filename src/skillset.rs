use super::*;

pub fn skillset_to_tatam(skillset: &Skillset, composite_skill_names: &Vec<String>, label: bool) -> String {
    let mut out = String::new();
    let mut trans_names = vec![];

    out += "enum SkillsetState = { Free, Lock }\n";
    out += "enum SkillState = { Idle, Running, Interrupting, InvariantFailure, Success, Failure, Interrupted}\n";

    if !composite_skill_names.is_empty() {
        out += "enum SkillInterfaceState = { NoCall, AsynCall, WaitResult }\n";
        out += "enum SkillCallResults = { NoneRes, SuccessRes, FailureRes, InvaFailRes, InterruptionRes, StartFailureRes, SkillBusyRes }\n";
    }

    out += &resources_to_tatam(skillset);
    // out += &events_to_tatam(skillset);

    out += "// ==================== Skillset ====================\n";
    out += &format!("var {}: SkillsetState\n", skillset_var(skillset));
    out += &format!("\ninit init_{} {{\n", skillset.name());
    out += &format!("\t{} = Free\n", skillset_var(skillset),);
    out += "}\n";

    // Invariant
    let result = skillset_invariant_propagation(skillset, composite_skill_names, label);
    out += &result.model;
    trans_names.extend(result.transition_names);

    // Skills
    out += "\n// ==================== Skill ====================\n";

    for skill in skillset.skills().iter() {
        let result = skill_to_tatam(skillset, skill, composite_skill_names, label);
        out += &result.model;
        trans_names.extend(result.transition_names);
    }

    if !composite_skill_names.is_empty() {
        out += &skill_interface_to_tatam(skillset, composite_skill_names);
    }

    if label {
        out += &format!("\nenum {} = {{ {}", &trans_label_enum_name(), &trans_label_enum_no_value());
        for trans_name in &trans_names {
            out += &format!(", {}", &trans_label_enum_value(&trans_name));
        }
        out += "}\n\n";
        out += &format!("var {}: {}\n\n", &trans_label_variable(), &trans_label_enum_name());
        out += &format!("init transition_label_init {{{} = {}}}\n", &trans_label_variable(), &trans_label_enum_no_value());
    }

    out
}

fn skillset_invariant_propagation(skillset: &Skillset, composite_skill_names: &Vec<String>, label: bool) -> ModelTransNames {
    let invariants = skillset
        .skills()
        .iter()
        .flat_map(|s| s.invariants())
        .collect::<Vec<_>>();

    let mut out = String::new();
    let mut trans_names = vec![];

    if let Some((first, others)) = invariants.split_first() {
        out += &format!("\ntrans {}_invariants_propagation {{\n", skillset.name());
        trans_names.push(format!("{}_invariants_propagation", skillset.name()));
        out += &format!("\t{} = Lock and\n", skillset_var(skillset),);
        // first
        let skill = skillset.get(first.id().0).unwrap();
        let guard = first.guard();
        out += &format!(
            "\tif ({} = Running or {} = Interrupting) and not {} then\n",
            skill_var(skillset, skill),
            skill_var(skillset, skill),
            expr_to_tatam(skillset, guard)
        );
        out += &format!(
            "\t\t|{}",
            effects_resources(first.effects())
                .iter()
                .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
                .fold(skill_var(skillset, skill), |acc, res| format!(
                    "{}, {}",
                    acc, res
                ))
            );
        if label {
            out += &format!(", {}", &trans_label_variable())
        }
        out += "|(\n";

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
        if label {
            out += &format!(
                    "\t\t\tand {}' = {}\n",
                    &trans_label_variable(),
                    &trans_label_enum_value(&format!("{}_invariants_propagation", skillset.name()))
            );
        }
        out += "\t\t)\n";
        // Others
        for invariant in others.iter() {
            let skill = skillset.get(invariant.id().0).unwrap();
            let guard = invariant.guard();
            out += &format!(
                "\telif ({} = Running or {} = Interrupting) and not {} then\n",
                skill_var(skillset, skill),
                skill_var(skillset, skill),
                expr_to_tatam(skillset, guard)
            );
            out += &format!(
                "\t\t|{}",
                effects_resources(invariant.effects())
                    .iter()
                    .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
                    .fold(skill_var(skillset, skill), |acc, res| format!(
                        "{}, {}",
                        acc, res
                    ))
            );
            if label {
                out += &format!(", {}", &trans_label_variable())
            }
            out += "|(\n";

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
            if label {
                out += &format!(
                    "\t\t\tand {}' = {}\n",
                    &trans_label_variable(),
                    &trans_label_enum_value(&format!("{}_invariants_propagation", skillset.name()))
                );
            }
            out += "\t\t)\n"
        }
        // Else
        out += "\telse\n";
        out += &format!(
            "\t\t|{}",
            skillset
                .skills()
                .iter()
                .fold(skillset_var(skillset), |acc, skill| format!(
                    "{}, {}",
                    acc,
                    skill_var(skillset, skill)
                ))
        );
        for comp_name in composite_skill_names {
            for skill in skillset.skills() {
                out += &format!(", {}, {}", &interface_var(skillset, skill, &comp_name), &interface_result_var(skillset, skill, &comp_name));
            }
        }
        if label {
            out += &format!(", {}", &trans_label_variable());
        }
        out += "|(\n";
        for skill in skillset.skills().iter() {
            out += &format!("\t\t\tif {} = InvariantFailure or {} = Success or {} = Failure or {} = Interrupted then\n", skill_var(skillset, skill), skill_var(skillset, skill), skill_var(skillset, skill), skill_var(skillset, skill));
            out += &format!("\t\t\t\t{}' = Idle\n", skill_var(skillset, skill));
            for comp_name in composite_skill_names {
                out += &format!("\t\t\t\tand if {} = WaitResult then\n", &interface_var(skillset, skill, &comp_name));
                out += &format!("\t\t\t\t\t{}' = NoCall and\n", &interface_var(skillset, skill, &comp_name));
                out += &format!("\t\t\t\t\tif {} = InvariantFailure then\n\t\t\t\t\t\t{}' = InvaFailRes\n", &skill_var(skillset, skill), &interface_result_var(skillset, skill, &comp_name));
                out += &format!("\t\t\t\t\telif {} = Success then\n\t\t\t\t\t\t{}' = SuccessRes\n", &skill_var(skillset, skill), &interface_result_var(skillset, skill, &comp_name));
                out += &format!("\t\t\t\t\telif {} = Failure then\n\t\t\t\t\t\t{}' = FailureRes\n", &skill_var(skillset, skill), &interface_result_var(skillset, skill, &comp_name));
                out += &format!("\t\t\t\t\telif {} = Interrupted then\n\t\t\t\t\t\t{}' = InterruptionRes\n", &skill_var(skillset, skill), &interface_result_var(skillset, skill, &comp_name));
                out += &format!("\t\t\t\t\telse\n\t\t\t\t\t\t{}' = NoneRes\n", &interface_result_var(skillset, skill, &comp_name));
                out += "\t\t\t\t\tend\n";
                out += "\t\t\t\telse\n";
                out += &format!("\t\t\t\t\t{}' = {} and {}' = {}\n", &interface_var(skillset, skill, &comp_name), &interface_var(skillset, skill, &comp_name), &interface_result_var(skillset, skill, &comp_name), &interface_result_var(skillset, skill, &comp_name));
                out += "\t\t\t\tend\n";
            }
            out += "\t\t\telse\n";
            out += &format!(
                "\t\t\t\t{}' = {}\n",
                skill_var(skillset, skill),
                skill_var(skillset, skill)
            );
            for comp_name in composite_skill_names {
                out += &format!("\t\t\t\tand {}' = {} and {}' = {}\n", &interface_var(skillset, skill, &comp_name), &interface_var(skillset, skill, &comp_name), &interface_result_var(skillset, skill, &comp_name), &interface_result_var(skillset, skill, &comp_name));
            }
            out += "\t\t\tend and\n";
        }
        out += &format!("\t\t\t{}' = Free\n", skillset_var(skillset));
        if label {
            out += &format!(
                "\t\t\tand {}' = {}\n",
                &trans_label_variable(),
                &trans_label_enum_value(&format!("{}_invariants_propagation", skillset.name()))
            );
        }
        out += "\t\t)\n";
        out += "\tend\n";
        out += "}\n";
    }

    ModelTransNames {
        model: out,
        transition_names: trans_names,
    }
}
