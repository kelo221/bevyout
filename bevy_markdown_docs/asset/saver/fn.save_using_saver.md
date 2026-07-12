[bevy](../../index.html)::[asset](../index.html)::[saver](index.html)

# Function save\_using\_saver 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#529-535)

```rust
pub async fn save_using_saver<S>(
    asset_server: AssetServer,
    saver: &S,
    path: &AssetPath<'_>,
    asset: SavedAsset<'_, '_, <S as AssetSaver>::Asset>,
    settings: &<S as AssetSaver>::Settings,
) -> Result<(), SaveAssetError>where
    S: AssetSaver,
```

Saves `asset` to `path` using the provided `saver` and `settings`.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/asset/asset\_saving.rs ([lines 49-55](../../../src/asset_saving/asset_saving.rs.html#49-55))

```rust
38fn perform_save(
39    image_to_save: Res<ImageToSave>,
40    images: Res<Assets<Image>>,
41    asset_server: Res<AssetServer>,
42) {
43    let image = images.get(&image_to_save.0).unwrap();
44
45    let image = image.clone();
46    let asset_server = asset_server.clone();
47    IoTaskPool::get()
48        .spawn(async move {
49            match save_using_saver(
50                asset_server.clone(),
51                &ImageSaver,
52                &ASSET_PATH.into(),
53                SavedAsset::from_asset(&image),
54                &ImageSaverSettings::default(),
55            )
56            .await
57            {
58                Ok(()) => info!("Completed save of {ASSET_PATH}"),
59                Err(err) => error!("Failed to save asset: {err}"),
60            }
61        })
62        .detach();
63}
```

Hide additional examples

examples/asset/asset\_saving\_with\_subassets.rs ([lines 73-79](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#73-79))

```rust
46fn perform_save(boxes: Query<(&Sprite, &Transform), With<Box>>, asset_server: Res<AssetServer>) {
47    // First we extract all the data needed to produce an asset we can save.
48    let boxes = boxes
49        .iter()
50        .map(|(sprite, transform)| OneBox {
51            position: transform.translation.xy(),
52            color: sprite.color,
53        })
54        .collect::<Vec<_>>();
55
56    let asset_server = asset_server.clone();
57    IoTaskPool::get()
58        .spawn(async move {
59            // Build a `SavedAsset` instance from the boxes we extracted.
60            let mut builder = SavedAssetBuilder::new(asset_server.clone(), ASSET_PATH.into());
61            let mut many_boxes = ManyBoxes { boxes: vec![] };
62            for (index, one_box) in boxes.iter().enumerate() {
63                many_boxes
64                    .boxes
65                    .push(builder.add_labeled_asset_with_new_handle(
66                        index.to_string(),
67                        SavedAsset::from_asset(one_box),
68                    ));
69            }
70
71            let saved_asset = builder.build(&many_boxes);
72            // Save the asset using the provided saver.
73            match save_using_saver(
74                asset_server.clone(),
75                &ManyBoxesSaver,
76                &ASSET_PATH.into(),
77                saved_asset,
78                &(),
79            )
80            .await
81            {
82                Ok(()) => info!("Completed save of {ASSET_PATH}"),
83                Err(err) => error!("Failed to save asset: {err}"),
84            }
85        })
86        .detach();
87}
```