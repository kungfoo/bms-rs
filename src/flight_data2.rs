use crate::flight_data2::bits::*;

use crate::MemoryFile;

mod bits;

#[repr(u8)]
#[derive(Debug, Default)]
pub enum TacanSources {
    #[default]
    UFC = 0,
    AUX = 1,
    NumberOfSources = 2,
}

// u32
#[repr(u32)]
#[derive(Debug, Default)]
pub enum CmdsModes {
    #[default]
    CmdsOFF = 0,
    CmdsSTBY = 1,
    CmdsMAN = 2,
    CmdsSEMI = 3,
    CmdsAUTO = 4,
    CmdsBYP = 5,
}
// u8
#[repr(u8)]
#[derive(Debug, Default)]
pub enum NavModes {
    #[default]
    IlsTacan = 0,
    TACAN = 1,
    NAV = 2,
    IlsNav = 3,
}

#[repr(u8)]
#[derive(Debug, Default)]
pub enum FlyStates {
    #[default]
    InUi = 0, // UI      - in the UI
    LOADING = 1, // UI>3D   - loading the sim data
    WAITING = 2, // UI>3D   - waiting for other players
    FLYING = 3,  // 3D      - flying
    DEAD = 4,    // 3D>Dead - dead, waiting to respawn
    UNKNOWN = 5, // ???
}

/// RTT area indices
#[repr(u8)]
#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum RTTAreas {
    #[default]
    RttHud = 0,
    RttPfl = 1,
    RttDed = 2,
    RttRwr = 3,
    RttMfdleft = 4,
    RttMfdright = 5,
    RttHms = 6,
    RttNoOfAreas = 7,
}

// instrument backlight brightness
#[repr(u8)]
#[derive(Debug, Default)]
pub enum InstrLight {
    #[default]
    InstrLightOff = 0,
    InstrLightDim = 1,
    InstrLightBrt = 2,
}

// flood console brightness
#[repr(u8)]
#[derive(Debug, Default)]
pub enum FloodConsole {
    #[default]
    FloodConsoleOff = 0,
    FloodConsole1 = 1,
    FloodConsole2 = 2,
    FloodConsole3 = 3,
    FloodConsole4 = 4,
    FloodConsole5 = 5,
    FloodConsole6 = 6,
}

// RWR jamming statesi
// u8
#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub enum JammingStates {
    #[default]
    JammedNo = 0,
    JammedYes = 1,
    JammedShould = 2,
}

const RWRINFO_SIZE: usize = 512;
const MAX_CALLSIGNS: usize = 32;
const CALLSIGN_LEN: usize = 12;
const MAX_ECM_PROGRAMS: usize = 5;
const MAX_RWR_OBJECTS: usize = 40;

#[repr(C)]
#[derive(Debug)]
pub struct FlightData2 {
    // VERSION 1
    pub nozzle_pos2: f32,   // Ownship engine nozzle2 percent open (0-100)
    pub rpm2: f32,          // Ownship engine rpm2 (Percent 0-103)
    pub ftit2: f32,         // Ownship Forward Turbine Inlet Temp2 (Degrees C)
    pub oil_pressure2: f32, // Ownship Oil Pressure2 (Percent 0-100)
    pub nav_mode: NavModes, // current mode selected for HSI/eHSI (added in BMS4)
    pub aauz: f32,          // Ownship barometric altitude given by AAU (depends on calibration)
    //     [MarshalAs(UnmanagedType.ByValArray, SizeConst = (int)TacanSources.NUMBER_OF_SOURCES)]
    pub tacan_info: [TacanSources; TacanSources::NumberOfSources as usize], // Tacan band/mode settings for UFC and AUX COMM

    //VERSION 2/7
    pub alt_cal_reading: i32, // barometric altitude calibration (depends on CalType)
    pub alt_bits: AltBits,    // various altimeter bits, see AltBits enum for details
    pub power_bits: PowerBits, // Ownship power bus / generator states, see PowerBits enum for details
    pub blink_bits: BlinkBits, // Cockpit indicator lights blink status, see BlinkBits enum for details
    // NOTE: these bits indicate only *if* a lamp is blinking, in addition to the
    // existing on/off bits. It's up to the external program to implement the
    // *actual* blinking.
    pub cmds_mode: CmdsModes, // Ownship CMDS mode state, see CmdsModes enum for details
    pub uhf_panel_preset: i32, // BUP UHF channel preset (F16), radio 1 preset (other aircraft).

    // VERSION 3
    pub uhf_panel_frequency: i32, // BUP UHF channel frequency, radio 1 frequency (other aircraft).
    pub cabin_alt: f32,           // Ownship cabin altitude
    pub hyd_pressure_a: f32,      // Ownship Hydraulic Pressure A
    pub hyd_pressure_b: f32,      // Ownship Hydraulic Pressure B
    pub current_time: i32,        // Current time in seconds (max 60 * 60 * 24)
    pub vehicle_acd: i16, // Ownship ACD index number, i.e. which aircraft type are we flying.
    pub version_num: i32, // Version of FlightData2 mem area

    // VERSION 4
    pub fuel_flow2: f32, // Ownship fuel flow2 (Lbs/Hour)

    // VERSION 5 / 8
    // [MarshalAs(UnmanagedType.ByValArray, SizeConst = RWRINFO_SIZE)]
    pub rwr_info: [u8; RWRINFO_SIZE], //[512] New RWR Info
    pub lef_pos: f32,                 // Ownship LEF position
    pub tef_pos: f32,                 // Ownship TEF position

    // VERSION 6
    pub vtol_pos: f32, // Ownship VTOL exhaust angle

    // VERSION 9
    pub pilots_online: u8, // Number of pilots in an MP session

    //[MarshalAs(UnmanagedType.ByValArray, SizeConst = MAX_CALLSIGNS * CALLSIGN_LEN)]
    pub pilots_callsign: [u8; MAX_CALLSIGNS * CALLSIGN_LEN], // [MAX_CALLSIGNS][CALLSIGN_LEN] List of pilots callsign connected to an MP session

    // [MarshalAs(UnmanagedType.ByValArray, SizeConst = MAX_CALLSIGNS)]
    pub pilots_status: [FlyStates; MAX_CALLSIGNS], // [MAX_CALLSIGNS] Status of the MP pilots, see enum FlyStates

    //VERSION 10
    pub bump_intensity: f32, // Intensity of a "bump" while taxiing/rolling, 0..1

    //VERSION 11
    pub latitude: f32,  // Ownship latitude in degrees (as known by avionics)
    pub longitude: f32, // Ownship longitude in degrees (as known by avionics)

    //VERSION 12
    //[MarshalAs(UnmanagedType.ByValArray, SizeConst = 2)]
    pub rtt_size: [u16; 2], // RTT overall width and height
    //[MarshalAs(UnmanagedType.ByValArray, SizeConst = (int)RTT_areas.RTT_noOfAreas * 4)]
    pub rtt_area: [u16; (RTTAreas::RttNoOfAreas as usize) * 4], // For each area: left/top/right/bottom

    // VERSION 13
    pub iff_backup_mode1_digit1: u8, // IFF panel backup Mode1 digit 1
    pub iff_backup_mode1_digit2: u8, // IFF panel backup Mode1 digit 2
    pub iff_backup_mode3_adigit1: u8, // IFF panel backup Mode3A digit 1
    pub iff_backup_mode3_adigit2: u8, // IFF panel backup Mode3A digit 2

    // VERSION 14
    pub instr_light: InstrLight, // (unsigned char) current instrument backlight brightness setting, see InstrLight enum for details

    // VERSION 15
    pub betty_bits: BettyBits,  // see BettyBits enum for details
    pub misc_bits: MiscBits,    // see MiscBits enum for details
    pub ralt: f32, // radar altitude (only valid/ reliable if MiscBit "RALT_Valid" is set)
    pub bingo_fuel: f32, // bingo fuel level
    pub cara_alow: f32, // cara alow setting
    pub bullseye_x: f32, // bullseye X in sim coordinates (same as ownship, i.e. North (Ft))
    pub bullseye_y: f32, // bullseye Y in sim coordinates (same as ownship, i.e. East (Ft))
    pub bmsversion_major: i32, // E.g.  4.
    pub bmsversion_minor: i32, //         34.
    pub bmsversion_micro: i32, //            1
    pub bmsbuild_number: i32, //              build 20050
    pub string_area_size: u32, // the overall size of the StringData/FalconSharedMemoryAreaString shared memory area
    pub string_area_time: u32, // last time the StringData/FalconSharedMemoryAreaString shared memory area has been changed - you only need to re-read the string shared mem if this changes
    pub drawing_area_size: u32, // the overall size of the DrawingData/FalconSharedMemoryAreaDrawing area

    // VERSION 16
    pub turn_rate: f32, // actual turn rate (no delay or dampening) in degrees/second

    // VERSION 18
    pub flood_console: FloodConsole, // (unsigned char) current floodconsole brightness setting, see FloodConsole enum for details

    // VERSION 19
    pub mag_deviation_system: f32, // current mag deviation of the system
    pub mag_deviation_real: f32,   // current mag deviation of the system

    // [MarshalAs(UnmanagedType.ByValArray, SizeConst = MAX_ECM_PROGRAMS)]
    pub ecm_bits: [EcmBits; MAX_ECM_PROGRAMS], // see EcmBits enum for details - Note: these are currently not combinable bits, but mutually exclusive states!

    pub ecm_oper_state: EcmOperStates, // (unsigned char) see enum EcmOperStates for details

    //[MarshalAs(UnmanagedType.ByValArray, SizeConst = MAX_RWR_OBJECTS)]
    pub rwr_jamming_status: [JammingStates; MAX_RWR_OBJECTS], // (unsigned) char see enum JammingStates for details

    // VERSION 20
    pub radio2_preset: i32,    // Radio 2 channel preset (if present).
    pub radio2_frequency: i32, // Radio 2 channel frequency (if present).

    // IFF transponder currently active (as seen from outside) codes, negative for OFF or n/a
    pub iff_transponder_active_code1: u8,    // mode 1
    pub iff_transponder_active_code2: i16,   // mode 2
    pub iff_transponder_active_code3_a: i16, // mode 3A
    pub iff_transponder_active_code_c: i16,  // mode C
    pub iff_transponder_active_code4: i16,   // mode 4; assumes the correct codeword

    // VERSION 21
    pub tacan_ils_frequency: i32, // Tacan ILS (110.30 = 11030). Valid interval [108.10, 111.95].

    // VERSION 22
    pub desired_rtt_fps: i32, // The configured RTT export FPS value, g_nRTTExport_FPS

    // SIDE SLIP ANGLE
    pub side_slipdeg: f32, // ADI side Slip
}

impl std::ops::Deref for FlightData2 {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.desired_rtt_fps
    }
}

unsafe impl Send for FlightData2 {}
unsafe impl Sync for FlightData2 {}

impl FlightData2 {
    pub fn new<'a>() -> Result<MemoryFile<'a, Self>, Box<dyn std::error::Error + Send + Sync>> {
        let file = unsafe { MemoryFile::<'a, Self>::new("FalconSharedMemoryArea2")? };
        Ok(file)
    }
}

impl Default for FlightData2 {
    fn default() -> Self {
        Self {
            nozzle_pos2: Default::default(),
            rpm2: Default::default(),
            ftit2: Default::default(),
            oil_pressure2: Default::default(),
            nav_mode: Default::default(),
            aauz: Default::default(),
            tacan_info: Default::default(),
            alt_cal_reading: Default::default(),
            alt_bits: Default::default(),
            power_bits: Default::default(),
            blink_bits: Default::default(),
            cmds_mode: Default::default(),
            uhf_panel_preset: Default::default(),
            uhf_panel_frequency: Default::default(),
            cabin_alt: Default::default(),
            hyd_pressure_a: Default::default(),
            hyd_pressure_b: Default::default(),
            current_time: Default::default(),
            vehicle_acd: Default::default(),
            version_num: Default::default(),
            fuel_flow2: Default::default(),
            rwr_info: [0; RWRINFO_SIZE],
            lef_pos: Default::default(),
            tef_pos: Default::default(),
            vtol_pos: Default::default(),
            pilots_online: Default::default(),
            pilots_callsign: [0; MAX_CALLSIGNS * CALLSIGN_LEN],
            pilots_status: Default::default(),
            bump_intensity: Default::default(),
            latitude: Default::default(),
            longitude: Default::default(),
            rtt_size: Default::default(),
            rtt_area: Default::default(),
            iff_backup_mode1_digit1: Default::default(),
            iff_backup_mode1_digit2: Default::default(),
            iff_backup_mode3_adigit1: Default::default(),
            iff_backup_mode3_adigit2: Default::default(),
            instr_light: Default::default(),
            betty_bits: Default::default(),
            misc_bits: Default::default(),
            ralt: Default::default(),
            bingo_fuel: Default::default(),
            cara_alow: Default::default(),
            bullseye_x: Default::default(),
            bullseye_y: Default::default(),
            bmsversion_major: Default::default(),
            bmsversion_minor: Default::default(),
            bmsversion_micro: Default::default(),
            bmsbuild_number: Default::default(),
            string_area_size: Default::default(),
            string_area_time: Default::default(),
            drawing_area_size: Default::default(),
            turn_rate: Default::default(),
            flood_console: Default::default(),
            mag_deviation_system: Default::default(),
            mag_deviation_real: Default::default(),
            ecm_bits: Default::default(),
            ecm_oper_state: Default::default(),
            rwr_jamming_status: [JammingStates::default(); MAX_RWR_OBJECTS],
            radio2_preset: Default::default(),
            radio2_frequency: Default::default(),
            tacan_ils_frequency: Default::default(),
            iff_transponder_active_code1: Default::default(),
            iff_transponder_active_code2: Default::default(),
            iff_transponder_active_code3_a: Default::default(),
            iff_transponder_active_code_c: Default::default(),
            iff_transponder_active_code4: Default::default(),
            desired_rtt_fps: Default::default(),
            side_slipdeg: Default::default(),
        }
    }
}
