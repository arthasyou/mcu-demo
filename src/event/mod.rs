/// 所有业务事件的统称，可以是枚举
pub enum Event<'a> {
    /// 外部输入数据
    Input(&'a [u8]),

    /// 输出数据（写给 USB/UART 的）
    Output(&'a [u8]),

    DeviceReady,
    ButtonPressed(u8),
    SensorUpdated(u16),
    Timeout(u32),
    Custom(&'static str),
}
