use super::*;

#[test]
fn source_rows_are_sorted_and_missing_backends_are_deterministic() {
    let corpus = CorpusInput {
        selected_pipeline: "native".into(),
        corpus_name: Some("synthetic".into()),
        rows: vec![
            CorpusRowInput {
                source: "B.nif".into(),
                source_path: None,
                native_report: None,
                native_output: None,
                native_cache_dir: None,
                native_cold_seconds: Some(1.0),
                native_warm_seconds: Some(0.1),
            },
            CorpusRowInput {
                source: "A.nif".into(),
                source_path: None,
                native_report: None,
                native_output: None,
                native_cache_dir: None,
                native_cold_seconds: None,
                native_warm_seconds: None,
            },
        ],
    };
    let report = build_report(corpus, Path::new(".")).unwrap();
    assert_eq!(report.rows[0].source, "A.nif");
    assert_eq!(report.rows[1].native.status, "missing");
    assert_eq!(report.summary.total, 2);
    assert_eq!(report.summary.native_failures, 2);
}
