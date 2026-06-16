//! The Settings Api is a trait that has all the functions that are used to read and write settings data.

#![allow(missing_docs)]

use crate::{math::Uuid, ApplicationType, QueryError, QueryMetadata};

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait SettingsApi {
    fn read_int(&self, key: &str) -> Option<i32>;
    fn read_float(&self, key: &str) -> Option<f32>;
    fn read_string(&self, key: &str) -> Option<String>;
    fn read_bool(&self, key: &str) -> Option<bool>;
    fn write_int(&self, key: &str, value: i32);
    fn write_float(&self, key: &str, value: f32);
    fn write_string(&self, key: &str, value: &str);
    fn write_bool(&self, key: &str, value: bool);
    fn project_path(&self) -> Option<String>;
    fn project_plugin_path(&self) -> Option<String>;
    fn application_type(&self) -> ApplicationType;
    fn set_has_crashed(&self);
    fn set_ui_disable(&self, disable: bool);
    fn read_scene_string(&self, key: &str) -> Option<String>;
    fn write_scene_string(&self, key: &str, value: &str) -> bool;
    fn exit_application(&self, force: bool);
    fn project_file_path(&self) -> Option<String>;
    fn publish_data(&self, data_id: &str, data: &[u8]);
    fn query_data(
        &self,
        data_id: &str,
        expected_data_size: i32,
    ) -> Result<(Vec<u8>, QueryMetadata), QueryError>;
    fn plugin_param(&self, key: &str) -> Option<String>;
    fn oden_version(&self) -> String;
    fn license_hash(&self) -> Option<String>;
    fn instance_uuid(&self) -> Option<Uuid>;
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_settings_api {
    ($interface:ident) => {
        #[inherent::inherent]
        impl $crate::settings_api::SettingsApi for $interface<'_> {
            /// Reads persistent `i32` data from the `key` identifier in the .vproj file.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let key_value = api.read_int("key");
            /// # }
            /// ```
            pub fn read_int(&self, key: &str) -> Option<i32> {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                if let Some(read_int) = unsafe { (*self.inner).readInt } {
                    let mut res = 0;
                    if unsafe { read_int(c_key.as_ptr(), &mut res as *mut _) } {
                        return Some(res);
                    }
                }

                None
            }

            /// Reads persistent `f32` data from the `key` identifier in the .vproj file.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let key_value = api.read_float("key");
            /// # };
            /// ```
            pub fn read_float(&self, key: &str) -> Option<f32> {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                if let Some(read_float) = unsafe { (*self.inner).readFloat } {
                    let mut res = 0.0;
                    if unsafe { read_float(c_key.as_ptr(), &mut res as *mut _) } {
                        return Some(res);
                    }
                }
                None
            }

            /// Reads persistent `String` data from the `key` identifier in the .vproj file.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let key_value = api.read_string("key");
            /// # }
            /// ```
            pub fn read_string(&self, key: &str) -> Option<String> {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                if let Some(read_string_ex) = unsafe { (*self.inner).readStringEx } {
                    let mut size = 0;

                    unsafe {
                        if !read_string_ex(c_key.as_ptr(), std::ptr::null_mut(), &mut size) {
                            return None;
                        }
                    }

                    let mut res = vec![0u8; size as usize];

                    if unsafe {
                        read_string_ex(
                            c_key.as_ptr(),
                            res.as_mut_slice().as_mut_ptr() as *mut _,
                            &mut size,
                        )
                    } {
                        return $crate::utils::utf8_from_raw_to_first_null_terminator(&res).ok();
                    }
                }
                None
            }

            /// Shortcut to read ints as bools
            pub fn read_bool(&self, key: &str) -> Option<bool> {
                if let Some(v) = self.read_int(key) {
                    Some(v == 1)
                } else {
                    None
                }
            }

            /// Writes persistent `i32` data to the `key` identifier in the .vproj file.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// api.write_int("key name", 1);
            /// # }
            /// ```
            pub fn write_int(&self, key: &str, value: i32) {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                if let Some(write_int) = unsafe { (*self.inner).writeInt } {
                    unsafe { write_int(c_key.as_ptr(), value) };
                }
            }

            /// Writes persistent `f32` data to the `key` identifier in the .vproj file.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// api.write_float("key name", 1.0);
            /// # }
            /// ```
            pub fn write_float(&self, key: &str, value: f32) {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                if let Some(write_float) = unsafe { (*self.inner).writeFloat } {
                    unsafe { write_float(c_key.as_ptr(), value) };
                }
            }

            /// Writes persistent `String` data to the `key` identifier in the .vproj file.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// api.write_string("key name", "string_value");
            /// # }
            /// ```
            pub fn write_string(&self, key: &str, value: &str) {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                let c_value = std::ffi::CString::new(value.trim_end_matches('\0')).unwrap();

                if let Some(write_string) = unsafe { (*self.inner).writeString } {
                    unsafe { write_string(c_key.as_ptr(), c_value.as_ptr()) };
                }
            }

            /// Writes persistent `bool` data as an `i32` to the `key` identifier in the .vproj file.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// api.write_bool("key name", true);
            /// # }
            /// ```
            pub fn write_bool(&self, key: &str, value: bool) {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                if let Some(write_int) = unsafe { (*self.inner).writeInt } {
                    unsafe { write_int(c_key.as_ptr(), value as i32) };
                }
            }

            /// Returns the current project path, or [`None`] if the project is not saved.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let plugin_path = api.project_path();
            /// # }
            /// ```
            pub fn project_path(&self) -> Option<String> {
                if let Some(get_project_path) = unsafe { (*self.inner).getProjectPath } {
                    let mut res_size = 0;
                    if unsafe { !get_project_path(std::ptr::null_mut(), &mut res_size as *mut _) } {
                        return None;
                    }
                    let mut res = vec![0u8; res_size as usize];
                    if unsafe {
                        get_project_path(
                            res.as_mut_slice().as_mut_ptr() as *mut _,
                            &mut res_size as *mut _,
                        )
                    } {
                        return $crate::utils::utf8_from_raw(&res).ok();
                    }
                }
                None
            }

            /// Returns the current project plugin path, or [`None`] if the path is unspecified.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let plugin_path = api.project_plugin_path();
            /// # }
            /// ```
            pub fn project_plugin_path(&self) -> Option<String> {
                if let Some(get_project_plugins_path) =
                    unsafe { (*self.inner).getProjectPluginsPath }
                {
                    let mut res_size = 0;
                    if unsafe {
                        !get_project_plugins_path(std::ptr::null_mut(), &mut res_size as *mut _)
                    } {
                        return None;
                    }
                    let mut res = vec![0u8; res_size as usize];
                    if unsafe {
                        get_project_plugins_path(
                            res.as_mut_slice().as_mut_ptr() as *mut _,
                            &mut res_size as *mut _,
                        )
                    } {
                        return $crate::utils::utf8_from_raw(&res).ok();
                    }
                }
                None
            }

            /// Returns the current [`ApplicationType`](crate::ApplicationType).
            /// Can be used to differentiate between Player, Producer and Streamer.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let application_type = api.application_type();
            /// # }
            /// ```
            pub fn application_type(&self) -> $crate::ApplicationType {
                if let Some(get_application_type) = unsafe { (*self.inner).getApplicationType } {
                    return unsafe { get_application_type() };
                }
                panic!("This version of Oden is too old to have the get_application_type function");
            }

            /// Signals to Oden that the plugin has crashed.
            /// This is used by the oden plugin panic handler
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// api.set_has_crashed();
            /// # }
            /// ```
            pub fn set_has_crashed(&self) {
                if let Some(set_has_crashed) = unsafe { (*self.inner).setHasCrashed } {
                    unsafe {
                        set_has_crashed();
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_has_crashed function");
                }
            }

            /// Disable the Ctrl+H GUI in Oden.
            ///
            /// If the `disable` arg is true than Ctrl+H functionality will be disabled.
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            ///  api.set_ui_disable(true);
            /// # }
            /// ```
            ///
            pub fn set_ui_disable(&self, disable: bool) {
                unsafe {
                    if let Some(set_ui_disable) = (*self.inner).setUiDisabled {
                        set_ui_disable(disable)
                    }
                }
            }

            /// Reads `String` data from the `key` identifier in the scene data.
            /// The `key` identifier can be written as a string such as "key_value" and the value will
            /// read from the current scene. To read data from a specific scene use the following
            /// syntax "Scene Name@key_value". For global plugins the latter alternative is required.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let key_value = api.read_scene_string("Scene1@key");
            /// # }
            /// ```
            pub fn read_scene_string(&self, key: &str) -> Option<String> {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                if let Some(read_scene_string) = unsafe { (*self.inner).readSceneString } {
                    let mut res = vec![0u8; 8192];
                    if unsafe {
                        read_scene_string(
                            c_key.as_ptr(),
                            res.as_mut_slice().as_mut_ptr() as *mut _,
                            res.len() as i32,
                        )
                    } {
                        return $crate::utils::utf8_from_raw_to_first_null_terminator(&res).ok();
                    }
                }
                None
            }

            /// Write persistent `String` data to the `key` identifier in the scene data.
            /// The `key` identifier can be written as a string as "key" and the value will
            /// be added to the current scene. To write data to a specific scene use the following
            /// syntax "Scene Name@key". For global plugins the latter alternative is required.
            ///
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// api.write_scene_string("Scene@key name", "string_value");
            /// # }
            /// ```
            pub fn write_scene_string(&self, key: &str, value: &str) -> bool {
                let c_key = std::ffi::CString::new(key.trim_end_matches('\0')).unwrap();
                let c_value = std::ffi::CString::new(value.trim_end_matches('\0')).unwrap();

                if let Some(write_scene_string) = unsafe { (*self.inner).writeSceneString } {
                    unsafe { write_scene_string(c_key.as_ptr(), c_value.as_ptr()) }
                } else {
                    false
                }
            }

            /// Will make the application gracefully exit without saving.
            /// With force_exit = false it will take into regard the "No Promt On Exit" setting and give a pop up to the user.
            /// If force_exit = true it will disregard the "No Promt On Exit" setting and exit on the next frame.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            ///     api.exit_application(false);
            /// # }
            /// ```
            pub fn exit_application(&self, force_exit: bool) {
                if let Some(exit_application) = unsafe { (*self.inner).exitApplication } {
                    unsafe { exit_application(force_exit) }
                } else {
                    panic!("This version of Oden is too old to have the exit_application function");
                }
            }

            /// Returns the current project plugin file path, or [`None`] if the path is unspecified.
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let plugin_file_path = api.project_file_path();
            /// # }
            /// ```
            pub fn project_file_path(&self) -> Option<String> {
                if let Some(get_project_file_path) = unsafe { (*self.inner).getProjectFilePath } {
                    let mut res_size = 0;
                    if unsafe {
                        !get_project_file_path(std::ptr::null_mut(), &mut res_size as *mut _)
                    } {
                        return None;
                    }
                    let mut res = vec![0u8; res_size as usize];
                    if unsafe {
                        get_project_file_path(
                            res.as_mut_slice().as_mut_ptr() as *mut _,
                            &mut res_size as *mut _,
                        )
                    } {
                        return $crate::utils::utf8_from_raw(&res).ok();
                    }
                }
                None
            }

            /// Publishes `data` as `data_id` that is shared between plugins.
            ///
            /// Oden offers a way to share data between plugins using named memory regions.
            /// One plugin can publish a named shared data and multiple plugins may query it.
            pub fn publish_data(&self, data_id: &str, data: &[u8]) {
                if let Some(oden_publish_data) = unsafe { (*self.inner).publishData } {
                    let data_id = std::ffi::CString::new(data_id.trim_end_matches('\0')).unwrap();
                    unsafe {
                        oden_publish_data(
                            data_id.as_ptr(),
                            data.as_ptr() as *const _,
                            data.len() as i32,
                        )
                    }
                } else {
                    panic!("This version of Oden is too old to have the publish_data function");
                }
            }

            /// Returns shared plugin data, metadata, and possible errors, or `crate::QueryError` if an error occurred.
            ///
            /// Query data that have been shared between plugins.
            /// Oden offers a way to share data between plugins using named memory regions.
            /// One plugin can publish a named shared data and multiple plugins may query it.
            /// Supplying `expected_data_size == -1` will return all data until a terminating byte'\0'
            ///
            /// Also used for command line data: `--plugin-data [key] [val]`
            pub fn query_data(
                &self,
                data_id: &str,
                expected_data_size: i32,
            ) -> Result<(Vec<u8>, $crate::QueryMetadata), $crate::QueryError> {
                if let Some(oden_query_data) = unsafe { (*self.inner).queryData } {
                    let data_id = std::ffi::CString::new(data_id.trim_end_matches('\0')).unwrap();
                    let mut metadata = $crate::QueryMetadata {
                        timestampPublished: 0,
                        framesSincePublished: 0,
                    };

                    let mut data: *mut std::ffi::c_void = std::ptr::null_mut();
                    let res = unsafe {
                        oden_query_data(
                            data_id.as_ptr(),
                            &mut metadata as *mut _,
                            &mut data as *mut _,
                            expected_data_size,
                        )
                    };

                    match res {
                        $crate::plugin_h::OdenQueryDataResult_e_OdenQueryDataResultDataIdNotFound => {
                            return Err($crate::QueryError::DataIdNotFound)
                        }
                        $crate::plugin_h::OdenQueryDataResult_e_OdenQueryDataResultDataIsNotNotExpectedSize => {
                            return Err($crate::QueryError::DataIsNotNotExpectedSize)
                        }
                        $crate::plugin_h::OdenQueryDataResult_e_OdenQueryDataResultOk => {
                            let data_size = if expected_data_size < 0 {
                                unsafe { libc::strlen(data as *const _) }
                            } else {
                                expected_data_size as usize
                            };

                            let mut res_data = vec![0; data_size];

                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    data as *mut u8,
                                    res_data.as_mut_ptr(),
                                    data_size,
                                )
                            };

                            return Ok((res_data, metadata));
                        }
                        _ => return Err($crate::QueryError::InvalidParameters),
                    }
                } else {
                    panic!("This version of Oden is too old to have the query_data function");
                }
            }

            /// Retrieve a plugin parameter
            ///
            /// Plugin parameters can be passed on command line as `[application] --plugin-param [key] [value]`
            /// This function takes `[key]` as input and returns `[value]`
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let window_title = api.plugin_param("window_title");
            /// # }
            /// ```
            pub fn plugin_param(&self, key: &str) -> Option<String> {
                if let Ok((raw_data, _)) = self.query_data(key, -1) {
                    let data = if raw_data.last() == Some(&0) {
                        &raw_data[..raw_data.len() - 1]
                    } else {
                        &raw_data
                    };
                    return String::from_utf8(data.to_vec()).ok();
                }

                None
            }

            /// Returns the current Oden version, e.g. "1.23.4"
            ///
            /// # Examples
            /// ```no_run
            /// # fn example(api: &dyn oden_plugin_rs::SettingsApi) {
            /// let version = api.oden_version();
            /// # }
            /// ```
            pub fn oden_version(&self) -> String {
                if let Some(get_oden_version) = unsafe { (*self.inner).getOdenVersion } {
                    let version_ptr = unsafe { get_oden_version() };
                    let c_str = unsafe { std::ffi::CStr::from_ptr(version_ptr) };
                    return c_str.to_str().unwrap().to_string();
                }
                panic!("This version of Oden is too old to have the get_oden_version function");
            }

            /// Fetches the hash of the currently activated oden license
            ///
            /// Example:
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SettingsApi) {
            /// let _license_hash = api.license_hash();
            /// # }
            /// ```
            pub fn license_hash(&self) -> Option<String> {
                if let Some(get_license_hash) = unsafe{(*self.inner).getLicenseHash} {

                    let mut buffer = [0u8; 128];

                    let bytes_written = unsafe { get_license_hash(buffer.as_mut_ptr() as *mut _, buffer.len() as _) };
                    if bytes_written > 0 {
                        Some(std::string::String::from_utf8_lossy(&buffer[..bytes_written as usize]).into_owned())
                    } else {
                        None
                    }

                } else {
                    panic!("This version of Oden is too old to have the license_hash function");
                }
            }

            /// Returns the current plugin instance uuid.
            ///
            /// Example:
            /// ```no_run
            /// # fn example(api: &impl oden_plugin_rs::SettingsApi) {
            /// let instance_uuid = api.instance_uuid();
            /// # }
            /// ```
            pub fn instance_uuid(&self) -> Option<$crate::math::Uuid> {
                unsafe {
                    if let Some(instance_uuid) =  (*self.inner).getInstanceUuid {
                         let mut uuid = $crate::math::Uuid::default();
                         if instance_uuid(&mut uuid){
                             Some(uuid)
                         } else {
                             None
                         }
                    } else {
                        panic!("This version of Oden is too old to have the instance_uuid function");
                    }
                }
            }
        }
    };
}

#[cfg(feature = "serialize")]
pub trait SettingsExtApi {
    fn read_struct<T: Default + serde::de::DeserializeOwned + 'static>(&self, key: &str) -> T;
    fn try_read_struct<T: serde::de::DeserializeOwned + 'static>(&self, key: &str) -> Option<T>;
    fn write_struct<T: serde::Serialize + 'static>(&self, key: &str, value: &T);
    fn modify_struct<T, F>(&self, key: &str, f: F)
    where
        T: Default + serde::Serialize + serde::de::DeserializeOwned + 'static,
        F: FnOnce(&mut T);

    fn read_scene_struct<T: Default + serde::de::DeserializeOwned + 'static>(&self, key: &str)
        -> T;
    fn write_scene_struct<T: serde::Serialize + 'static>(&self, key: &str, value: &T) -> bool;
    fn modify_scene_struct<T, F>(&self, key: &str, f: F) -> bool
    where
        T: Default + serde::Serialize + serde::de::DeserializeOwned + 'static,
        F: FnOnce(&mut T);

    fn publish_var_size_data<T>(&self, id: &str, data: &T)
    where
        T: serde::Serialize;
    fn query_var_size_data<'a, T>(&self, id: &str) -> Result<(T, QueryMetadata), QueryError>
    where
        T: for<'de> serde::Deserialize<'de>;
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_settings_ext_api {
    ($interface:ident) => {
        #[inherent::inherent]
        impl $crate::settings_api::SettingsExtApi for $interface<'_> {
            /// Reads an arbitrary deserializable struct from the `key` identifier in the .vproj file.
            #[cfg(feature = "serialize")]
            pub fn read_struct<T: Default + serde::de::DeserializeOwned + 'static>(
                &self,
                key: &str,
            ) -> T {
                self.try_read_struct(key).unwrap_or_default()
            }

            #[cfg(feature = "serialize")]
            pub fn try_read_struct<T: serde::de::DeserializeOwned + 'static>(
                &self,
                key: &str,
            ) -> Option<T> {
                if let Some(base64_encoded) = self.read_string(key) {
                    use base64::Engine;
                    if let Ok(decoded) =
                        base64::prelude::BASE64_STANDARD_NO_PAD.decode(&base64_encoded)
                    {
                        if let Ok(deserialized) =
                            ciborium::from_reader::<T, &[u8]>(decoded.as_slice())
                        {
                            return Some(deserialized);
                        }
                    }
                }

                return None;
            }

            /// Writes an arbitrary serializable struct to the `key` identifier in the .vproj file.
            #[cfg(feature = "serialize")]
            pub fn write_struct<T: serde::Serialize + 'static>(&self, key: &str, value: &T) {
                let mut encoded = Vec::new();

                ciborium::into_writer(value, &mut encoded).expect("Failed to serialize the struct");

                use base64::Engine;
                let base64_encoded = base64::prelude::BASE64_STANDARD_NO_PAD.encode(&encoded);
                self.write_string(key, &base64_encoded);
            }

            /// Convenient way of working with settings-backed structs
            #[cfg(feature = "serialize")]
            pub fn modify_struct<T, F>(&self, key: &str, f: F)
            where
                T: Default + serde::Serialize + serde::de::DeserializeOwned + 'static,
                F: FnOnce(&mut T),
            {
                let mut value = self.read_struct::<T>(key);
                f(&mut value);
                self.write_struct(key, &value)
            }

            /// Reads an arbitrary struct that have been written to the scene data.
            /// The `key` identifier can be written as a string as "key_value" and the value will
            /// be added to the current scene. To write data to a specific scene use the following
            /// syntax "Scene Name@key_value". For global plugins the later alternative is a must.
            #[cfg(feature = "serialize")]
            pub fn read_scene_struct<T: Default + serde::de::DeserializeOwned + 'static>(
                &self,
                key: &str,
            ) -> T {
                if let Some(base64_encoded) = self.read_scene_string(key) {
                    use base64::Engine;
                    if let Ok(decoded) =
                        base64::prelude::BASE64_STANDARD_NO_PAD.decode(&base64_encoded)
                    {
                        if let Ok(deserialized) =
                            ciborium::from_reader::<T, &[u8]>(decoded.as_slice())
                        {
                            return deserialized;
                        }
                    }
                }

                return T::default();
            }

            /// Write an arbitrary struct to the scene data.
            /// The `key` identifier is a string formatted as "key_value".
            /// To read data from specific scene use the following syntax "Scene Name@key_value".
            /// For global plugins the later alternative is a must.
            #[cfg(feature = "serialize")]
            pub fn write_scene_struct<T: serde::Serialize + 'static>(
                &self,
                key: &str,
                value: &T,
            ) -> bool {
                let mut encoded = Vec::new();

                ciborium::into_writer(value, &mut encoded).expect("Failed to serialize the struct");

                use base64::Engine;
                let base64_encoded = base64::prelude::BASE64_STANDARD_NO_PAD.encode(&encoded);
                self.write_scene_string(key, &base64_encoded)
            }

            /// Convenient function to modify a stored struct in the scene data.
            /// The `key` identifier is a string formatted as "key_value".
            /// To modify data from specific scene use the following syntax "Scene Name@key_value".
            /// For global plugins the later alternative is a must.
            #[cfg(feature = "serialize")]
            pub fn modify_scene_struct<T, F>(&self, key: &str, f: F) -> bool
            where
                T: Default + serde::Serialize + serde::de::DeserializeOwned + 'static,
                F: FnOnce(&mut T),
            {
                let mut value = self.read_scene_struct::<T>(key);
                f(&mut value);
                self.write_scene_struct(key, &value)
            }

            /// Publishes `data` as `data_id` that is shared between plugins.
            /// This is a utility function for `publish_data` to publish variable sized data.
            ///
            /// Oden offers a way to share data between plugins using named memory regions.
            /// One plugin can publish a named shared data and multiple plugins may query it.
            pub fn publish_var_size_data<T>(&self, id: &str, data: &T)
            where
                T: serde::Serialize,
            {
                let mut buffer = Vec::new();
                ciborium::into_writer(&data, &mut buffer).expect("Failed to serialize the data");

                let size = buffer.len() as i32;
                self.publish_data(&format!("{id}_size"), &size.to_le_bytes());
                self.publish_data(&format!("{id}_data"), &buffer);
            }

            /// Query data that have been shared between plugins.
            /// Returns shared plugin data, metadata, and possible errors, or `crate::QueryError` if an error occurred.
            /// This is a utility function for `query_data` to query variable sized data.
            ///
            /// Oden offers a way to share data between plugins using named memory regions.
            /// One plugin can publish a named shared data and multiple plugins may query it.
            ///
            /// Also used for command line data: `--plugin-data [key] [val]`
            pub fn query_var_size_data<'a, T>(
                &self,
                id: &str,
            ) -> Result<(T, $crate::QueryMetadata), $crate::QueryError>
            where
                T: for<'de> serde::Deserialize<'de>,
            {
                let (raw_size, _) =
                    self.query_data(&format!("{id}_size"), std::mem::size_of::<i32>() as i32)?;
                if raw_size.len() != 4 {
                    return Err($crate::QueryError::DataIsNotNotExpectedSize);
                }
                let data: [u8; 4] = raw_size.try_into().unwrap();
                let size = i32::from_le_bytes(data);

                let (raw_data, meta) = self.query_data(&format!("{id}_data"), size)?;
                let data = ciborium::from_reader::<T, _>(raw_data.as_slice())
                    .map_err(|_| $crate::QueryError::InvalidData)?;

                Ok((data, meta))
            }
        }
    };
}
