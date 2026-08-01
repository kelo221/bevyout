use super::*;

#[test]
fn catalog_is_sorted_and_content_addressed() {
    let modifiers = HashMap::from([
        (
            2,
            ImageSpaceModifier {
                form_id: 2,
                ..Default::default()
            },
        ),
        (
            1,
            ImageSpaceModifier {
                form_id: 1,
                ..Default::default()
            },
        ),
    ]);
    let catalog = build_image_space_modifier_catalog(&modifiers, "source");
    assert_eq!(catalog.revision, IMAGE_SPACE_MODIFIER_CATALOG_REVISION);
    assert_eq!(
        catalog
            .modifiers
            .iter()
            .map(|modifier| modifier.form_id)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    let root = std::env::temp_dir().join(format!("bevyout-imad-catalog-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let (path, hash) = write_image_space_modifier_catalog(&root, &catalog).unwrap();
    assert_eq!(path, "catalogs/source/image_space_modifiers.ron");
    assert!(!hash.is_empty());
    assert!(root.join(&path).is_file());
    let _ = fs::remove_dir_all(root);
}
