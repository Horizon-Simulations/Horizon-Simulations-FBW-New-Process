use crate::shared::interpolation;

use uom::si::{
    angular_velocity::revolution_per_minute, f64::*, pressure::psi, ratio::ratio,
    volume::cubic_inch,
};

/// Defines a pump by:
///     displacement map: giving max possible displacement vs current pressure
///     cavitation map: giving the pumping efficiency vs low pressure side air pressure
///     regulated speed: regulation speed value for constant speed pumps
pub struct PumpCharacteristics {
    pressure_map_breakpoints_psi: [f64; 9],
    displacement_map_cubic_inch: [f64; 9],

    air_pressure_map_breakpoints_psi: [f64; 9],
    cavitation_map_ratio: [f64; 9],

    regulated_speed: Option<AngularVelocity>,
}
impl PumpCharacteristics {
    const AIR_PRESSURE_BREAKPTS_PSI: [f64; 9] = [0., 5., 10., 15., 20., 30., 50., 70., 100.];
    const AIR_PRESSURE_CARAC_RATIO: [f64; 9] = [0.0, 0.1, 0.6, 0.8, 0.9, 1., 1., 1., 1.];

    const A319_EDP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2800.0, 2910.0, 3025.0, 3050.0, 3500.0,
    ];
    const A319_EDP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [2.4, 2.4, 2.4, 2.4, 2.4, 2.4, 0.0, 0.0, 0.0];

    const A319_EPUMP_REGULATED_SPEED_RPM: f64 = 7600.0;

    const A319_EPUMP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2175.0, 2850.0, 3080.0, 3100.0, 3500.0,
    ];
    const A319_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.263, 0.263, 0.263, 0.263, 0.263, 0.2, 0.0, 0.0, 0.0];

    const A319_RAT_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2100.0, 2300.0, 2600.0, 2700.0, 3500.0,
    ];
    const A319_RAT_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.5, 0.8, 1.15, 1.15, 1.15, 0.8, 0.3, 0.0, 0.0];

    const A318_EDP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2800.0, 2910.0, 3025.0, 3050.0, 3500.0,
    ];
    const A318_EDP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [2.4, 2.4, 2.4, 2.4, 2.4, 2.4, 0.0, 0.0, 0.0];

    const A318_EPUMP_REGULATED_SPEED_RPM: f64 = 7600.0;

    const A318_EPUMP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2175.0, 2850.0, 3080.0, 3100.0, 3500.0,
    ];
    const A318_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.263, 0.263, 0.263, 0.263, 0.263, 0.2, 0.0, 0.0, 0.0];

    const A318_RAT_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2100.0, 2300.0, 2600.0, 2700.0, 3500.0,
    ];
    const A318_RAT_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.5, 0.8, 1.15, 1.15, 1.15, 0.8, 0.3, 0.0, 0.0];

    const A320_EDP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2800.0, 2910.0, 3025.0, 3050.0, 3500.0,
    ];
    const A320_EDP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [2.4, 2.4, 2.4, 2.4, 2.4, 2.4, 0.0, 0.0, 0.0];

    const A320_EPUMP_REGULATED_SPEED_RPM: f64 = 7600.0;

    const A320_EPUMP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2175.0, 2850.0, 3080.0, 3100.0, 3500.0,
    ];
    const A320_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.263, 0.263, 0.263, 0.263, 0.263, 0.2, 0.0, 0.0, 0.0];

    const A320_RAT_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2100.0, 2300.0, 2600.0, 2700.0, 3500.0,
    ];
    const A320_RAT_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.5, 0.8, 1.15, 1.15, 1.15, 0.8, 0.3, 0.0, 0.0];
    
    const A21N_EDP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2800.0, 2910.0, 3025.0, 3050.0, 3500.0,
    ];
    const A21N_EDP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [2.4, 2.4, 2.4, 2.4, 2.4, 2.4, 0.0, 0.0, 0.0];

    const A21N_EPUMP_REGULATED_SPEED_RPM: f64 = 7600.0;

    const A21N_EPUMP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2175.0, 2850.0, 3080.0, 3100.0, 3500.0,
    ];
    const A21N_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.263, 0.263, 0.263, 0.263, 0.263, 0.2, 0.0, 0.0, 0.0];

    const A21N_RAT_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 1500.0, 2100.0, 2300.0, 2600.0, 2700.0, 3500.0,
    ];
    const A21N_RAT_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.5, 0.8, 1.15, 1.15, 1.15, 0.8, 0.3, 0.0, 0.0];

    const A380_EDP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] = [
        0.0, 500.0, 1000.0, 2900.0, 4790.0, 5150.0, 5225.0, 5350.0, 5500.0,
    ];
    const A380_EDP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [2.8, 2.8, 2.8, 2.8, 2.6, 0.00, 0.0, 0.0, 0.0];

    const A380_EPUMP_REGULATED_SPEED_RPM: f64 = 8000.0;

    const A380_EPUMP_DISPLACEMENT_BREAKPTS_PSI: [f64; 9] =
        [0., 2000., 3000., 4000., 5000., 5100., 5200., 5300., 5350.];
    const A380_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH: [f64; 9] =
        [0.294525, 0.28875, 0.2858625, 0.231, 0.17325, 0., 0., 0., 0.];

    fn new(
        pressure_map_breakpoints_psi: [f64; 9],
        displacement_map_cubic_inch: [f64; 9],

        air_pressure_map_breakpoints_psi: [f64; 9],
        cavitation_map_ratio: [f64; 9],

        regulated_speed: Option<AngularVelocity>,
    ) -> Self {
        Self {
            pressure_map_breakpoints_psi,
            displacement_map_cubic_inch,

            air_pressure_map_breakpoints_psi,
            cavitation_map_ratio,

            regulated_speed,
        }
    }

    pub fn a320_edp() -> Self {
        PumpCharacteristics::new(
            Self::A320_EDP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A320_EDP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn a320_rat() -> Self {
        PumpCharacteristics::new(
            Self::A320_RAT_DISPLACEMENT_BREAKPTS_PSI,
            Self::A320_RAT_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn a320_electric_pump() -> Self {
        PumpCharacteristics::new(
            Self::A320_EPUMP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A320_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            Some(AngularVelocity::new::<revolution_per_minute>(
                Self::A320_EPUMP_REGULATED_SPEED_RPM,
            )),
        )
    }

    pub fn a318_edp() -> Self {
        PumpCharacteristics::new(
            Self::A318_EDP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A318_EDP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn a318_rat() -> Self {
        PumpCharacteristics::new(
            Self::A318_RAT_DISPLACEMENT_BREAKPTS_PSI,
            Self::A318_RAT_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn a318_electric_pump() -> Self {
        PumpCharacteristics::new(
            Self::A318_EPUMP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A318_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            Some(AngularVelocity::new::<revolution_per_minute>(
                Self::A318_EPUMP_REGULATED_SPEED_RPM,
            )),
        )
    }

    pub fn a319_edp() -> Self {
        PumpCharacteristics::new(
            Self::A319_EDP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A319_EDP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn a319_rat() -> Self {
        PumpCharacteristics::new(
            Self::A319_RAT_DISPLACEMENT_BREAKPTS_PSI,
            Self::A319_RAT_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn a319_electric_pump() -> Self {
        PumpCharacteristics::new(
            Self::A319_EPUMP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A319_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            Some(AngularVelocity::new::<revolution_per_minute>(
                Self::A319_EPUMP_REGULATED_SPEED_RPM,
            )),
        )
    }

    pub fn a21n_edp() -> Self {
        PumpCharacteristics::new(
            Self::A21N_EDP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A21N_EDP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn a21n_rat() -> Self {
        PumpCharacteristics::new(
            Self::A21N_RAT_DISPLACEMENT_BREAKPTS_PSI,
            Self::A21N_RAT_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn a21n_electric_pump() -> Self {
        PumpCharacteristics::new(
            Self::A21N_EPUMP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A21N_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            Some(AngularVelocity::new::<revolution_per_minute>(
                Self::A21N_EPUMP_REGULATED_SPEED_RPM,
            )),
        )
    }

    pub fn a380_electric_pump() -> Self {
        PumpCharacteristics::new(
            Self::A380_EPUMP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A380_EPUMP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            Some(AngularVelocity::new::<revolution_per_minute>(
                Self::A380_EPUMP_REGULATED_SPEED_RPM,
            )),
        )
    }

    pub fn a380_edp() -> Self {
        PumpCharacteristics::new(
            Self::A380_EDP_DISPLACEMENT_BREAKPTS_PSI,
            Self::A380_EDP_DISPLACEMENT_MAP_CUBIC_INCH,
            Self::AIR_PRESSURE_BREAKPTS_PSI,
            Self::AIR_PRESSURE_CARAC_RATIO,
            None,
        )
    }

    pub fn current_displacement(&self, pressure: Pressure) -> Volume {
        Volume::new::<cubic_inch>(interpolation(
            &self.pressure_map_breakpoints_psi,
            &self.displacement_map_cubic_inch,
            pressure.get::<psi>(),
        ))
    }

    pub fn cavitation_efficiency(&self, air_pressure: Pressure, heat_factor: Ratio) -> Ratio {
        Ratio::new::<ratio>(
            (1. - heat_factor.get::<ratio>())
                * interpolation(
                    &self.air_pressure_map_breakpoints_psi,
                    &self.cavitation_map_ratio,
                    air_pressure.get::<psi>(),
                ),
        )
    }

    pub fn regulated_speed(&self) -> AngularVelocity {
        self.regulated_speed.unwrap_or_default()
    }
}
