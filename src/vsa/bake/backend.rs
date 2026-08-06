//! Backend selection for the surface-lightmap transport.
//!
//! The CPU implementation remains the reference path. Auto selection prefers
//! the feature-gated Solari adapter and the shared bake frontend can retry on
//! CPU when that adapter fails during a run.

use crate::cli::LightmapBackendPreference;
use anyhow::Result;
#[cfg(not(feature = "lightmap-gpu-solari"))]
use anyhow::bail;

#[cfg(feature = "lightmap-gpu-solari")]
pub(crate) mod solari;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SelectedLightmapBackend {
    Cpu,
    Solari,
}

impl SelectedLightmapBackend {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Solari => "solari",
        }
    }
}

pub(crate) fn select_backend(preference: LightmapBackendPreference) -> SelectedLightmapBackend {
    match preference {
        LightmapBackendPreference::Cpu => SelectedLightmapBackend::Cpu,
        LightmapBackendPreference::Solari => SelectedLightmapBackend::Solari,
        LightmapBackendPreference::Auto => {
            #[cfg(feature = "lightmap-gpu-solari")]
            {
                SelectedLightmapBackend::Solari
            }
            #[cfg(not(feature = "lightmap-gpu-solari"))]
            {
                SelectedLightmapBackend::Cpu
            }
        }
    }
}

pub(crate) fn validate_backend(preference: LightmapBackendPreference) -> Result<()> {
    match preference {
        LightmapBackendPreference::Auto | LightmapBackendPreference::Cpu => Ok(()),
        LightmapBackendPreference::Solari => {
            #[cfg(feature = "lightmap-gpu-solari")]
            {
                Ok(())
            }
            #[cfg(not(feature = "lightmap-gpu-solari"))]
            bail!("lightmap Solari backend is not available in this build; use --bake-backend cpu")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_and_cpu_are_valid_and_auto_selects_available_backend() {
        assert!(validate_backend(LightmapBackendPreference::Auto).is_ok());
        assert!(validate_backend(LightmapBackendPreference::Cpu).is_ok());
        #[cfg(feature = "lightmap-gpu-solari")]
        assert_eq!(
            select_backend(LightmapBackendPreference::Auto),
            SelectedLightmapBackend::Solari
        );
        #[cfg(not(feature = "lightmap-gpu-solari"))]
        assert_eq!(
            select_backend(LightmapBackendPreference::Auto),
            SelectedLightmapBackend::Cpu
        );
    }

    #[test]
    fn explicit_solari_request_is_feature_gated() {
        #[cfg(feature = "lightmap-gpu-solari")]
        assert!(validate_backend(LightmapBackendPreference::Solari).is_ok());
        #[cfg(not(feature = "lightmap-gpu-solari"))]
        let error = validate_backend(LightmapBackendPreference::Solari)
            .unwrap_err()
            .to_string();
        #[cfg(not(feature = "lightmap-gpu-solari"))]
        assert!(error.contains("Solari backend is not available"));
        #[cfg(not(feature = "lightmap-gpu-solari"))]
        assert!(error.contains("--bake-backend cpu"));
    }
}
