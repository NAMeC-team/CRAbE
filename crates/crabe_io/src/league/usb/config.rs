use clap::Args;
#[derive(Args)]
pub struct UsbConfig {
    #[arg(long, default_value = "/dev/ttyUSB0")]
    pub usb_port: String,
    #[arg(long, default_value_t = 115_200)]
    pub usb_baud: u32,
}
