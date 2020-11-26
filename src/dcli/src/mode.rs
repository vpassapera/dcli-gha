use std::fmt;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(PartialEq, Clone, Copy, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum Mode {
    None = 0,
    Story = 2,
    Strike = 3,
    Raid = 4,
    AllPvP = 5,
    Patrol = 6,
    AllPvE = 7,
    Reserved9 = 9,
    Control = 10,
    Reserved11 = 11,
    Clash = 12,
    Reserved13 = 13,
    CrimsonDoubles = 15,
    Nightfall = 16,
    HeroicNightfall = 17,
    AllStrikes = 18,
    IronBanner = 19,
    Reserved20 = 20,
    Reserved21 = 21,
    Reserved22 = 22,
    Reserved24 = 24,
    AllMayhem = 25,
    Reserved26 = 26,
    Reserved27 = 27,
    Reserved28 = 28,
    Reserved29 = 29,
    Reserved30 = 30,
    Supremacy = 31,
    PrivateMatchesAll = 32,
    Survival = 37,
    Countdown = 38,
    TrialsOfTheNine = 39,
    Social = 40,
    TrialsCountdown = 41,
    TrialsSurvival = 42,
    IronBannerControl = 43,
    IronBannerClash = 44,
    IronBannerSupremacy = 45,
    ScoredNightfall = 46,
    ScoredHeroicNightfall = 47,
    Rumble = 48,
    AllDoubles = 49,
    Doubles = 50,
    PrivateMatchesClash = 51,
    PrivateMatchesControl = 52,
    PrivateMatchesSupremacy = 53,
    PrivateMatchesCountdown = 54,
    PrivateMatchesSurvival = 55,
    PrivateMatchesMayhem = 56,
    PrivateMatchesRumble = 57,
    HeroicAdventure = 58,
    Showdown = 59,
    Lockdown = 60,
    Scorched = 61,
    ScorchedTeam = 62,
    Gambit = 63,
    AllPvECompetitive = 64,
    Breakthrough = 65,
    BlackArmoryRun = 66,
    Salvage = 67,
    IronBannerSalvage = 68,
    PvPCompetitive = 69,
    PvPQuickplay = 70,
    ClashQuickplay = 71,
    ClashCompetitive = 72,
    ControlQuickplay = 73,
    ControlCompetitive = 74,
    GambitPrime = 75,
    Reckoning = 76,
    Menagerie = 77,
    VexOffensive = 78,
    NightmareHunt = 79,
    Elimination = 80,
    Momentum = 81,
    Dungeon = 82,
    Sundial = 83,
    TrialsOfOsiris = 84,
}

impl Mode {
    pub fn is_gambit(&self) -> bool {
        *self == Mode::Gambit || *self == Mode::GambitPrime
    }

    pub fn is_nightfall(&self) -> bool {
        *self == Mode::Nightfall ||
        *self == Mode::HeroicNightfall ||
        *self == Mode::ScoredNightfall ||
        *self == Mode::ScoredHeroicNightfall

    }

    pub fn is_crucible(&self) -> bool {
        *self == Mode::AllPvP || 
        *self == Mode::Control || 
        *self == Mode::Clash ||
        *self == Mode::CrimsonDoubles ||
        *self == Mode::IronBanner ||
        *self == Mode::AllMayhem ||
        *self == Mode::Supremacy ||
        *self == Mode::Survival ||
        *self == Mode::Countdown ||
        *self == Mode::TrialsOfTheNine ||
        *self == Mode::TrialsCountdown ||
        *self == Mode::TrialsSurvival ||
        *self == Mode::IronBannerControl ||
        *self == Mode::IronBannerClash ||
        *self == Mode::IronBannerSupremacy ||
        *self == Mode::Rumble ||
        *self == Mode::AllDoubles ||
        *self == Mode::Doubles ||
        *self == Mode::PrivateMatchesClash ||
        *self == Mode::PrivateMatchesControl ||
        *self == Mode::PrivateMatchesSupremacy ||
        *self == Mode::PrivateMatchesCountdown ||
        *self == Mode::PrivateMatchesSurvival ||
        *self == Mode::PrivateMatchesMayhem ||
        *self == Mode::PrivateMatchesRumble ||
        *self == Mode::Showdown ||
        *self == Mode::Lockdown ||
        *self == Mode::Scorched ||
        *self == Mode::ScorchedTeam ||
        *self == Mode::Breakthrough ||
        *self == Mode::Salvage ||
        *self == Mode::IronBannerSalvage ||
        *self == Mode::PvPCompetitive ||
        *self == Mode::PvPQuickplay ||
        *self == Mode::ClashQuickplay ||
        *self == Mode::ClashCompetitive ||
        *self == Mode::ControlQuickplay ||
        *self == Mode::ControlCompetitive ||
        *self == Mode::TrialsOfOsiris
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Mode::None => "None",
            Mode::Story => "Story",
            Mode::Strike => "Strike",
            Mode::Raid => "Raid",
            Mode::AllPvP => "All PvP",
            Mode::Patrol => "Patrol",
            Mode::AllPvE => "All PvE",
            Mode::Reserved9 => "Reserved9",
            Mode::Control => "Control",
            Mode::Reserved11 => "Reserved11",
            Mode::Clash => "Clash",
            Mode::Reserved13 => "Reserved13",
            Mode::CrimsonDoubles => "Crimson Doubles",
            Mode::Nightfall => "Nightfall",
            Mode::HeroicNightfall => "Heroic Nightfall",
            Mode::AllStrikes => "All Strikes",
            Mode::IronBanner => "Iron Banner",
            Mode::Reserved20 => "Reserved20",
            Mode::Reserved21 => "Reserved21",
            Mode::Reserved22 => "Reserved22",
            Mode::Reserved24 => "Reserved24",
            Mode::AllMayhem => "All Mayhem",
            Mode::Reserved26 => "Reserved26",
            Mode::Reserved27 => "Reserved27",
            Mode::Reserved28 => "Reserved28",
            Mode::Reserved29 => "Reserved29",
            Mode::Reserved30 => "Reserved30",
            Mode::Supremacy => "Supremacy",
            Mode::PrivateMatchesAll => "Private Matches All",
            Mode::Survival => "Survival",
            Mode::Countdown => "Countdown",
            Mode::TrialsOfTheNine => "Trials Of The Nine",
            Mode::Social => "Social",
            Mode::TrialsCountdown => "Trials Countdown",
            Mode::TrialsSurvival => "Trials Survival",
            Mode::IronBannerControl => "Iron Banner Control",
            Mode::IronBannerClash => "Iron Banner Clash",
            Mode::IronBannerSupremacy => "Iron Banner Supremacy",
            Mode::ScoredNightfall => "Scored Nightfall",
            Mode::ScoredHeroicNightfall => "Scored Heroic Nightfall",
            Mode::Rumble => "Rumble",
            Mode::AllDoubles => "All Doubles",
            Mode::Doubles => "Doubles",
            Mode::PrivateMatchesClash => "Private Matches Clash",
            Mode::PrivateMatchesControl => "Private Matches Control",
            Mode::PrivateMatchesSupremacy => "Private Matches Supremacy",
            Mode::PrivateMatchesCountdown => "Private Matches Countdown",
            Mode::PrivateMatchesSurvival => "Private Matches Survival",
            Mode::PrivateMatchesMayhem => "Private Matches Mayhem",
            Mode::PrivateMatchesRumble => "Private Matches Rumble",
            Mode::HeroicAdventure => "Heroic Adventure",
            Mode::Showdown => "Showdown",
            Mode::Lockdown => "Lockdown",
            Mode::Scorched => "Scorched",
            Mode::ScorchedTeam => "Scorched Team",
            Mode::Gambit => "Gambit",
            Mode::AllPvECompetitive => "All PvE Competitive",
            Mode::Breakthrough => "Breakthrough",
            Mode::BlackArmoryRun => "Black Armory Run",
            Mode::Salvage => "Salvage",
            Mode::IronBannerSalvage => "Iron BannerS alvage",
            Mode::PvPCompetitive => "PvP Competitive",
            Mode::PvPQuickplay => "PvP Quickplay",
            Mode::ClashQuickplay => "Clash Quickplay",
            Mode::ClashCompetitive => "Clash Competitive",
            Mode::ControlQuickplay => "Control Quickplay",
            Mode::ControlCompetitive => "Control Competitive",
            Mode::GambitPrime => "Gambit Prime",
            Mode::Reckoning => "Reckoning",
            Mode::Menagerie => "Menagerie",
            Mode::VexOffensive => "Vex Offensive",
            Mode::NightmareHunt => "Nightmare Hunt",
            Mode::Elimination => "Elimination",
            Mode::Momentum => "Momentum",
            Mode::Dungeon => "Dungeon",
            Mode::Sundial => "Sundial",
            Mode::TrialsOfOsiris => "Trials Of Osiris",
        };

        write!(f, "{}", out)
    }
}