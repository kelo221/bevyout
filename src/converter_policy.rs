//! Pure converter-selection policy shared by CLI adapters and feature tests.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ConverterBackend {
    #[default]
    Native,
    Blender,
}

impl ConverterBackend {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Blender => "blender",
        }
    }
}

pub(crate) const fn resolve_converter_backend(
    requested: Option<ConverterBackend>,
) -> ConverterBackend {
    match requested {
        Some(backend) => backend,
        None => ConverterBackend::Native,
    }
}
