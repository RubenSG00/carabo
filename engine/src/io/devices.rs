//! Enumerating and choosing devices (06 §13.1), with Windows' default sound system through
//! `cpal` (WASAPI in shared mode, never ASIO: DEC-19).

use cpal::traits::HostTrait;
use serde::Serialize;

/// The output and input devices of the system (04 §6.1).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceList {
    pub outputs: Vec<DeviceInfo>,
    pub inputs: Vec<DeviceInfo>,
}

/// One device of a [`DeviceList`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub name: String,
    /// Whether it is Windows' default device.
    pub is_default: bool,
}

/// The devices in use (04 §6.1).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDevices {
    /// Real name of the output device in use, or `None` if there is no output.
    pub output_name: Option<String>,
    /// Real name of the device that will be used as input, or `None` if there is none.
    pub input_name: Option<String>,
    /// Whether the requested output device does not exist and the default one is used.
    pub output_fell_back: bool,
    pub input_fell_back: bool,
}

/// Windows' default output device, or `None` if there is no output device.
// TODO(T-080): enumerate the devices and choose one by name (06 §13.1).
pub(crate) fn default_output_device() -> Option<cpal::Device> {
    cpal::default_host().default_output_device()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn device_types_serialize_with_the_keys_of_04_6_1() {
        let list = DeviceList {
            outputs: vec![DeviceInfo {
                name: "Speakers".to_string(),
                is_default: true,
            }],
            inputs: Vec::new(),
        };
        assert_eq!(
            serde_json::to_value(&list).expect("a device list serializes"),
            json!({ "outputs": [{ "name": "Speakers", "isDefault": true }], "inputs": [] })
        );

        let active = ActiveDevices {
            output_name: Some("Speakers".to_string()),
            input_name: None,
            output_fell_back: true,
            input_fell_back: false,
        };
        assert_eq!(
            serde_json::to_value(&active).expect("active devices serialize"),
            json!({
                "outputName": "Speakers",
                "inputName": null,
                "outputFellBack": true,
                "inputFellBack": false
            })
        );
    }
}
