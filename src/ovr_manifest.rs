extern crate openvr;

const OVR_APP_KEY: &str = "tech.qsys.vrcwatch";

pub async fn status(){
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
        },
        Err(e) => {
            eprintln!("Error checking VRCWatch installation status: {:?}", e);
        }
    }
}

pub async fn install() {
    println!("Installation functionality is not implemented yet.");
}

pub async fn uninstall() {
    println!("Uninstallation functionality is not implemented yet.");
}