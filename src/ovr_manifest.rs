use dirs::config_local_dir;
use std::{env, path::PathBuf};
use tokio::fs::remove_dir_all;
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
                println!("VRCWatch is installed in SteamVR.");
            } else {
                println!("VRCWatch is NOT installed in SteamVR.");
            }
        }
        Err(e) => {
            eprintln!("Error checking VRCWatch installation status: {:?}", e);
        }
    }
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
                println!("VRCWatch is already installed in SteamVR.");
                println!(
                    "If executable path has changed, please uninstall and reinstall VRCWatch."
                );
            } else {
                let manifest_path = create_manifest().await.expect("Failed to create manifest");
                // Implementation for installing the manifest would go here
                println!("Manifest created at: {:?}", manifest_path);
                match application.add_application_manifest(&manifest_path, false) {
                    Ok(_) => {
                        println!("VRCWatch has been installed in SteamVR.");
                        println!("If executable path has changed, please uninstall and reinstall VRCWatch.");
                    }
                    Err(openvr::errors::VRApplicationError::AppKeyAlreadyExists) => {
                        println!("VRCWatch is already registered in SteamVR.");
                    }
                    Err(e) => {
                        eprintln!("Failed to install VRCWatch: {e:?}");
                        return;
                    }
                }
            }

            match registration::configure_current_process(OVR_APP_KEY) {
                Ok(_) => {
                    println!("VRCWatch has been identified and enabled for SteamVR auto launch.")
                }
                Err(e) => eprintln!("Failed to configure VRCWatch in SteamVR: {e}"),
            }
        }
        Err(e) => {
            eprintln!("Error checking VRCWatch installation status: {e:?}");
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
                    eprintln!("Manifest file does not exist at expected path: {:?}", path);
                } else {
                    application
                        .remove_application_manifest(&path)
                        .expect("Failed to uninstall VRCWatch");
                    remove_dir_all(path.parent().unwrap())
                        .await
                        .expect("Failed to remove manifest directory");
                    println!("VRCWatch has been uninstalled from SteamVR.");
                }
            } else {
                println!("VRCWatch is not installed in SteamVR.");
            }
        }
        Err(e) => {
            eprintln!("Error checking VRCWatch installation status: {:?}", e);
        }
    }
}
