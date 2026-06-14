extern crate openvr;

const OVR_APP_KEY: &str = "tech.qsys.vrcwatch";

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
                todo!("Uninstallation functionality is not implemented yet.");
            } else {
                println!("VRCWatch is not installed in SteamVR.");
            }
        }
        Err(e) => {
            eprintln!("Error checking VRCWatch installation status: {:?}", e);
        }
    }
}
