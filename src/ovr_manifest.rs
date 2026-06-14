extern crate openvr;

use openvr::errors::VRApplicationError;

const OVR_APP_KEY: &str = "tech.qsys.vrcwatch";

pub async fn status() -> Result<bool, VRApplicationError> {
    let context = unsafe { openvr::init(openvr::ApplicationType::Utility) }
        .expect("Unable in OpenVR initialization");
    let mut application = context
        .application()
        .expect("Unable to get OpenVR application");

    match application.is_application_installed(OVR_APP_KEY) {
        Ok(installed) => Ok(installed),
        Err(e) => Err(e.into()),
    }
}
