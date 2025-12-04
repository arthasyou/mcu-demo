// pub mod device {
//     include!(concat!(env!("OUT_DIR"), "/device.rs"));
// }

// pub use device::{CommandType, DeviceCommand, DeviceResponse, ResponseType};
// use heapless::Vec;
// use prost::Message;

// pub fn decode_command(data: &[u8]) -> Result<DeviceCommand, ()> {
//     DeviceCommand::decode(data).map_err(|_| ())
// }

// pub fn encode_response(response: &DeviceResponse) -> Result<Vec<u8, 256>, ()> {
//     let mut buf = Vec::new();
//     buf.resize(response.encoded_len(), 0).map_err(|_| ())?;
//     response.encode(&mut buf.as_mut_slice()).map_err(|_| ())?;
//     buf.truncate(response.encoded_len());
//     Ok(buf)
// }

// pub fn create_ack_response(device_id: u32) -> DeviceResponse {
//     DeviceResponse {
//         device_id,
//         resp_type: ResponseType::Ack as i32,
//         data: alloc::vec::Vec::new(),
//     }
// }

// pub fn create_nack_response(device_id: u32) -> DeviceResponse {
//     DeviceResponse {
//         device_id,
//         resp_type: ResponseType::Nack as i32,
//         data: alloc::vec::Vec::new(),
//     }
// }

// pub fn create_sensor_response(device_id: u32, sensor_data: &[u8]) -> Result<DeviceResponse, ()> {
//     Ok(DeviceResponse {
//         device_id,
//         resp_type: ResponseType::SensorData as i32,
//         data: sensor_data.to_vec(),
//     })
// }
