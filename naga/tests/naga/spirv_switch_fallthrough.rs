#![cfg(all(feature = "glsl-in", spv_out))]

use naga::valid::{Capabilities, ValidationFlags, Validator};
use naga::SwitchValue;

fn validate_switch(
    case_body: &str,
    expected_values: &[SwitchValue],
    expected_empty_cases: &[usize],
    expected_fallthrough_cases: &[usize],
) {
    let source = format!(
        "#version 450\nlayout(local_size_x = 1) in;\nlayout(set = 0, binding = 0, std430) buffer Output {{ uint values[]; }};\nvoid main() {{ switch (values[0]) {{ {case_body} }} }}"
    );
    let module = naga::front::glsl::Frontend::default()
        .parse(
            &naga::front::glsl::Options::from(naga::ShaderStage::Compute),
            &source,
        )
        .unwrap_or_else(|error| panic!("{error:?}\n{source}"));

    let mut switch_count = 0;
    let mut check_switch = |statement: &naga::Statement| {
        if let naga::Statement::Switch { cases, .. } = statement {
            switch_count += 1;
            assert_eq!(
                cases.iter().map(|case| case.value).collect::<Vec<_>>(),
                expected_values
            );
            assert_eq!(
                cases
                    .iter()
                    .enumerate()
                    .filter_map(|(index, case)| case.body.is_empty().then_some(index))
                    .collect::<Vec<_>>(),
                expected_empty_cases
            );
            assert_eq!(
                cases
                    .iter()
                    .enumerate()
                    .filter_map(|(index, case)| case.fall_through.then_some(index))
                    .collect::<Vec<_>>(),
                expected_fallthrough_cases
            );
        }
        false
    };
    for (_, function) in module.functions.iter() {
        naga::proc::any_statement(&function.body, &mut check_switch);
    }
    for entry in &module.entry_points {
        naga::proc::any_statement(&entry.function.body, &mut check_switch);
    }
    assert_eq!(switch_count, 1);

    let info = Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .unwrap();
    let words = naga::back::spv::write_vec(
        &module,
        &info,
        &naga::back::spv::Options::default(),
        Some(&naga::back::spv::PipelineOptions {
            entry_point: "main".to_string(),
            shader_stage: naga::ShaderStage::Compute,
        }),
    )
    .unwrap();
    super::helpers::validate_spirv(&words);
}

#[cfg_attr(miri, ignore)]
#[test]
fn spirv_switch_grouped_labels_then_nonempty_fallthrough() {
    validate_switch(
        "case 0u: case 1u: values[1] = 3u; case 2u: values[1] += 5u; break; default: values[1] = 7u; break;",
        &[
            SwitchValue::U32(0),
            SwitchValue::U32(1),
            SwitchValue::U32(2),
            SwitchValue::Default,
        ],
        &[0],
        &[0, 1],
    );
}

#[cfg_attr(miri, ignore)]
#[test]
fn spirv_switch_grouped_default_then_nonempty_fallthrough() {
    validate_switch(
        "case 0u: values[1] = 1u; break; case 1u: default: values[1] = 3u; case 2u: values[1] += 5u; break;",
        &[
            SwitchValue::U32(0),
            SwitchValue::U32(1),
            SwitchValue::Default,
            SwitchValue::U32(2),
        ],
        &[1],
        &[1, 2],
    );
}

#[cfg_attr(miri, ignore)]
#[test]
fn spirv_switch_grouped_labels_then_break() {
    validate_switch(
        "case 0u: case 1u: values[1] = 3u; break; default: values[1] = 7u; break;",
        &[
            SwitchValue::U32(0),
            SwitchValue::U32(1),
            SwitchValue::Default,
        ],
        &[0],
        &[0],
    );
}
