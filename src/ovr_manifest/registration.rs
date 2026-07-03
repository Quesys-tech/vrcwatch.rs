use openvr_sys as ovr_sys;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::process;

pub(super) fn configure_current_process(app_key: &str) -> Result<(), String> {
    let app_key = c_app_key(app_key)?;
    let registration = OpenVrManifestRegistration::load()?;

    registration.identify_current_process(&app_key)?;
    registration.set_auto_launch(&app_key, true)
}

struct OpenVrManifestRegistration(&'static ovr_sys::VR_IVRApplications_FnTable);

impl OpenVrManifestRegistration {
    fn load() -> Result<Self, String> {
        let interface_name = applications_interface_name();

        let mut init_error = ovr_sys::EVRInitError_VRInitError_None;
        let ptr = unsafe {
            ovr_sys::VR_GetGenericInterface(
                interface_name.as_ptr() as *const c_char,
                &mut init_error,
            )
        } as *const ovr_sys::VR_IVRApplications_FnTable;

        if init_error != ovr_sys::EVRInitError_VRInitError_None || ptr.is_null() {
            return Err(format!(
                "VR_GetGenericInterface(IVRApplications) failed: {}",
                init_error_name(init_error)
            ));
        }

        Ok(Self(unsafe { &*ptr }))
    }

    fn identify_current_process(&self, app_key: &CStr) -> Result<(), String> {
        let identify_application = self
            .0
            .IdentifyApplication
            .ok_or_else(|| "IVRApplications::IdentifyApplication is unavailable".to_owned())?;
        let err = unsafe { identify_application(process::id(), app_key.as_ptr() as *mut c_char) };
        self.ensure_success("IVRApplications::IdentifyApplication", err)
    }

    fn set_auto_launch(&self, app_key: &CStr, enabled: bool) -> Result<(), String> {
        let set_auto_launch = self
            .0
            .SetApplicationAutoLaunch
            .ok_or_else(|| "IVRApplications::SetApplicationAutoLaunch is unavailable".to_owned())?;
        let err = unsafe { set_auto_launch(app_key.as_ptr() as *mut c_char, enabled) };
        self.ensure_success("IVRApplications::SetApplicationAutoLaunch", err)
    }

    fn ensure_success(
        &self,
        operation: &str,
        error: ovr_sys::EVRApplicationError,
    ) -> Result<(), String> {
        application_result(operation, error, |error| self.error_name(error))
    }

    fn error_name(&self, error: ovr_sys::EVRApplicationError) -> String {
        if let Some(get_error_name) = self.0.GetApplicationsErrorNameFromEnum {
            let ptr = unsafe { get_error_name(error) };
            if !ptr.is_null() {
                return unsafe { CStr::from_ptr(ptr) }
                    .to_string_lossy()
                    .into_owned();
            }
        }

        format!("EVRApplicationError({error})")
    }
}

fn applications_interface_name() -> Vec<u8> {
    let mut interface_name = Vec::from(b"FnTable:".as_ref());
    interface_name.extend_from_slice(ovr_sys::IVRApplications_Version);
    interface_name
}

fn c_app_key(app_key: &str) -> Result<CString, String> {
    CString::new(app_key).map_err(|e| format!("Invalid OpenVR app key: {e}"))
}

fn application_result(
    operation: &str,
    error: ovr_sys::EVRApplicationError,
    error_name: impl FnOnce(ovr_sys::EVRApplicationError) -> String,
) -> Result<(), String> {
    if error == ovr_sys::EVRApplicationError_VRApplicationError_None {
        Ok(())
    } else {
        Err(format!("{operation} failed: {}", error_name(error)))
    }
}

fn init_error_name(error: ovr_sys::EVRInitError) -> String {
    let ptr = unsafe { ovr_sys::VR_GetVRInitErrorAsSymbol(error) };
    if ptr.is_null() {
        format!("EVRInitError({error})")
    } else {
        unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_ivr_applications_fn_table_interface_name() {
        let mut expected = Vec::from(b"FnTable:".as_ref());
        expected.extend_from_slice(ovr_sys::IVRApplications_Version);

        assert_eq!(applications_interface_name(), expected);
    }

    #[test]
    fn application_result_accepts_no_error() {
        let result = application_result(
            "IVRApplications::IdentifyApplication",
            ovr_sys::EVRApplicationError_VRApplicationError_None,
            |_| panic!("error name should not be requested for success"),
        );

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn application_result_formats_error_name() {
        let result = application_result(
            "IVRApplications::SetApplicationAutoLaunch",
            ovr_sys::EVRApplicationError_VRApplicationError_InvalidManifest,
            |_| "VRApplicationError_InvalidManifest".to_owned(),
        );

        assert_eq!(
            result,
            Err(
                "IVRApplications::SetApplicationAutoLaunch failed: VRApplicationError_InvalidManifest"
                    .to_owned()
            )
        );
    }

    #[test]
    fn identify_current_process_reports_missing_function() {
        let registration = registration_with_empty_table();
        let app_key = CString::new("tech.qsys.test").unwrap();

        assert_eq!(
            registration.identify_current_process(&app_key),
            Err("IVRApplications::IdentifyApplication is unavailable".to_owned())
        );
    }

    #[test]
    fn set_auto_launch_reports_missing_function() {
        let registration = registration_with_empty_table();
        let app_key = CString::new("tech.qsys.test").unwrap();

        assert_eq!(
            registration.set_auto_launch(&app_key, true),
            Err("IVRApplications::SetApplicationAutoLaunch is unavailable".to_owned())
        );
    }

    #[test]
    fn ensure_success_uses_numeric_fallback_without_error_name_function() {
        let registration = registration_with_empty_table();

        assert_eq!(
            registration.ensure_success(
                "IVRApplications::SetApplicationAutoLaunch",
                ovr_sys::EVRApplicationError_VRApplicationError_InvalidManifest,
            ),
            Err(
                "IVRApplications::SetApplicationAutoLaunch failed: EVRApplicationError(107)"
                    .to_owned()
            )
        );
    }

    #[test]
    fn rejects_app_key_with_nul_byte_before_loading_openvr() {
        assert!(c_app_key("tech.qsys\0vrcwatch-rs")
            .unwrap_err()
            .starts_with("Invalid OpenVR app key:"));
    }

    fn registration_with_empty_table() -> OpenVrManifestRegistration {
        OpenVrManifestRegistration(Box::leak(Box::new(
            ovr_sys::VR_IVRApplications_FnTable::default(),
        )))
    }
}
