//! Backend selection for the surface-lightmap transport.
//!
//! The CPU implementation remains the default/reference path. This module
//! keeps user-facing selection and capability errors at one seam while the
//! feature-gated Solari adapter feeds the same bake frontend.

use crate::cli::LightmapBackendPreference;
use anyhow::Result;
#[cfg(not(feature = "lightmap-gpu-solari"))]
use anyhow::bail;

#[cfg(feature = "lightmap-gpu-solari")]
pub(crate) mod solari;

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
    fn auto_and_cpu_select_the_reference_backend() {
        assert!(validate_backend(LightmapBackendPreference::Auto).is_ok());
        assert!(validate_backend(LightmapBackendPreference::Cpu).is_ok());
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
