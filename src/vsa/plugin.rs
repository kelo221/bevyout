//! Compatibility facade for the Fallout cell slice's attributed ESM4 reader.

pub(crate) use super::openmw_esm4::{
    ActorBaseConfig, BaseRecord, ParsedPlugin, PluginSource, RECORD_DELETED, RECORD_DISABLED,
    RecipeItemRecord, RecipeRecord, ReferenceKind, ReferenceRecord, SoundRecord,
    SoundReferenceRecord, parse_content_set, read_master_names,
};
