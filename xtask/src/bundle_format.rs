// Bundle format dispatch for the install-launch-ingest-query smoke harness.
//
// Per arch §Occupied Resources Bundle artifact names: smoke targets exactly
// these formats and no others. Each format has a per-platform install /
// launch / cleanup recipe (see smoke::run_smoke). The expected-* helpers
// mirror window::detect_*_backend in pulse-app/src/window.rs so the smoke
// can assert the launched bundle emits the right platform-specific span
// values in agent-latest.jsonl.

use std::path::Path;

use clap::ValueEnum;

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum BundleFormat {
    Msi,
    Dmg,
    AppImage,
    Deb,
}

impl BundleFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        match ext.as_str() {
            "msi" => Some(Self::Msi),
            "dmg" => Some(Self::Dmg),
            "appimage" => Some(Self::AppImage),
            "deb" => Some(Self::Deb),
            _ => None,
        }
    }

    pub fn expected_webview_backend(self) -> &'static str {
        match self {
            Self::Msi => "WebView2",
            Self::Dmg => "WKWebView",
            Self::AppImage | Self::Deb => "GTKWebKit",
        }
    }

    pub fn expected_tray_api(self) -> &'static str {
        match self {
            Self::Msi => "NotifyIcon",
            Self::Dmg => "NSStatusItem",
            Self::AppImage | Self::Deb => "AppIndicator",
        }
    }

    pub fn expected_wgpu_backend(self) -> &'static str {
        match self {
            Self::Msi => "dx12",
            Self::Dmg => "metal",
            Self::AppImage | Self::Deb => "vulkan",
        }
    }

    pub fn host_platform(self) -> &'static str {
        match self {
            Self::Msi => "windows",
            Self::Dmg => "macos",
            Self::AppImage | Self::Deb => "linux",
        }
    }

    pub fn matches_host(self) -> bool {
        let host = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            "unknown"
        };
        self.host_platform() == host
    }
}

impl std::str::FromStr for BundleFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "msi" => Ok(Self::Msi),
            "dmg" => Ok(Self::Dmg),
            "appimage" => Ok(Self::AppImage),
            "deb" => Ok(Self::Deb),
            other => Err(format!("unknown bundle format: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_path_recognizes_each_format() {
        assert_eq!(
            BundleFormat::from_path(Path::new("andromeda-pulse_0.1.0_x64-setup.msi")),
            Some(BundleFormat::Msi),
        );
        assert_eq!(
            BundleFormat::from_path(Path::new("andromeda-pulse_0.1.0_x64.dmg")),
            Some(BundleFormat::Dmg),
        );
        assert_eq!(
            BundleFormat::from_path(Path::new("andromeda-pulse_0.1.0_amd64.AppImage")),
            Some(BundleFormat::AppImage),
        );
        assert_eq!(
            BundleFormat::from_path(Path::new("andromeda-pulse_0.1.0_amd64.deb")),
            Some(BundleFormat::Deb),
        );
    }

    #[test]
    fn from_path_returns_none_on_unknown_extension() {
        assert_eq!(
            BundleFormat::from_path(Path::new("andromeda-pulse_0.1.0.tar.gz")),
            None,
        );
        assert_eq!(BundleFormat::from_path(Path::new("README.md")), None);
        assert_eq!(BundleFormat::from_path(Path::new("no-extension")), None);
    }

    #[test]
    fn expected_backends_align_with_pulse_app_window_module() {
        // Mirrors pulse-app/src/window.rs::detect_webview_backend /
        // detect_tray_api / detect_wgpu_backend (chunk #24 substrate). The
        // smoke harness asserts these literal strings appear in the
        // bundle's agent-latest.jsonl after boot.
        assert_eq!(BundleFormat::Msi.expected_webview_backend(), "WebView2");
        assert_eq!(BundleFormat::Dmg.expected_webview_backend(), "WKWebView");
        assert_eq!(
            BundleFormat::AppImage.expected_webview_backend(),
            "GTKWebKit"
        );
        assert_eq!(BundleFormat::Deb.expected_webview_backend(), "GTKWebKit");

        assert_eq!(BundleFormat::Msi.expected_tray_api(), "NotifyIcon");
        assert_eq!(BundleFormat::Dmg.expected_tray_api(), "NSStatusItem");
        assert_eq!(BundleFormat::AppImage.expected_tray_api(), "AppIndicator");
        assert_eq!(BundleFormat::Deb.expected_tray_api(), "AppIndicator");

        assert_eq!(BundleFormat::Msi.expected_wgpu_backend(), "dx12");
        assert_eq!(BundleFormat::Dmg.expected_wgpu_backend(), "metal");
        assert_eq!(BundleFormat::AppImage.expected_wgpu_backend(), "vulkan");
        assert_eq!(BundleFormat::Deb.expected_wgpu_backend(), "vulkan");
    }

    #[test]
    fn host_platform_matches_format_origin() {
        assert_eq!(BundleFormat::Msi.host_platform(), "windows");
        assert_eq!(BundleFormat::Dmg.host_platform(), "macos");
        assert_eq!(BundleFormat::AppImage.host_platform(), "linux");
        assert_eq!(BundleFormat::Deb.host_platform(), "linux");
    }

    #[test]
    fn from_str_round_trips_clap_value_enum_form() {
        // `BundleFormat::from_str` is defined by BOTH `clap::ValueEnum` and
        // `std::str::FromStr` — disambiguate via the `.parse::<T>()` form
        // (delegates to FromStr exclusively).
        assert_eq!("msi".parse::<BundleFormat>(), Ok(BundleFormat::Msi));
        assert_eq!("DMG".parse::<BundleFormat>(), Ok(BundleFormat::Dmg));
        assert_eq!(
            "appimage".parse::<BundleFormat>(),
            Ok(BundleFormat::AppImage)
        );
        assert_eq!("deb".parse::<BundleFormat>(), Ok(BundleFormat::Deb));
        assert!("rpm".parse::<BundleFormat>().is_err());
    }
}
