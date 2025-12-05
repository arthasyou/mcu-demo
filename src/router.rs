use crate::protocol::{
    CommandType, DeviceCommand, DeviceResponse, create_ack_response, create_nack_response,
};

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Self {}
    }

    pub fn route(&mut self, cmd: DeviceCommand) -> DeviceResponse {
        let device_id = cmd.device_id;

        match CommandType::try_from(cmd.cmd_type) {
            Ok(CommandType::CoinDispense) => create_ack_response(device_id),
            Ok(CommandType::LightOn) => create_ack_response(device_id),
            Ok(CommandType::LightOff) => create_ack_response(device_id),
            Ok(CommandType::MotorForward) => create_ack_response(device_id),
            Ok(CommandType::MotorBackward) => create_ack_response(device_id),
            Ok(CommandType::MotorStop) => create_ack_response(device_id),
            Ok(CommandType::SpeakerPlay) => create_ack_response(device_id),
            Ok(CommandType::ElectromagnetOn) => create_ack_response(device_id),
            Ok(CommandType::ElectromagnetOff) => create_ack_response(device_id),
            Ok(CommandType::SensorRead) => create_ack_response(device_id),
            Err(_) => create_nack_response(device_id),
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
