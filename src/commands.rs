//! Commands for communicating with WiFi bridges. The values on this page come from http://www.limitlessled.com/dev/.

pub mod v3 {
    //! These commands work with Version 3 and below bridges.
    use colors::Color;

    /// A command that the wifi bridge can perform
    /// # Example
    /// ```
    /// # use milight::colors::*;
    /// # use milight::commands::v3::*;
    /// # fn main() {
    /// let set_color: Vec<u8> = Command::SetColor(GREEN).into();
    /// assert_eq!([0x20, 0x60, 0x55].to_vec(), set_color);
    ///
    /// let prev: Vec<u8> = Command::ModeDown.into();
    /// assert_eq!([0x28, 0x00, 0x55].to_vec(), prev);
    /// # }
    /// ```

    #[derive(Debug)]
    pub enum Command {
        /// Turns lights off
        Off,
        /// Turns lights on
        On,
        /// Sets lights to a solid RGB color
        SetColor(Color),
        /// Makes lights 1 step dimmer
        BrightnessUp,
        /// Makes lights 1 step brighter
        BrightnessDown,
        /// Makes pattern move faster
        SpeedUp,
        /// Makes pattern move slower
        SpeedDown,
        /// Changes to next pattern mode
        ModeUp,
        /// Changes to previous pattern mode
        ModeDown,
    }

    impl Into<Vec<u8>> for Command {
        fn into(self) -> Vec<u8> {
            match self {
                Command::SetColor(color) => [0x20, color.into(), 0x55],
                Command::Off => [0x21, 0x00, 0x55],
                Command::On => [0x22, 0x00, 0x55],
                Command::BrightnessUp => [0x23, 0x00, 0x55],
                Command::BrightnessDown => [0x24, 0x00, 0x55],
                Command::SpeedUp => [0x25, 0x00, 0x55],
                Command::SpeedDown => [0x26, 0x00, 0x55],
                Command::ModeUp => [0x27, 0x00, 0x55],
                Command::ModeDown => [0x28, 0x00, 0x55],
            }.to_vec()
        }
    }
}
