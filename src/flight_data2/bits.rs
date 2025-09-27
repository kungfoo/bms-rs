use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct TacanBits : u8 {
        const Band = 0x01;   // true in this bit position if band is X
        const Mode = 0x02;   // true in this bit position if domain is air to air
    }
}

bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct AltBits : u32{
        const CalType = 0x01; // true if calibration in inches of Mercury (Hg), false if in hectoPascal (hPa)
        const PneuFlag = 0x02;	// true if PNEU flag is visible
    }
}

bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct PowerBits : u32 {
        const BusPowerBattery = 0x01; // true if at least the battery bus is powered
        const BusPowerEmergency = 0x02;   // true if at least the emergency bus is powered
        const BusPowerEssential = 0x04;   // true if at least the essential bus is powered
        const BusPowerNonEssential = 0x08;    // true if at least the non-essential bus is powered
        const MainGenerator = 0x10;   // true if the main generator is online
        const StandbyGenerator = 0x20;    // true if the standby generator is online
        const JetFuelStarter = 0x40;	// true if JFS is running, can be used for magswitch
    }
}

bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct BlinkBits : u32 {
        const OuterMarker = 0x01; // defined in HsiBits    - slow flashing for outer marker
        const MiddleMarker = 0x02;    // defined in HsiBits    - fast flashing for middle marker
        const PROBEHEAT = 0x04;   // defined in LightBits2 - probeheat system is tested
        const AuxSrch = 0x08; // defined in LightBits2 - search function in NOT activated and a search radar is painting ownship
        const Launch = 0x10;  // defined in LightBits2 - missile is fired at ownship
        const PriMode = 0x20; // defined in LightBits2 - priority mode is enabled but more than 5 threat emitters are detected
        const Unk = 0x40; // defined in LightBits2 - unknown is not active but EWS detects unknown radar

        // not working yet, defined for future use
        const Elec_Fault = 0x80;  // defined in LightBits3 - non-resetting fault
        const OXY_BROW = 0x100;   // defined in LightBits  - monitor fault during Obogs
        const EPUOn = 0x200;  // defined in LightBits3 - abnormal EPU operation

        // working
        const JFSOn_Slow = 0x400; // defined in LightBits3 - slow blinking: non-critical failure
        const JFSOn_Fast = 0x800;	// defined in LightBits3 - fast blinking: critical failure

        // VERSION 19
        const ECM_Oper = 0x1000;  // defined in EcmOperStates - system warming up
    }
}

// ECM indicator states
bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct EcmBits : u32 {
        const ECM_UNPRESSED_NO_LIT  = 0x01;
        const ECM_UNPRESSED_ALL_LIT = 0x02;
        const ECM_PRESSED_NO_LIT    = 0x04;
        const ECM_PRESSED_STANDBY   = 0x08;
        const ECM_PRESSED_ACTIVE    = 0x10;
        const ECM_PRESSED_TRANSMIT  = 0x20;
        const ECM_PRESSED_FAIL      = 0x40;
        const ECM_PRESSED_ALL_LIT   = 0x80;
    }
}

// IDIAS operating states
bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct EcmOperStates : u8 {
        const ECM_OPER_NO_LIT = 0;
        const ECM_OPER_STDBY = 1;
        const ECM_OPER_ACTIVE = 2;
        const ECM_OPER_ALL_LIT = 3;
    }
}

bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct MiscBits : u32 {
        const RALT_Valid = 0x01; // indicates weather the RALT reading is valid/reliable
        const Flcs_Flcc_A = 0x02;
        const Flcs_Flcc_B = 0x04;
        const Flcs_Flcc_C = 0x08;
        const Flcs_Flcc_D = 0x10;
        const SolenoidStatus = 0x20; // 0 not powered or failed or WOW, 1 is working OK

        const AllLampBitsFlccOn = 0x1e; // Not bit by itself! This is the check mask for ALL the Flcs bits
    }
}
