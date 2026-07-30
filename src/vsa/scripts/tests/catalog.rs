use super::*;
use crate::vsa::content_index::PluginSource;
use crate::vsa::record_stream::walk_resolved_records;

fn record(signature: &[u8; 4], flags: u32, form_id: u32, data: &[u8]) -> Vec<u8> {
    let mut out = signature.to_vec();
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(&flags.to_le_bytes());
    out.extend_from_slice(&form_id.to_le_bytes());
    out.extend_from_slice(&[0; 8]);
    out.extend_from_slice(data);
    out
}

fn tes4() -> Vec<u8> {
    record(b"TES4", 0, 0, b"")
}

/// A malformed record of a type that never carries scripts (WEAP) must
/// not register itself (or a later override) as a script owner.
#[test]
fn malformed_unrelated_record_does_not_register_an_owner() {
    let mut payload = Vec::new();
    payload.extend_from_slice(b"EDID");
    payload.extend_from_slice(&5u16.to_le_bytes());
    payload.extend_from_slice(b"weap\0");
    payload.extend_from_slice(b"BA"); // truncated subrecord header
    let mut plugin = tes4();
    plugin.extend_from_slice(&record(b"WEAP", 0, 0x500, &payload));
    let sources = [PluginSource {
        name: "broken.esp",
        bytes: &plugin,
    }];

    let mut builder = ScriptCatalogBuilder::default();
    walk_resolved_records(&sources, |envelope| builder.observe(envelope)).unwrap();
    assert!(builder.owners.get(0x500).is_none());
}
