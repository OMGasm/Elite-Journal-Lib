// use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Result;
use serde_json::Value;
use serde_repr::Deserialize_repr;
use serde_with::with_prefix;
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Deserialize, Debug)]
#[serde(tag = "event")]
enum Event {
    Fileheader(Fileheader),
    Commander(Commander),
    Materials(Materials),
    Rank(Rank),
    Progress(Progress),
    #[serde(untagged)]
    Unknown(HashMap<String, Value>),
}

#[derive(Deserialize, Debug)]
struct Fileheader {
    part: i32,
    #[serde(rename = "Odyssey")]
    odyssey: bool,
    #[serde(flatten)]
    version: Version,
}

#[derive(Deserialize, Debug)]
struct Commander {
    #[serde(rename = "FID")]
    fid: Fid,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Deserialize)]
struct Fid(String);
impl std::fmt::Debug for Fid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "F~~~~~~")
    }
}

#[derive(Deserialize, Debug)]
struct Materials {
    #[serde(flatten)]
    stuff: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct Item {
    name: String,
    name_localized: String,
    count: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Rank {
    combat: CombatRank,
    trade: TradeRank,
    explore: ExplorationRank,
    soldier: MercenaryRank,
    exobiologist: ExobiologistRank,
    empire: EmpireRank,
    federation: FederationRank,
    #[serde(rename = "CQC")]
    cqc: u8,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum CombatRank {
    Harmless,
    MostlyHarmless,
    Novice,
    Competent,
    Expert,
    Master,
    Dangerous,
    Deadly,
    Elite,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum TradeRank {
    Penniless,
    MostlyPenniless,
    Peddler,
    Dealer,
    Merchant,
    Broker,
    Entrepreneur,
    Tycoon,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum ExplorationRank {
    Aimless,
    MostlyAimless,
    Scout,
    Surveyor,
    Trailblazer,
    Pathfinder,
    Ranger,
    Pioneer,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum MercenaryRank {
    Defenceless,
    MostlyDefenceless,
    Rookie,
    Soldier,
    Gunslinger,
    Warrior,
    Gladiator,
    Deadeye,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum ExobiologistRank {
    Directionless,
    MostlyDirectionless,
    Compiler,
    Collector,
    Cataloguer,
    Taxonomist,
    Ecologist,
    Geneticist,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum FederationRank {
    None,
    Recruit,
    Cadet,
    Midshipman,
    PettyOfficer,
    ChiefPettyOfficer,
    WarrantOfficer,
    Ensign,
    Lieutenant,
    LieutenantCommander,
    PostCommander,
    PostCaptain,
    RearAdmiral,
    ViceAdmiral,
    Admiral,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum EmpireRank {
    None,
    Outsider,
    Serf,
    Master,
    Squire,
    Knight,
    Lord,
    Baron,
    Viscount,
    Count,
    Earl,
    Marquis,
    Duke,
    Prince,
    King,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Progress {
    combat: u32,
    trade: u32,
    explore: u32,
    soldier: u32,
    exobiologist: u32,
    empire: u32,
    federation: u32,
    #[serde(rename = "CQC")]
    cqc: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Reputation {
    empire: f32,
    federation: f32,
    independent: f32,
    alliance: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct EngineerProgress {
    engineers: Vec<Engineer>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Engineer {
    engineer: String,
    engineer_id: String,
    progress: EngineerDiscovery, // ?
}

#[derive(Deserialize, Debug)]
enum EngineerDiscovery {
    Known,
    Unknown(HashMap<String, Value>),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct LoadGame {
    #[serde(flatten)]
    commander: Commander,
    #[serde(flatten)]
    game_type: GameType,
    ship: Ship,
    credits: u64,
    loan: u64,
    #[serde(flatten)]
    version: Version,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Version {
    game_version: String,
    build: String,
    language: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Ship {
    ship: String,
    #[serde(rename = "Ship_Localized")]
    ship_localized: String,
    #[serde(rename = "Ship_ID")]
    ship_id: String,
    #[serde(deserialize_with = "empty_string_to_none")]
    ship_name: Option<String>,
    #[serde(deserialize_with = "empty_string_to_none")]
    ship_ident: Option<String>,
    fuel_level: f32,
    fuel_capacity: f32,
}

fn empty_string_to_none<'de, D>(de: D) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(de)?;
    Ok(s.filter(|s| !s.is_empty()).map(|s| s.to_owned()))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct GameType {
    horizons: bool,
    odyssey: bool,
    game_mode: GameMode,
}

#[derive(Deserialize, Debug)]
enum GameMode {
    Open,
    Private,
    Solo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct Statistics {
    bank_account: BankStats,
    combat: CombatStats,
    crime: CrimeStats,
    smuggling: SmugglingStats,
    trading: TradingStats,
    mining: MiningStats,
    exploration: ExplorationStats,
    #[serde(with = "prefix_passenger_stats")]
    passengers: PassengerStats,
    crafting: CraftingStats,
    crew: CrewStats,
    multicrew: MulticrewStats,
    material_trader_stats: MaterialTraderStats,
    #[serde(rename = "CQC")]
    cqc: CQCStats,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct BankStats {
    current_wealth: u64,
    spent_on_ships: u64,
    spent_on_outfitting: u64,
    spent_on_repairs: u64,
    spent_on_fuel: u64,
    spent_on_ammo_consumables: u64,
    insurance_claims: u64,
    spent_on_insurance: u64,
    owned_ship_count: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct CombatStats {
    bounties_claimed: u64,
    bounty_hunting_profit: u64,
    combat_bonds: u64,
    combat_bond_profits: u64,
    assasinations: u64,
    assasination_profits: u64,
    highest_single_reward: u64,
    skimmers_killed: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct CrimeStats {
    notoriety: f64,
    fines: u64,
    total_fines: u64,
    bounties_received: u64,
    total_bounties: u64,
    highest_bounty: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct SmugglingStats {
    black_markets_traded_with: u64,
    black_markets_profits: u64,
    resources_smuggled: u64,
    average_profit: u64,
    highest_single_transaction: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct TradingStats {
    markets_traded_with: u64,
    market_profits: u64,
    resources_traded: u64,
    average_profit: f64,
    highest_single_transaction: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct MiningStats {
    mining_profits: u64,
    quantity_mined: u64,
    materials_collected: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct ExplorationStats {
    systems_visited: u64,
    exploration_profits: u64,
    planets_scanned_to_level_2: u64,
    planets_scanned_to_level_3: u64,
    efficient_scans: u64,
    highest_payout: u64,
    total_hyperspace_distance: u64,
    total_hyperspace_jumps: u64,
    greatest_distance_from_start: f64,
    times_played: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct PassengerStats {
    bulk: u64,
    #[serde(rename = "VIP")]
    vip: u64,
    delivered: u64,
    ejected: u64,
}
with_prefix!(prefix_passenger_stats "Passengers_Missions_");

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SearchAndRescueStats {
    traded: u64,
    profit: u64,
    count: u64,
}
with_prefix!(prefix_search_and_rescue_stats "SearchRescue_");

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct CraftingStats {
    count_of_used_engineers: u64,
    recipes_generated: u64,
    recipes_generated_rank_1: u64,
    recipes_generated_rank_2: u64,
    recipes_generated_rank_3: u64,
    recipes_generated_rank_4: u64,
    recipes_generated_rank_5: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct CrewStats {
    total_wages: u64,
    hired: u64,
    fired: u64,
    died: u64,
}
with_prefix!(prefix_crew_stats "NpcCrew_");

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct MulticrewStats {
    time_total: u64,
    gunner_time_total: u64,
    fighter_time_total: u64,
    credits_total: u64,
    fines_total: u64,
}
with_prefix!(prefix_multicrew_stats "Multicrew_");

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct MaterialTraderStats {
    trades_completed: u64,
    materials_traded: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct CQCStats {
    time_played: u64,
    kd: u64,
    kills: u64,
    wl: u64,
}
with_prefix!(prefix_cqc_stats "CQC_");

#[derive(Deserialize, Debug)]
#[serde(rename_all = "Pascal_Snake_Case")]
struct ReceiveText {
    from: String,
    message: String,
    message_localised: Option<String>,
    channel: Channel,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Channel {
    Npc,
    Player,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SendText {
    to: String,
    message: String,
    sent: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Location {
    docked: bool,
    taxi: bool,
    multicrew: bool,
    star_system: String,
    star_pos: StarPos,
    system: StarSystem,
}

#[derive(Deserialize, Debug)]
struct StarPos {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Deserialize, Debug)]
struct StarSystem {
    address: u64,
    allegiance: Allegiance,
    economy: SysEconomy,
}

#[derive(Deserialize, Debug)]
struct SysEconomy {}

#[derive(Deserialize, Debug)]
enum Allegiance {
    Independent,
    Federation,
    PilotsFederation,
    #[serde(untagged)]
    None,
}

const H: &str = r#"{ "timestamp":"2024-07-04T08:14:46Z", "event":"Fileheader", "part":1, "language":"English/UK", "Odyssey":true, "gameversion":"4.0.0.1806", "build":"r302447/r0 " }"#;
const C: &str = r#"{ "timestamp":"2024-07-04T08:15:41Z", "event":"Commander", "FID":"F1908163", "Name":"OMGasm" }"#;

fn main() {
    let mut file = String::new();
    File::open("Journal.log")
        .expect("oops file")
        .read_to_string(&mut file)
        .unwrap();
    let lines = file.lines();
    for line in lines {
        let ev: Result<Event> = serde_json::from_str(line);
        match ev {
            Ok(ev) => {
                println!("{ev:?}\n")
            }
            Err(e) => {
                println!("{e:?}\n")
            }
        };
    }
}
