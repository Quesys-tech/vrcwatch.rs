use dirs::config_local_dir;
use std::{
    env,
    path::{Path, PathBuf},
};
use tokio::fs::remove_dir_all;
use tracing::{error, info};
extern crate openvr;
use serde::Serialize;

mod registration;

const ORGANIZATION: &str = "tech.qsys";
const APPLICATION: &str = env!("CARGO_PKG_NAME");
const OVR_APP_KEY: &str = "tech.qsys.vrcwatch-rs";

pub async fn manifest_path() -> PathBuf {
    config_local_dir()
        .unwrap()
        .join(ORGANIZATION)
        .join(APPLICATION)
        .join("manifest.vrmanifest")
}

pub async fn status() {
    let context = unsafe { openvr::init(openvr::ApplicationType::Utility) }
        .expect("Unable in OpenVR initialization");
    let mut application = context
        .application()
        .expect("Unable to get OpenVR application");

    match application.is_application_installed(OVR_APP_KEY) {
        Ok(installed) => {
            if installed {
                info!("VRCWatch is installed in SteamVR.");
                check_registered_executable_path();
            } else {
                info!("VRCWatch is NOT installed in SteamVR.");
            }
        }
        Err(e) => {
            error!(error = ?e, "Error checking VRCWatch installation status");
        }
    }
}

fn check_registered_executable_path() {
    let current_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            error!(error = ?e, "Failed to get the current executable path");
            return;
        }
    };
    let registered_path = match registration::registered_binary_path(OVR_APP_KEY) {
        Ok(path) => path,
        Err(e) => {
            error!(error = %e, "Failed to get the executable path registered in SteamVR");
            return;
        }
    };

    if executable_paths_match(&registered_path, &current_path) {
        info!(
            registered_path = %registered_path.display(),
            current_path = %current_path.display(),
            "The registered executable path matches the current executable."
        );
    } else {
        error!(
            registered_path = %registered_path.display(),
            current_path = %current_path.display(),
            "The registered executable path does NOT match the current executable. Please uninstall and reinstall VRCWatch."
        );
    }
}

fn executable_paths_match(registered_path: &Path, current_path: &Path) -> bool {
    let registered_path =
        std::fs::canonicalize(registered_path).unwrap_or_else(|_| registered_path.to_path_buf());
    let current_path =
        std::fs::canonicalize(current_path).unwrap_or_else(|_| current_path.to_path_buf());

    registered_path == current_path
}

#[derive(Debug, Serialize)]
struct SteamVrManifest {
    applications: [SteamVrApplication; 1],
}

#[derive(Debug, Serialize)]
struct SteamVrApplication {
    source: &'static str,
    app_key: &'static str,
    launch_type: &'static str,

    #[cfg(target_os = "windows")]
    binary_path_windows: String,

    #[cfg(target_os = "linux")]
    binary_path_linux: String,

    #[cfg(target_os = "macos")]
    binary_path_osx: String,

    is_dashboard_overlay: bool,
    strings: SteamVrStrings,
}

#[derive(Debug, Serialize)]
struct SteamVrStrings {
    en_us: SteamVrLocalizedStrings,
}

#[derive(Debug, Serialize)]
struct SteamVrLocalizedStrings {
    name: &'static str,
    description: &'static str,
}

async fn create_manifest() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let manifest = SteamVrManifest {
        applications: [SteamVrApplication {
            source: "builtin",
            app_key: OVR_APP_KEY,
            launch_type: "binary",
            #[cfg(target_os = "windows")]
            binary_path_windows: env::current_exe()
                .expect("Failed to get executable path!")
                .to_str()
                .unwrap()
                .to_owned(),
            #[cfg(target_os = "linux")]
            binary_path_linux: env::current_exe()
                .expect("Failed to get executable path!")
                .to_str()
                .unwrap()
                .to_owned(),
            #[cfg(target_os = "macos")]
            binary_path_osx: env::current_exe()
                .expect("Failed to get executable path!")
                .to_str()
                .unwrap()
                .to_owned(),
            is_dashboard_overlay: true,
            strings: SteamVrStrings {
                en_us: SteamVrLocalizedStrings {
                    name: env!("CARGO_PKG_NAME"),
                    description: env!("CARGO_PKG_DESCRIPTION"),
                },
            },
        }],
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    let path = manifest_path().await;
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(&path, manifest_json)?;
    Ok(path)
}

pub async fn install() {
    let context = unsafe { openvr::init(openvr::ApplicationType::Utility) }
        .expect("Unable in OpenVR initialization");
    let mut application = context
        .application()
        .expect("Unable to get OpenVR application");
    match application.is_application_installed(OVR_APP_KEY) {
        Ok(installed) => {
            if installed {
                info!("VRCWatch is already installed in SteamVR.");
                info!("If executable path has changed, please uninstall and reinstall VRCWatch.");
            } else {
                let manifest_path = create_manifest().await.expect("Failed to create manifest");
                // Implementation for installing the manifest would go here
                info!(manifest_path = ?manifest_path, "Manifest created");
                match application.add_application_manifest(&manifest_path, false) {
                    Ok(_) => {
                        info!("VRCWatch has been installed in SteamVR.");
                        info!("If executable path has changed, please uninstall and reinstall VRCWatch.");
                    }
                    Err(openvr::errors::VRApplicationError::AppKeyAlreadyExists) => {
                        info!("VRCWatch is already registered in SteamVR.");
                    }
                    Err(e) => {
                        error!(error = ?e, "Failed to install VRCWatch");
                        return;
                    }
                }
            }

            match registration::configure_current_process(OVR_APP_KEY) {
                Ok(_) => {
                    info!("VRCWatch has been identified and enabled for SteamVR auto launch.")
                }
                Err(e) => error!(error = %e, "Failed to configure VRCWatch in SteamVR"),
            }
        }
        Err(e) => {
            error!(error = ?e, "Error checking VRCWatch installation status");
        }
    }
}

pub async fn uninstall() {
    let context = unsafe { openvr::init(openvr::ApplicationType::Utility) }
        .expect("Unable in OpenVR initialization");
    let mut application = context
        .application()
        .expect("Unable to get OpenVR application");

    match application.is_application_installed(OVR_APP_KEY) {
        Ok(installed) => {
            if installed {
                let path = manifest_path().await;
                if !path.exists() {
                    error!(manifest_path = ?path, "Manifest file does not exist at expected path");
                } else {
                    application
                        .remove_application_manifest(&path)
                        .expect("Failed to uninstall VRCWatch");
                    remove_dir_all(path.parent().unwrap())
                        .await
                        .expect("Failed to remove manifest directory");
                    info!("VRCWatch has been uninstalled from SteamVR.");
                }
            } else {
                info!("VRCWatch is not installed in SteamVR.");
            }
        }
        Err(e) => {
            error!(error = ?e, "Error checking VRCWatch installation status");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::executable_paths_match;
    use std::path::Path;

    #[test]
    fn executable_paths_match_for_identical_paths() {
        assert!(executable_paths_match(
            Path::new("test/path/vrcwatch-rs"),
            Path::new("test/path/vrcwatch-rs")
        ));
    }

    #[test]
    fn executable_paths_do_not_match_for_different_paths() {
        assert!(!executable_paths_match(
            Path::new("old/path/vrcwatch-rs"),
            Path::new("new/path/vrcwatch-rs")
        ));
    }
}
