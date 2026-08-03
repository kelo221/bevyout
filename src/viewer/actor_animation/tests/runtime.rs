use super::*;

fn ctda(function_index: u16, comparison_value: f32, param1: u32) -> Vec<u8> {
    let mut bytes = vec![0u8; 20];
    bytes[4..8].copy_from_slice(&comparison_value.to_le_bytes());
    bytes[8..10].copy_from_slice(&function_index.to_le_bytes());
    bytes[12..16].copy_from_slice(&param1.to_le_bytes());
    bytes
}

#[test]
fn idle_ctda_boundary_reads_only_authoritative_runtime_facts() {
    let mut facts = idle_policy::IdleRuntimeFacts {
        weapon_out: true,
        last_idle_played: Some(7),
        ..Default::default()
    };
    facts.factions.insert(42);
    facts.equipped_item_form_ids.insert(99);
    let evaluator = RuntimeIdleConditions;
    for (function, param, expected) in [
        (101, 0, 1.0),
        (71, 42, 1.0),
        (77, 0, 37.0),
        (182, 99, 1.0),
        (451, 7, 1.0),
    ] {
        let result = evaluator.evaluate(&[ctda(function, expected, param)], 37, &facts);
        assert_eq!(result, IdleConditionOutcome::True, "function {function}");
    }
    assert_eq!(
        evaluator.evaluate(&[ctda(999, 1.0, 0)], 37, &facts),
        IdleConditionOutcome::Unevaluable
    );
}

#[test]
fn malformed_idle_ctda_is_never_implicitly_true() {
    let evaluator = RuntimeIdleConditions;
    assert_eq!(
        evaluator.evaluate(&[vec![0; 3]], 0, &Default::default()),
        IdleConditionOutcome::Unevaluable
    );
}
