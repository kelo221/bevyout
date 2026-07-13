//! Enable-parent resolution for target-cell references.

use super::*;

pub(crate) fn resolve_initially_enabled(
    reference_form_id: u32,
    references: &HashMap<u32, ReferenceRecord>,
) -> std::result::Result<bool, EnableResolutionError> {
    fn visit(
        form_id: u32,
        references: &HashMap<u32, ReferenceRecord>,
        visiting: &mut Vec<u32>,
        memo: &mut HashMap<u32, bool>,
    ) -> std::result::Result<bool, EnableResolutionError> {
        if let Some(enabled) = memo.get(&form_id) {
            return Ok(*enabled);
        }
        if visiting.contains(&form_id) {
            return Err(EnableResolutionError::Cycle(form_id));
        }
        let reference = references
            .get(&form_id)
            .ok_or(EnableResolutionError::Unresolved(form_id))?;
        visiting.push(form_id);
        let enabled = if let Some(enable_parent) = reference.enable_parent {
            let parent_enabled = visit(
                enable_parent.parent_reference_form_id,
                references,
                visiting,
                memo,
            )?;
            if enable_parent.is_inverted() {
                !parent_enabled
            } else {
                parent_enabled
            }
        } else {
            reference.flags & RECORD_DISABLED == 0
        };
        visiting.pop();
        memo.insert(form_id, enabled);
        Ok(enabled)
    }

    visit(
        reference_form_id,
        references,
        &mut Vec::new(),
        &mut HashMap::new(),
    )
}
