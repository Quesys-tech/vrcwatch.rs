use openvr_sys as ovr_sys;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::process;

pub(super) fn configure_current_process(app_key: &str) -> Result<(), String> {
    let app_key = c_app_key(app_key)?;
    let registration = OpenVrManifestRegistration::load()?;

    registration.identify_current_process(&app_key)?;
    registration.set_auto_launch(&app_key, true)
}

pub(super) fn registered_binary_path(app_key: &str) -> Result<PathBuf, String> {
    let app_key = c_app_key(app_key)?;
    let registration = OpenVrManifestRegistration::load()?;
    let path = registration.application_property_string(
        &app_key,
        ovr_sys::EVRApplicationProperty_VRApplicationProperty_BinaryPath_String,
    )?;

    Ok(PathBuf::from(path))
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

    fn application_property_string(
        &self,
        app_key: &CStr,
        property: ovr_sys::EVRApplicationProperty,
    ) -> Result<String, String> {
        let get_property = self.0.GetApplicationPropertyString.ok_or_else(|| {
            "IVRApplications::GetApplicationPropertyString is unavailable".to_owned()
        })?;
        let mut error = ovr_sys::EVRApplicationError_VRApplicationError_None;
        let required_length = unsafe {
            get_property(
                app_key.as_ptr().cast_mut(),
                property,
                std::ptr::null_mut(),
                0,
                &mut error,
            )
        };

        if error != ovr_sys::EVRApplicationError_VRApplicationError_None
            && error != ovr_sys::EVRApplicationError_VRApplicationError_BufferTooSmall
        {
            return Err(format!(
                "IVRApplications::GetApplicationPropertyString failed: {}",
                self.error_name(error)
            ));
        }
        if required_length == 0 {
            return Err(
                "IVRApplications::GetApplicationPropertyString returned an empty value".to_owned(),
            );
        }

        let mut buffer = vec![0_u8; required_length as usize];
        error = ovr_sys::EVRApplicationError_VRApplicationError_None;
        unsafe {
            get_property(
                app_key.as_ptr().cast_mut(),
                property,
                buffer.as_mut_ptr().cast(),
                required_length,
                &mut error,
            )
        };
        self.ensure_success("IVRApplications::GetApplicationPropertyString", error)?;

        CStr::from_bytes_until_nul(&buffer)
            .map(|value| value.to_string_lossy().into_owned())
            .map_err(|e| format!("Invalid application property string: {e}"))
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
    fn application_property_string_reports_missing_function() {
        let registration = registration_with_empty_table();
        let app_key = CString::new("tech.qsys.test").unwrap();

        assert_eq!(
            registration.application_property_string(
                &app_key,
                ovr_sys::EVRApplicationProperty_VRApplicationProperty_BinaryPath_String,
            ),
            Err("IVRApplications::GetApplicationPropertyString is unavailable".to_owned())
        );
    }

    #[test]
    fn application_property_string_reads_the_returned_value() {
        let table = ovr_sys::VR_IVRApplications_FnTable {
            GetApplicationPropertyString: Some(test_application_property_string),
            ..Default::default()
        };
        let registration = OpenVrManifestRegistration(Box::leak(Box::new(table)));
        let app_key = CString::new("tech.qsys.test").unwrap();

        assert_eq!(
            registration.application_property_string(
                &app_key,
                ovr_sys::EVRApplicationProperty_VRApplicationProperty_BinaryPath_String,
            ),
            Ok(r"C:\Tools\vrcwatch-rs.exe".to_owned())
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

    unsafe extern "C" fn test_application_property_string(
        _app_key: *mut c_char,
        _property: ovr_sys::EVRApplicationProperty,
        buffer: *mut c_char,
        buffer_length: u32,
        error: *mut ovr_sys::EVRApplicationError,
    ) -> u32 {
        const VALUE: &[u8] = b"C:\\Tools\\vrcwatch-rs.exe\0";

        if buffer_length < VALUE.len() as u32 {
            unsafe {
                *error = ovr_sys::EVRApplicationError_VRApplicationError_BufferTooSmall;
            }
            return VALUE.len() as u32;
        }

        unsafe {
            std::ptr::copy_nonoverlapping(VALUE.as_ptr().cast(), buffer, VALUE.len());
            *error = ovr_sys::EVRApplicationError_VRApplicationError_None;
        }
        VALUE.len() as u32
    }
}
