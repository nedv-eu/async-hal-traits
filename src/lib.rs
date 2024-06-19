#![no_std]
#![feature(async_fn_in_trait)]


pub mod adc {
    use embedded_hal::adc::Channel;
    pub trait OneShotAsync<ADC, Word, Pin: Channel<ADC>> {
        /// Error type returned by ADC methods
        type Error;

        /// Request that the ADC begin a conversion on the specified pin
        ///
        /// This method takes a `Pin` reference, as it is expected that the ADC will be able to sample
        /// whatever channel underlies the pin.
        async fn read_async(&mut self, pin: &mut Pin) -> Result<Word, Self::Error>;
    }

    pub trait OneShotAsyncMilliVolts<ADC, Pin: Channel<ADC>> {
        /// Error type returned by ADC methods
        type Error;

        /// Request that the ADC begin a conversion on the specified pin
        ///
        /// This method takes a `Pin` reference, as it is expected that the ADC will be able to sample
        /// whatever channel underlies the pin. Returns value in millivolts 
        async fn read_async_mv(&mut self, pin: &mut Pin, ref_voltage_mv: Option<i32>) -> Result<i32, Self::Error>;
    }

    pub trait SupplyVoltage {
        /// Error type returned by ADC methods
        type Error;
        async fn read_supply_voltage_mv(&mut self) -> Result<i32, Self::Error>;
    }

}

