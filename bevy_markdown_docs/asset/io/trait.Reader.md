[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Trait Reader 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#97)

```rust
pub trait Reader:
    AsyncRead
    + Unpin
    + Send
    + Sync {
    // Required method
    fn seekable(
        &mut self,
    ) -> Result<&mut dyn SeekableReader, ReaderNotSeekableError>;

    // Provided method
    fn read_to_end<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
    ) -> StackFuture<'a, Result<usize, Error>, bevy_asset::::io::Reader::read_to_end::{constant#0}> ⓘ { ... }
}
```

A type returned from [`AssetReader::read`](trait.AssetReader.html#tymethod.read "method bevy::asset::io::AssetReader::read"), which is used to read the contents of a file (or virtual file) corresponding to an asset.

This is essentially a trait alias for types implementing [`AsyncRead`](../../tasks/futures_lite/trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") and [`AsyncSeek`](../../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek"). The only reason a blanket implementation is not provided for applicable types is to allow implementors to override the provided implementation of [`Reader::read_to_end`](trait.Reader.html#method.read_to_end "method bevy::asset::io::Reader::read_to_end").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#139)

#### fn [seekable](#tymethod.seekable)( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn [SeekableReader](trait.SeekableReader.html "trait bevy::asset::io::SeekableReader"), [ReaderNotSeekableError](struct.ReaderNotSeekableError.html "struct bevy::asset::io::ReaderNotSeekableError")\>

Casts this [`Reader`](trait.Reader.html "trait bevy::asset::io::Reader") as a [`SeekableReader`](trait.SeekableReader.html "trait bevy::asset::io::SeekableReader"), which layers on [`AsyncSeek`](../../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") functionality. Returns [`Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") if this [`Reader`](trait.Reader.html "trait bevy::asset::io::Reader") supports seeking. Otherwise returns [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err").

Implementers of [`Reader`](trait.Reader.html "trait bevy::asset::io::Reader") are highly encouraged to provide this functionality, as it makes the reader compatible with “seeking” [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader") implementations.

[`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader") implementations that call this are encouraged to provide fallback behavior when it fails, such as reading into a seek-able [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") (or [`AsyncSeek`](../../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek")\-able [`VecReader`](struct.VecReader.html "struct bevy::asset::io::VecReader")):

```rust
let mut fallback_reader;
let reader = match reader.seekable() {
    Ok(seek) => seek,
    Err(_) => {
        fallback_reader = VecReader::new(Vec::new());
        reader.read_to_end(&mut fallback_reader.bytes).await.unwrap();
        &mut fallback_reader
    }
};
reader.seek(SeekFrom::Start(10)).await.unwrap();
```

## Provided Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#104-107)

#### fn [read\_to\_end](#method.read_to_end)<'a>( &'a mut self, buf: &'a mut [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, ) -> [StackFuture](struct.StackFuture.html "struct bevy::asset::io::StackFuture")<'a, [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>, bevy\_asset::::io::Reader::read\_to\_end::{constant#0}> [ⓘ](#)

Reads the entire contents of this reader and appends them to a vec.

##### Note for implementors

You should override the provided implementation if you can fill up the buffer more efficiently than the default implementation, which calls `poll_read` repeatedly to fill up the buffer 32 bytes at a time.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/asset/custom\_asset.rs ([line 46](../../../src/custom_asset/custom_asset.rs.html#46))

```rust
39    async fn load(
40        &self,
41        reader: &mut dyn Reader,
42        _settings: &(),
43        _load_context: &mut LoadContext<'_>,
44    ) -> Result<Self::Asset, Self::Error> {
45        let mut bytes = Vec::new();
46        reader.read_to_end(&mut bytes).await?;
47        let custom_asset = ron::de::from_bytes::<CustomAsset>(&bytes)?;
48        Ok(custom_asset)
49    }
50
51    fn extensions(&self) -> &[&str] {
52        &["custom"]
53    }
54}
55
56#[derive(Asset, TypePath, Debug)]
57struct Blob {
58    bytes: Vec<u8>,
59}
60
61#[derive(Default, TypePath)]
62struct BlobAssetLoader;
63
64/// Possible errors that can be produced by [`BlobAssetLoader`]
65#[non_exhaustive]
66#[derive(Debug, Error)]
67enum BlobAssetLoaderError {
68    /// An [IO](std::io) Error
69    #[error("Could not load file: {0}")]
70    Io(#[from] std::io::Error),
71}
72
73impl AssetLoader for BlobAssetLoader {
74    type Asset = Blob;
75    type Settings = ();
76    type Error = BlobAssetLoaderError;
77
78    async fn load(
79        &self,
80        reader: &mut dyn Reader,
81        _settings: &(),
82        _load_context: &mut LoadContext<'_>,
83    ) -> Result<Self::Asset, Self::Error> {
84        info!("Loading Blob...");
85        let mut bytes = Vec::new();
86        reader.read_to_end(&mut bytes).await?;
87
88        Ok(Blob { bytes })
89    }
```

Hide additional examples

examples/asset/processing/asset\_processing.rs ([line 91](../../../src/asset_processing/asset_processing.rs.html#91))

```rust
84    async fn load(
85        &self,
86        reader: &mut dyn Reader,
87        settings: &TextSettings,
88        _load_context: &mut LoadContext<'_>,
89    ) -> Result<Text, Self::Error> {
90        let mut bytes = Vec::new();
91        reader.read_to_end(&mut bytes).await?;
92        let value = if let Some(ref text) = settings.text_override {
93            text.clone()
94        } else {
95            String::from_utf8(bytes).unwrap()
96        };
97        Ok(Text(value))
98    }
99
100    fn extensions(&self) -> &[&str] {
101        &["txt"]
102    }
103}
104
105#[derive(Serialize, Deserialize)]
106struct CoolTextRon {
107    text: String,
108    dependencies: Vec<String>,
109    embedded_dependencies: Vec<String>,
110    dependencies_with_settings: Vec<(String, TextSettings)>,
111}
112
113#[derive(Asset, TypePath, Debug)]
114struct CoolText {
115    text: String,
116    #[expect(
117        dead_code,
118        reason = "Used to show that our assets can hold handles to other assets"
119    )]
120    dependencies: Vec<Handle<Text>>,
121}
122
123#[derive(Default, TypePath)]
124struct CoolTextLoader;
125
126#[derive(Debug, Error)]
127enum CoolTextLoaderError {
128    #[error(transparent)]
129    Io(#[from] std::io::Error),
130    #[error(transparent)]
131    RonSpannedError(#[from] ron::error::SpannedError),
132    #[error(transparent)]
133    LoadDirectError(#[from] bevy::asset::LoadDirectError),
134}
135
136impl AssetLoader for CoolTextLoader {
137    type Asset = CoolText;
138    type Settings = ();
139    type Error = CoolTextLoaderError;
140
141    async fn load(
142        &self,
143        reader: &mut dyn Reader,
144        _settings: &Self::Settings,
145        load_context: &mut LoadContext<'_>,
146    ) -> Result<CoolText, Self::Error> {
147        let mut bytes = Vec::new();
148        reader.read_to_end(&mut bytes).await?;
149        let ron: CoolTextRon = ron::de::from_bytes(&bytes)?;
150        let mut base_text = ron.text;
151        for embedded in ron.embedded_dependencies {
152            let loaded = load_context
153                .load_builder()
154                .load_value::<Text>(&embedded)
155                .await?;
156            base_text.push_str(&loaded.get().0);
157        }
158        for (path, settings_override) in ron.dependencies_with_settings {
159            let loaded = load_context
160                .load_builder()
161                .with_settings(move |settings| {
162                    *settings = settings_override.clone();
163                })
164                .load_value::<Text>(&path)
165                .await?;
166            base_text.push_str(&loaded.get().0);
167        }
168        Ok(CoolText {
169            text: base_text,
170            dependencies: ron
171                .dependencies
172                .iter()
173                .map(|p| load_context.load(p))
174                .collect(),
175        })
176    }
```

examples/asset/asset\_saving\_with\_subassets.rs ([line 213](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#213))

```rust
206    async fn load(
207        &self,
208        reader: &mut dyn Reader,
209        _settings: &Self::Settings,
210        load_context: &mut LoadContext<'_>,
211    ) -> Result<Self::Asset, Self::Error> {
212        let mut bytes = vec![];
213        reader.read_to_end(&mut bytes).await?;
214
215        let serialized: SerializableManyBoxes = ron::de::from_bytes(&bytes)?;
216
217        // Add the boxes as subassets.
218        let mut result_boxes = vec![];
219        for (index, one_box) in serialized.boxes.into_iter().enumerate() {
220            result_boxes.push(load_context.add_labeled_asset(index.to_string(), one_box));
221        }
222
223        Ok(ManyBoxes {
224            boxes: result_boxes,
225        })
226    }
```

examples/asset/asset\_decompression.rs ([line 64](../../../src/asset_decompression/asset_decompression.rs.html#64))

```rust
43    async fn load(
44        &self,
45        reader: &mut dyn Reader,
46        _settings: &(),
47        load_context: &mut LoadContext<'_>,
48    ) -> Result<Self::Asset, Self::Error> {
49        let compressed_path = load_context.path();
50        let file_name = compressed_path
51            .path()
52            .file_name()
53            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?
54            .to_string_lossy();
55        let uncompressed_file_name = file_name
56            .strip_suffix(".gz")
57            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?;
58        let contained_path = compressed_path
59            .resolve_embed_str(uncompressed_file_name)
60            .map_err(|_| GzAssetLoaderError::IndeterminateFilePath)?;
61
62        let mut bytes_compressed = Vec::new();
63
64        reader.read_to_end(&mut bytes_compressed).await?;
65
66        let mut decoder = GzDecoder::new(bytes_compressed.as_slice());
67
68        let mut bytes_uncompressed = Vec::new();
69
70        decoder.read_to_end(&mut bytes_uncompressed)?;
71
72        // Now that we have decompressed the asset, let's pass it back to the
73        // context to continue loading
74
75        let mut reader = VecReader::new(bytes_uncompressed);
76
77        let uncompressed = load_context
78            .load_builder()
79            .load_untyped_value_from_reader(contained_path, &mut reader)
80            .await?;
81
82        Ok(GzAsset { uncompressed })
83    }
```

## Trait Implementations

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#155)

### impl [Reader](trait.Reader.html "trait bevy::asset::io::Reader") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reader](trait.Reader.html "trait bevy::asset::io::Reader") + '\_>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#156-159)

#### fn [read\_to\_end](trait.Reader.html#method.read_to_end)<'a>( &'a mut self, buf: &'a mut [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, ) -> [StackFuture](struct.StackFuture.html "struct bevy::asset::io::StackFuture")<'a, [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>, bevy\_asset::::io::{impl#4}::read\_to\_end::{constant#0}> [ⓘ](#)

Reads the entire contents of this reader and appends them to a vec. [Read more](trait.Reader.html#method.read_to_end)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#163)

#### fn [seekable](trait.Reader.html#tymethod.seekable)( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn [SeekableReader](trait.SeekableReader.html "trait bevy::asset::io::SeekableReader"), [ReaderNotSeekableError](struct.ReaderNotSeekableError.html "struct bevy::asset::io::ReaderNotSeekableError")\>

Casts this [`Reader`](trait.Reader.html "trait bevy::asset::io::Reader") as a [`SeekableReader`](trait.SeekableReader.html "trait bevy::asset::io::SeekableReader"), which layers on [`AsyncSeek`](../../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") functionality. Returns [`Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") if this [`Reader`](trait.Reader.html "trait bevy::asset::io::Reader") supports seeking. Otherwise returns [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err"). [Read more](trait.Reader.html#tymethod.seekable)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/file/file_asset.rs.html#23)

### impl [Reader](trait.Reader.html "trait bevy::asset::io::Reader") for [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/file/file_asset.rs.html#24)

#### fn [seekable](#tymethod.seekable)( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn [SeekableReader](trait.SeekableReader.html "trait bevy::asset::io::SeekableReader"), [ReaderNotSeekableError](struct.ReaderNotSeekableError.html "struct bevy::asset::io::ReaderNotSeekableError")\>

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#155)

### impl [Reader](trait.Reader.html "trait bevy::asset::io::Reader") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reader](trait.Reader.html "trait bevy::asset::io::Reader") + '\_>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#689)

### impl [Reader](trait.Reader.html "trait bevy::asset::io::Reader") for [SliceReader](struct.SliceReader.html "struct bevy::asset::io::SliceReader")<'\_>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/processor_gated.rs.html#141)

### impl [Reader](trait.Reader.html "trait bevy::asset::io::Reader") for [TransactionLockedReader](processor_gated/struct.TransactionLockedReader.html "struct bevy::asset::io::processor_gated::TransactionLockedReader")<'\_>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#640)

### impl [Reader](trait.Reader.html "trait bevy::asset::io::Reader") for [VecReader](struct.VecReader.html "struct bevy::asset::io::VecReader")

{"StackFuture<'a, Result<usize, Error>, bevy\_asset::::io::Reader::read\_to\_end::{constant#0}>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.StackFuture.html\\" title=\\"struct bevy::asset::io::StackFuture\\">StackFuture</a>&lt;'a, T, STACK\_SIZE&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, const STACK\_SIZE: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.StackFuture.html\\" title=\\"struct bevy::asset::io::StackFuture\\">StackFuture</a>&lt;'a, T, STACK\_SIZE&gt;</div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>","StackFuture<'a, Result<usize, Error>, bevy\_asset::::io::{impl#4}::read\_to\_end::{constant#0}>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.StackFuture.html\\" title=\\"struct bevy::asset::io::StackFuture\\">StackFuture</a>&lt;'a, T, STACK\_SIZE&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, const STACK\_SIZE: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.StackFuture.html\\" title=\\"struct bevy::asset::io::StackFuture\\">StackFuture</a>&lt;'a, T, STACK\_SIZE&gt;</div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>"}