[bevy](../index.html)::[diagnostic](index.html)

# Trait RegisterDiagnostic 

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#404)

```rust
pub trait RegisterDiagnostic {
    // Required method
    fn register_diagnostic(&mut self, diagnostic: Diagnostic) -> &mut Self;
}
```

Extend [`App`](../prelude/struct.App.html "struct bevy::prelude::App") with new `register_diagnostic` function.

## Required Methods

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#420)

#### fn [register\_diagnostic](#tymethod.register_diagnostic)(&mut self, diagnostic: [Diagnostic](struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic")) -> &mut Self

Register a new [`Diagnostic`](struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic") with an [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

Will initialize a [`DiagnosticsStore`](struct.DiagnosticsStore.html "struct bevy::diagnostic::DiagnosticsStore") if it doesn’t exist.

```rust
use bevy_app::App;
use bevy_diagnostic::{Diagnostic, DiagnosticsPlugin, DiagnosticPath, RegisterDiagnostic};

const UNIQUE_DIAG_PATH: DiagnosticPath = DiagnosticPath::const_new("foo/bar");

App::new()
    .register_diagnostic(Diagnostic::new(UNIQUE_DIAG_PATH))
    .add_plugins(DiagnosticsPlugin)
    .run();
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#433)

### impl [RegisterDiagnostic](trait.RegisterDiagnostic.html "trait bevy::diagnostic::RegisterDiagnostic") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#423)

### impl [RegisterDiagnostic](trait.RegisterDiagnostic.html "trait bevy::diagnostic::RegisterDiagnostic") for [SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")