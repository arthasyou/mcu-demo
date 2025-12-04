/// 所有业务事件的统称，可以是枚举
pub enum Event {
    DeviceReady,
    ButtonPressed(u8),
    SensorUpdated(u16),
    Timeout(u32),
    Custom(&'static str),
}
