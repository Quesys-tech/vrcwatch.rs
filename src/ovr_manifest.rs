use dirs::config_local_dir;
use std::path::PathBuf;
extern crate openvr;

const ORGANIZATION: &str = "tech.qsys";
const APPLICATION: &str = env!("CARGO_PKG_NAME");
const OVR_APP_KEY: &str = "tech.qsys.vrcwatch-rs";

pub async fn manifest_path() -> PathBuf {
    let dir = config_local_dir().unwrap();
    dir.join(ORGANIZATION)
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
            } else {
                todo!("Installation functionality is not implemented yet.");
            }
        }
        Err(e) => {
            eprintln!("Error checking VRCWatch installation status: {:?}", e);
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
