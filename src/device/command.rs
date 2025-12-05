pub enum DeviceCommand<'a> {
    Led(LedCommand<'a>),
    Coin(CoinCommand),
    Uart(UartCommand<'a>),
    Pwm(PwmCommand),
    // 未来新增设备就在这里扩展
}

pub enum LedCommand<'a> {
    Set(&'a [u8]),
}

pub enum CoinCommand {
    Reset,
    Enable,
    Disable,
    Eject,
}

pub enum UartCommand<'a> {
    Send(&'a [u8]),
}

pub enum PwmCommand {
    SetDuty(u16),
    SetFreq(u32),
}
