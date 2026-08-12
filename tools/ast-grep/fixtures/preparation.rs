pub fn prepare_manifest(records: &[Record]) -> PreparedManifest {
    let mut output = Vec::with_capacity(records.len());
    for record in records {
        output.push(prepare_record(record));
    }
    PreparedManifest { output }
}
