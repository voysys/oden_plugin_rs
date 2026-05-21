#![allow(missing_docs)]

use crate::{
    plugin_h::{
        OdenInferenceSettings, OdenNeuralNetworkInfo, OdenTensorData, OdenTensorDataType,
        OdenTensorFormat, OdenTensorInput, OdenTensorShape,
    },
    NeuralNetworkError, OdenUuid,
};
use std::error::Error;
use std::fmt;

pub struct TensorData<'a> {
    pub name: &'a str,
    pub format: OdenTensorFormat,
    pub shape: OdenTensorShape,
    pub data_type: OdenTensorDataType,
    pub data: Option<&'a [f32]>,
    pub timestamp: i64,
    pub frame_id: i32,
}

impl TryFrom<OdenTensorData> for TensorData<'_> {
    type Error = crate::NeuralNetworkError;

    fn try_from(oden_tensor_data: OdenTensorData) -> Result<Self, Self::Error> {
        let name = unsafe {
            std::ffi::CStr::from_ptr(oden_tensor_data.name)
                .to_str()
                .map_err(|_| crate::NeuralNetworkError::OdenNeuralNetworkErrorInvalidTensorName)?
        };

        if oden_tensor_data.dataType != OdenTensorDataType::OdenTensorDataTypeFp32 {
            return Err(crate::NeuralNetworkError::OdenNeuralNetworkErrorNotSupported);
        }

        let data = if !oden_tensor_data.data.is_null() {
            let num_elements = oden_tensor_data.dataSize as usize / std::mem::size_of::<f32>();
            Some(unsafe {
                std::slice::from_raw_parts(oden_tensor_data.data as *const f32, num_elements)
            })
        } else {
            None
        };

        Ok(TensorData {
            name,
            format: oden_tensor_data.format,
            shape: oden_tensor_data.shape,
            data_type: oden_tensor_data.dataType,
            data,
            timestamp: oden_tensor_data.timestamp,
            frame_id: oden_tensor_data.frameId,
        })
    }
}

#[derive(Debug)]
pub struct TensorInput<'a> {
    pub name: &'a str,
    pub video_stream_uuid: OdenUuid,
}

impl TryFrom<&OdenTensorInput> for TensorInput<'_> {
    type Error = crate::NeuralNetworkError;

    fn try_from(oden_tensor_input: &OdenTensorInput) -> Result<Self, Self::Error> {
        let name = unsafe {
            std::ffi::CStr::from_ptr(oden_tensor_input.name)
                .to_str()
                .map_err(|_| crate::NeuralNetworkError::OdenNeuralNetworkErrorInvalidTensorName)?
        };

        Ok(TensorInput {
            name,
            video_stream_uuid: oden_tensor_input.videoStreamUuid,
        })
    }
}

impl fmt::Display for TensorInput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TensorInput: {{ name: {}, video_stream_uuid: {} }}",
            self.name, self.video_stream_uuid
        )
    }
}

#[derive(Debug)]
pub struct NeuralNetworkInfo<'a> {
    pub uuid: OdenUuid,
    pub network_path: &'a str,
    pub manually_trigger: bool,
    pub inputs: Vec<TensorInput<'a>>,
}

impl TryFrom<OdenNeuralNetworkInfo> for NeuralNetworkInfo<'_> {
    type Error = crate::NeuralNetworkError;

    fn try_from(oden_neural_network_info: OdenNeuralNetworkInfo) -> Result<Self, Self::Error> {
        let network_path = unsafe {
            std::ffi::CStr::from_ptr(oden_neural_network_info.networkPath)
                .to_str()
                .map_err(|_| crate::NeuralNetworkError::OdenNeuralNetworkErrorInvalidPath)?
        };

        let inputs = oden_neural_network_info.inputs
            [..oden_neural_network_info.tensorInputsCount as usize]
            .iter()
            .map(TensorInput::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(NeuralNetworkInfo {
            uuid: oden_neural_network_info.uuid,
            network_path,
            manually_trigger: oden_neural_network_info.manuallyTrigger,
            inputs,
        })
    }
}

impl TryFrom<&OdenNeuralNetworkInfo> for NeuralNetworkInfo<'_> {
    type Error = crate::NeuralNetworkError;

    fn try_from(oden_neural_network_info: &OdenNeuralNetworkInfo) -> Result<Self, Self::Error> {
        let network_path = unsafe {
            std::ffi::CStr::from_ptr(oden_neural_network_info.networkPath)
                .to_str()
                .map_err(|_| crate::NeuralNetworkError::OdenNeuralNetworkErrorInvalidPath)?
        };

        let inputs = oden_neural_network_info.inputs
            [..oden_neural_network_info.tensorInputsCount as usize]
            .iter()
            .map(TensorInput::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(NeuralNetworkInfo {
            uuid: oden_neural_network_info.uuid,
            network_path,
            manually_trigger: oden_neural_network_info.manuallyTrigger,
            inputs,
        })
    }
}

impl fmt::Display for NeuralNetworkInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs_str = self
            .inputs
            .iter()
            .map(|input| input.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "NeuralNetworkInfo: {{ uuid: {}, network_path: {}, manually_trigger: {}, inputs: [{}] }}", self.uuid, self.network_path, self.manually_trigger, inputs_str)
    }
}

#[derive(Debug)]
pub struct InferenceSettings<'a> {
    pub neural_networks: Vec<NeuralNetworkInfo<'a>>,
}

impl TryFrom<OdenInferenceSettings> for InferenceSettings<'_> {
    type Error = crate::NeuralNetworkError;

    fn try_from(oden_inference_settings: OdenInferenceSettings) -> Result<Self, Self::Error> {
        let neural_networks = oden_inference_settings.neuralNetworks
            [..oden_inference_settings.neuralNetworkCount as usize]
            .iter()
            .map(NeuralNetworkInfo::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(InferenceSettings { neural_networks })
    }
}

impl fmt::Display for InferenceSettings<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let networks_str = self
            .neural_networks
            .iter()
            .map(|network| network.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "InferenceSettings: {{ neural_networks: [{networks_str}] }}"
        )
    }
}

impl fmt::Display for NeuralNetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            NeuralNetworkError::OdenNeuralNetworkErrorOk => "Ok",
            NeuralNetworkError::OdenNeuralNetworkErrorUnknown => "Unknown error",
            NeuralNetworkError::OdenNeuralNetworkErrorNotSupported => "Not supported",
            NeuralNetworkError::OdenNeuralNetworkErrorFailedToLoadNvInfer => {
                "Failed to load nvinfer"
            }
            NeuralNetworkError::OdenNeuralNetworkErrorInvalidUuid => "Invalid UUID",
            NeuralNetworkError::OdenNeuralNetworkErrorInvalidPath => "Invalid path",
            NeuralNetworkError::OdenNeuralNetworkErrorInvalidTensorName => "Invalid tensor name",
            NeuralNetworkError::OdenNeuralNetworkErrorInvalidEntityId => "Invalid entity ID",
            NeuralNetworkError::OdenNeuralNetworkErrorInvalidTensorData => "Invalid tensor data",
            NeuralNetworkError::OdenNeuralNetworkErrorNetworkNotFound => "Network not found",
            NeuralNetworkError::OdenNeuralNetworkErrorIsLoading => "Neural network is loading",
            NeuralNetworkError::OdenNeuralNetworkErrorNotLoaded => "Neural network is not loaded",
            NeuralNetworkError::OdenNeuralNetworkErrorMaxEnum => "Max enum",
            NeuralNetworkError::OdenNeuralNetworkErrorArgumentIsNull => "Argument is null",
        };

        write!(f, "NeuralNetworkError: {msg}")
    }
}

impl Error for NeuralNetworkError {}
