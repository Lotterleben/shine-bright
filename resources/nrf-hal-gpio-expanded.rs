pub mod gpio {
    use core::marker::PhantomData;
    /// Disconnected pin in input mode (type state, reset value).
    pub struct Disconnected;
    /// Input mode (type state).
    pub struct Input<MODE> {
        _mode: PhantomData<MODE>,
    }
    /// Floating input (type state).
    pub struct Floating;
    /// Pulled down input (type state).
    pub struct PullDown;
    /// Pulled up input (type state).
    pub struct PullUp;
    /// Output mode (type state).
    pub struct Output<MODE> {
        _mode: PhantomData<MODE>,
    }
    /// Push pull output (type state).
    pub struct PushPull;
    /// Open drain output (type state).
    pub struct OpenDrain;
    /// Represents a digital input or output level.
    pub enum Level {
        Low,
        High,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Level {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Level::Low,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Low");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Level::High,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "High");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for Level {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Level {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl ::core::marker::StructuralPartialEq for Level {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Level {
        #[inline]
        fn eq(&self, other: &Level) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    /// A GPIO port with up to 32 pins.
    pub enum Port {
        /// Port 0, available on all nRF52 and nRF51 MCUs.
        Port0,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Port {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Port::Port0,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Port0");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for Port {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Port {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl ::core::marker::StructuralPartialEq for Port {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Port {
        #[inline]
        fn eq(&self, other: &Port) -> bool {
            match (&*self, &*other) {
                _ => true,
            }
        }
    }
    /// Generic $PX pin
    pub struct Pin<MODE> {
        /// 00AB BBBB
        /// A: Port
        /// B: Pin
        pin_port: u8,
        _mode: PhantomData<MODE>,
    }
    use crate::hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};
    #[cfg(not(any(feature = "9160", feature = "51")))]
    use crate::pac::{p0 as gpio, P0};
    use void::Void;
    impl<MODE> Pin<MODE> {
        fn new(port: Port, pin: u8) -> Self {
            let port_bits = match port {
                Port::Port0 => 0x00,
            };
            Self {
                pin_port: pin | port_bits,
                _mode: PhantomData,
            }
        }
        pub unsafe fn from_psel_bits(psel_bits: u32) -> Self {
            Self {
                pin_port: psel_bits as u8,
                _mode: PhantomData,
            }
        }
        #[inline]
        pub fn pin(&self) -> u8 {
            #[cfg(not(any(feature = "52833", feature = "52840")))]
            {
                self.pin_port
            }
        }
        #[inline]
        pub fn port(&self) -> Port {
            #[cfg(not(any(feature = "52833", feature = "52840")))]
            {
                Port::Port0
            }
        }
        #[inline]
        pub fn psel_bits(&self) -> u32 {
            self.pin_port as u32
        }
        fn block(&self) -> &gpio::RegisterBlock {
            let ptr = match self.port() {
                Port::Port0 => P0::ptr(),
            };
            unsafe { &*ptr }
        }
        pub(crate) fn conf(&self) -> &gpio::PIN_CNF {
            &self.block().pin_cnf[self.pin() as usize]
        }
        /// Convert the pin to be a floating input
        pub fn into_floating_input(self) -> Pin<Input<Floating>> {
            self.conf().write(|w| {
                w.dir().input();
                w.input().connect();
                w.pull().disabled();
                w.drive().s0s1();
                w.sense().disabled();
                w
            });
            Pin {
                _mode: PhantomData,
                pin_port: self.pin_port,
            }
        }
        pub fn into_pullup_input(self) -> Pin<Input<PullUp>> {
            self.conf().write(|w| {
                w.dir().input();
                w.input().connect();
                w.pull().pullup();
                w.drive().s0s1();
                w.sense().disabled();
                w
            });
            Pin {
                _mode: PhantomData,
                pin_port: self.pin_port,
            }
        }
        pub fn into_pulldown_input(self) -> Pin<Input<PullDown>> {
            self.conf().write(|w| {
                w.dir().input();
                w.input().connect();
                w.pull().pulldown();
                w.drive().s0s1();
                w.sense().disabled();
                w
            });
            Pin {
                _mode: PhantomData,
                pin_port: self.pin_port,
            }
        }
        /// Convert the pin to be a push-pull output with normal drive.
        pub fn into_push_pull_output(self, initial_output: Level) -> Pin<Output<PushPull>> {
            let mut pin = Pin {
                _mode: PhantomData,
                pin_port: self.pin_port,
            };
            match initial_output {
                Level::Low => pin.set_low().unwrap(),
                Level::High => pin.set_high().unwrap(),
            }
            self.conf().write(|w| {
                w.dir().output();
                w.input().connect();
                w.pull().disabled();
                w.drive().s0s1();
                w.sense().disabled();
                w
            });
            pin
        }
        /// Convert the pin to be an open-drain output.
        ///
        /// This method currently does not support configuring an internal pull-up or pull-down
        /// resistor.
        pub fn into_open_drain_output(
            self,
            config: OpenDrainConfig,
            initial_output: Level,
        ) -> Pin<Output<OpenDrain>> {
            let mut pin = Pin {
                _mode: PhantomData,
                pin_port: self.pin_port,
            };
            match initial_output {
                Level::Low => pin.set_low().unwrap(),
                Level::High => pin.set_high().unwrap(),
            }
            self.conf().write(|w| {
                w.dir().output();
                w.input().disconnect();
                w.pull().disabled();
                w.drive().variant(config.variant());
                w.sense().disabled();
                w
            });
            pin
        }
        /// Disconnects the pin.
        ///
        /// In disconnected mode the pin cannot be used as input or output.
        /// It is primarily useful to reduce power usage.
        pub fn into_disconnected(self) -> Pin<Disconnected> {
            self.conf().reset();
            Pin {
                _mode: PhantomData,
                pin_port: self.pin_port,
            }
        }
    }
    impl<MODE> InputPin for Pin<Input<MODE>> {
        type Error = Void;
        fn is_high(&self) -> Result<bool, Self::Error> {
            self.is_low().map(|v| !v)
        }
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.block().in_.read().bits() & (1 << self.pin()) == 0)
        }
    }
    impl<MODE> OutputPin for Pin<Output<MODE>> {
        type Error = Void;
        /// Set the output as high.
        fn set_high(&mut self) -> Result<(), Self::Error> {
            unsafe {
                self.block().outset.write(|w| w.bits(1u32 << self.pin()));
            }
            Ok(())
        }
        /// Set the output as low.
        fn set_low(&mut self) -> Result<(), Self::Error> {
            unsafe {
                self.block().outclr.write(|w| w.bits(1u32 << self.pin()));
            }
            Ok(())
        }
    }
    impl<MODE> StatefulOutputPin for Pin<Output<MODE>> {
        /// Is the output pin set as high?
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            self.is_set_low().map(|v| !v)
        }
        /// Is the output pin set as low?
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.block().out.read().bits() & (1 << self.pin()) == 0)
        }
    }
    /// Pin configuration for open-drain mode.
    pub enum OpenDrainConfig {
        Disconnect0Standard1,
        Disconnect0HighDrive1,
        Standard0Disconnect1,
        HighDrive0Disconnect1,
    }
    #[cfg(not(any(feature = "9160", feature = "51")))]
    use crate::pac::p0::pin_cnf;
    impl OpenDrainConfig {
        fn variant(self) -> pin_cnf::DRIVE_A {
            use self::OpenDrainConfig::*;
            match self {
                Disconnect0Standard1 => pin_cnf::DRIVE_A::D0S1,
                Disconnect0HighDrive1 => pin_cnf::DRIVE_A::D0H1,
                Standard0Disconnect1 => pin_cnf::DRIVE_A::S0D1,
                HighDrive0Disconnect1 => pin_cnf::DRIVE_A::H0D1,
            }
        }
    }
    /// GPIO
    pub mod p0 {
        use super::{
            Disconnected, Floating, Input, Level, OpenDrain, OpenDrainConfig, Output, PhantomData,
            Pin, Port, PullDown, PullUp, PushPull, P0,
        };
        use crate::hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};
        use void::Void;
        /// GPIO parts
        pub struct Parts {
            /// Pin
            pub p0_00: P0_00<Disconnected>,
            /// Pin
            pub p0_01: P0_01<Disconnected>,
            /// Pin
            pub p0_02: P0_02<Disconnected>,
            /// Pin
            pub p0_03: P0_03<Disconnected>,
            /// Pin
            pub p0_04: P0_04<Disconnected>,
            /// Pin
            pub p0_05: P0_05<Disconnected>,
            /// Pin
            pub p0_06: P0_06<Disconnected>,
            /// Pin
            pub p0_07: P0_07<Disconnected>,
            /// Pin
            pub p0_08: P0_08<Disconnected>,
            /// Pin
            pub p0_09: P0_09<Disconnected>,
            /// Pin
            pub p0_10: P0_10<Disconnected>,
            /// Pin
            pub p0_11: P0_11<Disconnected>,
            /// Pin
            pub p0_12: P0_12<Disconnected>,
            /// Pin
            pub p0_13: P0_13<Disconnected>,
            /// Pin
            pub p0_14: P0_14<Disconnected>,
            /// Pin
            pub p0_15: P0_15<Disconnected>,
            /// Pin
            pub p0_16: P0_16<Disconnected>,
            /// Pin
            pub p0_17: P0_17<Disconnected>,
            /// Pin
            pub p0_18: P0_18<Disconnected>,
            /// Pin
            pub p0_19: P0_19<Disconnected>,
            /// Pin
            pub p0_20: P0_20<Disconnected>,
            /// Pin
            pub p0_21: P0_21<Disconnected>,
            /// Pin
            pub p0_22: P0_22<Disconnected>,
            /// Pin
            pub p0_23: P0_23<Disconnected>,
            /// Pin
            pub p0_24: P0_24<Disconnected>,
            /// Pin
            pub p0_25: P0_25<Disconnected>,
            /// Pin
            pub p0_26: P0_26<Disconnected>,
            /// Pin
            pub p0_27: P0_27<Disconnected>,
            /// Pin
            pub p0_28: P0_28<Disconnected>,
            /// Pin
            pub p0_29: P0_29<Disconnected>,
            /// Pin
            pub p0_30: P0_30<Disconnected>,
            /// Pin
            pub p0_31: P0_31<Disconnected>,
        }
        impl Parts {
            pub fn new(_gpio: P0) -> Self {
                Self {
                    p0_00: P0_00 { _mode: PhantomData },
                    p0_01: P0_01 { _mode: PhantomData },
                    p0_02: P0_02 { _mode: PhantomData },
                    p0_03: P0_03 { _mode: PhantomData },
                    p0_04: P0_04 { _mode: PhantomData },
                    p0_05: P0_05 { _mode: PhantomData },
                    p0_06: P0_06 { _mode: PhantomData },
                    p0_07: P0_07 { _mode: PhantomData },
                    p0_08: P0_08 { _mode: PhantomData },
                    p0_09: P0_09 { _mode: PhantomData },
                    p0_10: P0_10 { _mode: PhantomData },
                    p0_11: P0_11 { _mode: PhantomData },
                    p0_12: P0_12 { _mode: PhantomData },
                    p0_13: P0_13 { _mode: PhantomData },
                    p0_14: P0_14 { _mode: PhantomData },
                    p0_15: P0_15 { _mode: PhantomData },
                    p0_16: P0_16 { _mode: PhantomData },
                    p0_17: P0_17 { _mode: PhantomData },
                    p0_18: P0_18 { _mode: PhantomData },
                    p0_19: P0_19 { _mode: PhantomData },
                    p0_20: P0_20 { _mode: PhantomData },
                    p0_21: P0_21 { _mode: PhantomData },
                    p0_22: P0_22 { _mode: PhantomData },
                    p0_23: P0_23 { _mode: PhantomData },
                    p0_24: P0_24 { _mode: PhantomData },
                    p0_25: P0_25 { _mode: PhantomData },
                    p0_26: P0_26 { _mode: PhantomData },
                    p0_27: P0_27 { _mode: PhantomData },
                    p0_28: P0_28 { _mode: PhantomData },
                    p0_29: P0_29 { _mode: PhantomData },
                    p0_30: P0_30 { _mode: PhantomData },
                    p0_31: P0_31 { _mode: PhantomData },
                }
            }
        }
        pub struct P0_00<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_00<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_00<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[0] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_00 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_00<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[0] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_00 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_00<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[0] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_00 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_00<Output<PushPull>> {
                let mut pin = P0_00 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[0] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_00<Output<OpenDrain>> {
                let mut pin = P0_00 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[0] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_00<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[0] }.reset();
                P0_00 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 0)
            }
        }
        impl<MODE> InputPin for P0_00<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 0)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_00<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 0));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 0));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_00<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 0)) == 0 })
            }
        }
        pub struct P0_01<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_01<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_01<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[1] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_01 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_01<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[1] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_01 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_01<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[1] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_01 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_01<Output<PushPull>> {
                let mut pin = P0_01 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[1] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_01<Output<OpenDrain>> {
                let mut pin = P0_01 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[1] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_01<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[1] }.reset();
                P0_01 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 1)
            }
        }
        impl<MODE> InputPin for P0_01<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 1)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_01<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 1));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 1));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_01<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 1)) == 0 })
            }
        }
        pub struct P0_02<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_02<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_02<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[2] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_02 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_02<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[2] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_02 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_02<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[2] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_02 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_02<Output<PushPull>> {
                let mut pin = P0_02 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[2] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_02<Output<OpenDrain>> {
                let mut pin = P0_02 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[2] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_02<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[2] }.reset();
                P0_02 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 2)
            }
        }
        impl<MODE> InputPin for P0_02<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 2)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_02<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 2));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 2));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_02<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 2)) == 0 })
            }
        }
        pub struct P0_03<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_03<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_03<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[3] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_03 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_03<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[3] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_03 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_03<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[3] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_03 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_03<Output<PushPull>> {
                let mut pin = P0_03 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[3] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_03<Output<OpenDrain>> {
                let mut pin = P0_03 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[3] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_03<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[3] }.reset();
                P0_03 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 3)
            }
        }
        impl<MODE> InputPin for P0_03<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 3)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_03<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 3));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 3));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_03<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 3)) == 0 })
            }
        }
        pub struct P0_04<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_04<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_04<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[4] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_04 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_04<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[4] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_04 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_04<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[4] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_04 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_04<Output<PushPull>> {
                let mut pin = P0_04 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[4] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_04<Output<OpenDrain>> {
                let mut pin = P0_04 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[4] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_04<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[4] }.reset();
                P0_04 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 4)
            }
        }
        impl<MODE> InputPin for P0_04<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 4)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_04<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 4));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 4));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_04<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 4)) == 0 })
            }
        }
        pub struct P0_05<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_05<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_05<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[5] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_05 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_05<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[5] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_05 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_05<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[5] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_05 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_05<Output<PushPull>> {
                let mut pin = P0_05 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[5] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_05<Output<OpenDrain>> {
                let mut pin = P0_05 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[5] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_05<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[5] }.reset();
                P0_05 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 5)
            }
        }
        impl<MODE> InputPin for P0_05<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 5)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_05<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 5));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 5));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_05<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 5)) == 0 })
            }
        }
        pub struct P0_06<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_06<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_06<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[6] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_06 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_06<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[6] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_06 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_06<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[6] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_06 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_06<Output<PushPull>> {
                let mut pin = P0_06 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[6] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_06<Output<OpenDrain>> {
                let mut pin = P0_06 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[6] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_06<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[6] }.reset();
                P0_06 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 6)
            }
        }
        impl<MODE> InputPin for P0_06<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 6)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_06<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 6));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 6));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_06<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 6)) == 0 })
            }
        }
        pub struct P0_07<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_07<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_07<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[7] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_07 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_07<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[7] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_07 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_07<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[7] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_07 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_07<Output<PushPull>> {
                let mut pin = P0_07 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[7] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_07<Output<OpenDrain>> {
                let mut pin = P0_07 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[7] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_07<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[7] }.reset();
                P0_07 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 7)
            }
        }
        impl<MODE> InputPin for P0_07<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 7)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_07<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 7));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 7));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_07<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 7)) == 0 })
            }
        }
        pub struct P0_08<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_08<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_08<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[8] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_08 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_08<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[8] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_08 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_08<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[8] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_08 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_08<Output<PushPull>> {
                let mut pin = P0_08 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[8] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_08<Output<OpenDrain>> {
                let mut pin = P0_08 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[8] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_08<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[8] }.reset();
                P0_08 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 8)
            }
        }
        impl<MODE> InputPin for P0_08<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 8)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_08<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 8));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 8));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_08<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 8)) == 0 })
            }
        }
        pub struct P0_09<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_09<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_09<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[9] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_09 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_09<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[9] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_09 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_09<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[9] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_09 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_09<Output<PushPull>> {
                let mut pin = P0_09 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[9] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_09<Output<OpenDrain>> {
                let mut pin = P0_09 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[9] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_09<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[9] }.reset();
                P0_09 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 9)
            }
        }
        impl<MODE> InputPin for P0_09<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 9)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_09<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 9));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 9));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_09<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 9)) == 0 })
            }
        }
        pub struct P0_10<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_10<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_10<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[10] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_10 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_10<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[10] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_10 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_10<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[10] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_10 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_10<Output<PushPull>> {
                let mut pin = P0_10 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[10] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_10<Output<OpenDrain>> {
                let mut pin = P0_10 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[10] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_10<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[10] }.reset();
                P0_10 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 10)
            }
        }
        impl<MODE> InputPin for P0_10<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 10)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_10<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 10));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 10));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_10<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 10)) == 0 })
            }
        }
        pub struct P0_11<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_11<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_11<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[11] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_11 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_11<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[11] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_11 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_11<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[11] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_11 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_11<Output<PushPull>> {
                let mut pin = P0_11 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[11] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_11<Output<OpenDrain>> {
                let mut pin = P0_11 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[11] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_11<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[11] }.reset();
                P0_11 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 11)
            }
        }
        impl<MODE> InputPin for P0_11<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 11)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_11<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 11));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 11));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_11<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 11)) == 0 })
            }
        }
        pub struct P0_12<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_12<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_12<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[12] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_12 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_12<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[12] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_12 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_12<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[12] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_12 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_12<Output<PushPull>> {
                let mut pin = P0_12 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[12] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_12<Output<OpenDrain>> {
                let mut pin = P0_12 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[12] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_12<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[12] }.reset();
                P0_12 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 12)
            }
        }
        impl<MODE> InputPin for P0_12<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 12)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_12<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 12));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 12));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_12<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 12)) == 0 })
            }
        }
        pub struct P0_13<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_13<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_13<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[13] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_13 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_13<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[13] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_13 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_13<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[13] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_13 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_13<Output<PushPull>> {
                let mut pin = P0_13 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[13] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_13<Output<OpenDrain>> {
                let mut pin = P0_13 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[13] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_13<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[13] }.reset();
                P0_13 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 13)
            }
        }
        impl<MODE> InputPin for P0_13<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 13)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_13<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 13));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 13));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_13<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 13)) == 0 })
            }
        }
        pub struct P0_14<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_14<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_14<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[14] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_14 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_14<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[14] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_14 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_14<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[14] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_14 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_14<Output<PushPull>> {
                let mut pin = P0_14 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[14] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_14<Output<OpenDrain>> {
                let mut pin = P0_14 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[14] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_14<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[14] }.reset();
                P0_14 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 14)
            }
        }
        impl<MODE> InputPin for P0_14<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 14)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_14<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 14));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 14));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_14<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 14)) == 0 })
            }
        }
        pub struct P0_15<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_15<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_15<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[15] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_15 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_15<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[15] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_15 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_15<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[15] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_15 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_15<Output<PushPull>> {
                let mut pin = P0_15 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[15] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_15<Output<OpenDrain>> {
                let mut pin = P0_15 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[15] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_15<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[15] }.reset();
                P0_15 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 15)
            }
        }
        impl<MODE> InputPin for P0_15<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 15)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_15<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 15));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 15));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_15<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 15)) == 0 })
            }
        }
        pub struct P0_16<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_16<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_16<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[16] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_16 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_16<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[16] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_16 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_16<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[16] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_16 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_16<Output<PushPull>> {
                let mut pin = P0_16 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[16] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_16<Output<OpenDrain>> {
                let mut pin = P0_16 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[16] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_16<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[16] }.reset();
                P0_16 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 16)
            }
        }
        impl<MODE> InputPin for P0_16<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 16)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_16<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 16));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 16));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_16<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 16)) == 0 })
            }
        }
        pub struct P0_17<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_17<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_17<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[17] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_17 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_17<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[17] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_17 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_17<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[17] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_17 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_17<Output<PushPull>> {
                let mut pin = P0_17 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[17] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_17<Output<OpenDrain>> {
                let mut pin = P0_17 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[17] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_17<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[17] }.reset();
                P0_17 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 17)
            }
        }
        impl<MODE> InputPin for P0_17<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 17)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_17<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 17));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 17));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_17<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 17)) == 0 })
            }
        }
        pub struct P0_18<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_18<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_18<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[18] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_18 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_18<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[18] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_18 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_18<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[18] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_18 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_18<Output<PushPull>> {
                let mut pin = P0_18 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[18] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_18<Output<OpenDrain>> {
                let mut pin = P0_18 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[18] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_18<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[18] }.reset();
                P0_18 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 18)
            }
        }
        impl<MODE> InputPin for P0_18<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 18)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_18<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 18));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 18));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_18<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 18)) == 0 })
            }
        }
        pub struct P0_19<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_19<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_19<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[19] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_19 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_19<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[19] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_19 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_19<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[19] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_19 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_19<Output<PushPull>> {
                let mut pin = P0_19 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[19] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_19<Output<OpenDrain>> {
                let mut pin = P0_19 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[19] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_19<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[19] }.reset();
                P0_19 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 19)
            }
        }
        impl<MODE> InputPin for P0_19<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 19)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_19<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 19));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 19));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_19<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 19)) == 0 })
            }
        }
        pub struct P0_20<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_20<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_20<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[20] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_20 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_20<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[20] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_20 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_20<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[20] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_20 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_20<Output<PushPull>> {
                let mut pin = P0_20 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[20] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_20<Output<OpenDrain>> {
                let mut pin = P0_20 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[20] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_20<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[20] }.reset();
                P0_20 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 20)
            }
        }
        impl<MODE> InputPin for P0_20<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 20)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_20<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 20));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 20));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_20<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 20)) == 0 })
            }
        }
        pub struct P0_21<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_21<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_21<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[21] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_21 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_21<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[21] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_21 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_21<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[21] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_21 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_21<Output<PushPull>> {
                let mut pin = P0_21 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[21] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_21<Output<OpenDrain>> {
                let mut pin = P0_21 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[21] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_21<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[21] }.reset();
                P0_21 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 21)
            }
        }
        impl<MODE> InputPin for P0_21<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 21)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_21<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 21));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 21));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_21<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 21)) == 0 })
            }
        }
        pub struct P0_22<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_22<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_22<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[22] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_22 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_22<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[22] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_22 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_22<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[22] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_22 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_22<Output<PushPull>> {
                let mut pin = P0_22 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[22] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_22<Output<OpenDrain>> {
                let mut pin = P0_22 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[22] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_22<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[22] }.reset();
                P0_22 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 22)
            }
        }
        impl<MODE> InputPin for P0_22<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 22)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_22<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 22));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 22));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_22<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 22)) == 0 })
            }
        }
        pub struct P0_23<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_23<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_23<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[23] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_23 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_23<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[23] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_23 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_23<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[23] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_23 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_23<Output<PushPull>> {
                let mut pin = P0_23 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[23] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_23<Output<OpenDrain>> {
                let mut pin = P0_23 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[23] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_23<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[23] }.reset();
                P0_23 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 23)
            }
        }
        impl<MODE> InputPin for P0_23<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 23)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_23<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 23));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 23));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_23<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 23)) == 0 })
            }
        }
        pub struct P0_24<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_24<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_24<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[24] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_24 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_24<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[24] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_24 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_24<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[24] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_24 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_24<Output<PushPull>> {
                let mut pin = P0_24 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[24] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_24<Output<OpenDrain>> {
                let mut pin = P0_24 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[24] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_24<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[24] }.reset();
                P0_24 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 24)
            }
        }
        impl<MODE> InputPin for P0_24<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 24)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_24<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 24));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 24));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_24<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 24)) == 0 })
            }
        }
        pub struct P0_25<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_25<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_25<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[25] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_25 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_25<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[25] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_25 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_25<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[25] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_25 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_25<Output<PushPull>> {
                let mut pin = P0_25 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[25] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_25<Output<OpenDrain>> {
                let mut pin = P0_25 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[25] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_25<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[25] }.reset();
                P0_25 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 25)
            }
        }
        impl<MODE> InputPin for P0_25<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 25)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_25<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 25));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 25));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_25<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 25)) == 0 })
            }
        }
        pub struct P0_26<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_26<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_26<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[26] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_26 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_26<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[26] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_26 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_26<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[26] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_26 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_26<Output<PushPull>> {
                let mut pin = P0_26 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[26] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_26<Output<OpenDrain>> {
                let mut pin = P0_26 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[26] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_26<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[26] }.reset();
                P0_26 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 26)
            }
        }
        impl<MODE> InputPin for P0_26<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 26)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_26<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 26));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 26));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_26<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 26)) == 0 })
            }
        }
        pub struct P0_27<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_27<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_27<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[27] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_27 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_27<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[27] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_27 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_27<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[27] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_27 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_27<Output<PushPull>> {
                let mut pin = P0_27 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[27] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_27<Output<OpenDrain>> {
                let mut pin = P0_27 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[27] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_27<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[27] }.reset();
                P0_27 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 27)
            }
        }
        impl<MODE> InputPin for P0_27<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 27)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_27<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 27));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 27));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_27<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 27)) == 0 })
            }
        }
        pub struct P0_28<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_28<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_28<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[28] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_28 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_28<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[28] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_28 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_28<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[28] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_28 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_28<Output<PushPull>> {
                let mut pin = P0_28 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[28] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_28<Output<OpenDrain>> {
                let mut pin = P0_28 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[28] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_28<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[28] }.reset();
                P0_28 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 28)
            }
        }
        impl<MODE> InputPin for P0_28<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 28)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_28<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 28));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 28));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_28<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 28)) == 0 })
            }
        }
        pub struct P0_29<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_29<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_29<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[29] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_29 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_29<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[29] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_29 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_29<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[29] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_29 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_29<Output<PushPull>> {
                let mut pin = P0_29 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[29] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_29<Output<OpenDrain>> {
                let mut pin = P0_29 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[29] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_29<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[29] }.reset();
                P0_29 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 29)
            }
        }
        impl<MODE> InputPin for P0_29<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 29)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_29<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 29));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 29));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_29<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 29)) == 0 })
            }
        }
        pub struct P0_30<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_30<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_30<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[30] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_30 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_30<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[30] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_30 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_30<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[30] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_30 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_30<Output<PushPull>> {
                let mut pin = P0_30 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[30] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_30<Output<OpenDrain>> {
                let mut pin = P0_30 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[30] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_30<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[30] }.reset();
                P0_30 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 30)
            }
        }
        impl<MODE> InputPin for P0_30<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 30)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_30<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 30));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 30));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_30<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 30)) == 0 })
            }
        }
        pub struct P0_31<MODE> {
            _mode: PhantomData<MODE>,
        }
        impl<MODE> P0_31<MODE> {
            /// Convert the pin to be a floating input
            pub fn into_floating_input(self) -> P0_31<Input<Floating>> {
                unsafe { &(*P0::ptr()).pin_cnf[31] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_31 { _mode: PhantomData }
            }
            pub fn into_pulldown_input(self) -> P0_31<Input<PullDown>> {
                unsafe { &(*P0::ptr()).pin_cnf[31] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pulldown();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_31 { _mode: PhantomData }
            }
            pub fn into_pullup_input(self) -> P0_31<Input<PullUp>> {
                unsafe { &(*P0::ptr()).pin_cnf[31] }.write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().pullup();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                P0_31 { _mode: PhantomData }
            }
            /// Convert the pin to bepin a push-pull output with normal drive
            pub fn into_push_pull_output(self, initial_output: Level) -> P0_31<Output<PushPull>> {
                let mut pin = P0_31 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                unsafe { &(*P0::ptr()).pin_cnf[31] }.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Convert the pin to be an open-drain output
            ///
            /// This method currently does not support configuring an
            /// internal pull-up or pull-down resistor.
            pub fn into_open_drain_output(
                self,
                config: OpenDrainConfig,
                initial_output: Level,
            ) -> P0_31<Output<OpenDrain>> {
                let mut pin = P0_31 { _mode: PhantomData };
                match initial_output {
                    Level::Low => pin.set_low().unwrap(),
                    Level::High => pin.set_high().unwrap(),
                }
                let pin_cnf = unsafe { &(*P0::ptr()).pin_cnf[31] };
                pin_cnf.write(|w| {
                    w.dir().output();
                    w.input().disconnect();
                    w.pull().disabled();
                    w.drive().variant(config.variant());
                    w.sense().disabled();
                    w
                });
                pin
            }
            /// Disconnects the pin.
            ///
            /// In disconnected mode the pin cannot be used as input or output.
            /// It is primarily useful to reduce power usage.
            pub fn into_disconnected(self) -> P0_31<Disconnected> {
                unsafe { &(*P0::ptr()).pin_cnf[31] }.reset();
                P0_31 { _mode: PhantomData }
            }
            /// Degrade to a generic pin struct, which can be used with peripherals
            pub fn degrade(self) -> Pin<MODE> {
                Pin::new(Port::Port0, 31)
            }
        }
        impl<MODE> InputPin for P0_31<Input<MODE>> {
            type Error = Void;
            fn is_high(&self) -> Result<bool, Self::Error> {
                self.is_low().map(|v| !v)
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).in_.read().bits() & (1 << 31)) == 0 })
            }
        }
        impl<MODE> OutputPin for P0_31<Output<MODE>> {
            type Error = Void;
            /// Set the output as high
            fn set_high(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outset.write(|w| w.bits(1u32 << 31));
                }
                Ok(())
            }
            /// Set the output as low
            fn set_low(&mut self) -> Result<(), Self::Error> {
                unsafe {
                    (*P0::ptr()).outclr.write(|w| w.bits(1u32 << 31));
                }
                Ok(())
            }
        }
        impl<MODE> StatefulOutputPin for P0_31<Output<MODE>> {
            /// Is the output pin set as high?
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                self.is_set_low().map(|v| !v)
            }
            /// Is the output pin set as low?
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*P0::ptr()).out.read().bits() & (1 << 31)) == 0 })
            }
        }
    }
}
