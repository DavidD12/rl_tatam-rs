use super::*;
use heck::ToSnakeCase;

pub fn resources_to_tatam(skillset: &Skillset) -> String {
    let mut out = "".to_string();

    out += "// ==================== Resouces ====================\n";
    for resource in skillset.resources() {
        // Enum
        out += &format!("\nenum {} = {{ ", resource_enum(skillset, resource));
        if let Some((first, others)) = resource.states().split_first() {
            out += &resource_state(skillset, first);
            for state in others {
                out += &format!(", {}", resource_state(skillset, state));
            }
        }
        out += " }\n";
        // Var
        out += &format!(
            "var {}: {}\n",
            resource_var(skillset, resource),
            resource_enum(skillset, resource)
        );
    }
    // Init
    out += &format!(
        "\ninit init_{}_resources {{\n",
        skillset.name().to_snake_case()
    );
    if let Some((first, others)) = skillset.resources().split_first() {
        let state = skillset.get(first.initial()).unwrap();
        out += &format!(
            "  {} = {}",
            resource_var(skillset, first),
            resource_state(skillset, state)
        );
        for resource in others {
            let state = skillset.get(resource.initial()).unwrap();
            out += &format!(
                " and {} = {}",
                resource_var(skillset, resource),
                resource_state(skillset, state)
            );
        }
        out += "\n}\n";
    }

    out
}

pub fn resource_to_tatam(skillset: &Skillset, resource: &Resource) -> String {
    let mut out = "".to_string();

    // Enum
    out += &format!("\nenum {}: {{", resource_enum(skillset, resource));
    if let Some((first, others)) = resource.states().split_first() {
        out += &resource_state(skillset, first);
        for state in others {
            out += &format!(", {}", resource_state(skillset, state));
        }
    }
    out += "}\n";
    // Var
    out += &format!(
        "var {}: {}\n",
        resource_var(skillset, resource),
        resource_enum(skillset, resource)
    );

    out
}
