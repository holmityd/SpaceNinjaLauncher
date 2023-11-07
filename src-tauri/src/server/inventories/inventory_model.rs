use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::server::{
    mods::mods_model::{RawUpgrade, Upgrade},
    suits::suit_model::Suit,
};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "accountOwnerId")]
    pub account_owner_id: Option<ObjectId>,
    #[serde(rename = "SubscribedToEmails")]
    pub subscribed_to_emails: Option<i32>,
    #[serde(rename = "Created")]
    pub created: Option<Date>,
    #[serde(rename = "RewardSeed")]
    pub reward_seed: Option<serde_json::Value>,
    #[serde(rename = "RegularCredits")]
    pub regular_credits: Option<i32>,
    #[serde(rename = "PremiumCredits")]
    pub premium_credits: Option<i32>,
    #[serde(rename = "PremiumCreditsFree")]
    pub premium_credits_free: Option<i32>,
    #[serde(rename = "FusionPoints")]
    pub fusion_points: Option<i32>,
    #[serde(rename = "SuitBin")]
    pub suit_bin: Option<EquipmentBin>,
    #[serde(rename = "WeaponBin")]
    pub weapon_bin: Option<EquipmentBin>,
    #[serde(rename = "SentinelBin")]
    pub sentinel_bin: Option<EquipmentBin>,
    #[serde(rename = "SpaceSuitBin")]
    pub space_suit_bin: Option<EquipmentBin>,
    #[serde(rename = "SpaceWeaponBin")]
    pub space_weapon_bin: Option<EquipmentBin>,
    #[serde(rename = "PvpBonusLoadoutBin")]
    pub pvp_bonus_loadout_bin: Option<EquipmentBin>,
    #[serde(rename = "PveBonusLoadoutBin")]
    pub pve_bonus_loadout_bin: Option<EquipmentBin>,
    #[serde(rename = "RandomModBin")]
    pub random_mod_bin: Option<EquipmentBin>,
    #[serde(rename = "TradesRemaining")]
    pub trades_remaining: Option<i32>,
    #[serde(rename = "DailyAffiliation")]
    pub daily_affiliation: Option<i32>,
    #[serde(rename = "DailyAffiliationPvp")]
    pub daily_affiliation_pvp: Option<i32>,
    #[serde(rename = "DailyAffiliationLibrary")]
    pub daily_affiliation_library: Option<i32>,
    #[serde(rename = "DailyFocus")]
    pub daily_focus: Option<i32>,
    #[serde(rename = "GiftsRemaining")]
    pub gifts_remaining: Option<i32>,
    #[serde(rename = "MiscItems")]
    pub misc_items: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ChallengesFixVersion")]
    pub challenges_fix_version: Option<i32>,
    #[serde(rename = "ChallengeProgress")]
    pub challenge_progress: Option<Vec<ChallengeProgress>>,
    #[serde(rename = "RawUpgrades")]
    pub raw_upgrades: Option<Vec<RawUpgrade>>,
    #[serde(rename = "ReceivedStartingGear")]
    pub received_starting_gear: Option<bool>,
    #[serde(rename = "Suits")]
    pub suits: Option<Vec<Suit>>,
    #[serde(rename = "LongGuns")]
    pub long_guns: Option<Vec<Weapon>>,
    #[serde(rename = "Pistols")]
    pub pistols: Option<Vec<Weapon>>,
    #[serde(rename = "Melee")]
    pub melee: Option<Vec<Weapon>>,
    #[serde(rename = "Ships")]
    pub ships: Option<Vec<Item>>,
    #[serde(rename = "QuestKeys")]
    pub quest_keys: Option<Vec<QuestKey>>,
    #[serde(rename = "FlavourItems")]
    pub flavour_items: Option<Vec<FlavourItem>>,
    #[serde(rename = "Scoops")]
    pub scoops: Option<Vec<Item>>,
    #[serde(rename = "TrainingRetriesLeft")]
    pub training_retries_left: Option<i32>,
    #[serde(rename = "CurrentLoadOutIds")]
    pub current_loadout_ids: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Missions")]
    pub missions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "XPInfo")]
    pub xp_info: Option<Vec<XPInfo>>,
    #[serde(rename = "Recipes")]
    pub recipes: Option<Vec<Recipe>>,
    #[serde(rename = "WeaponSkins")]
    pub weapon_skins: Option<Vec<Item>>,
    #[serde(rename = "PendingRecipes")]
    pub pending_recipes: Option<Vec<Recipe>>,
    #[serde(rename = "TrainingDate")]
    pub training_date: Option<Date>,
    #[serde(rename = "PlayerLevel")]
    pub player_level: Option<i32>,
    #[serde(rename = "Upgrades")]
    pub upgrades: Option<Vec<Upgrade>>,
    #[serde(rename = "EquippedGear")]
    pub equipped_gear: Option<Vec<serde_json::Value>>,
    #[serde(rename = "DeathMarks")]
    pub death_marks: Option<Vec<serde_json::Value>>,
    #[serde(rename = "FusionTreasures")]
    pub fusion_treasures: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CompletedAlerts")]
    pub completed_alerts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Consumables")]
    pub consumables: Option<Vec<Consumable>>,
    #[serde(rename = "LevelKeys")]
    pub level_keys: Option<Vec<serde_json::Value>>,
    #[serde(rename = "TauntHistory")]
    pub taunt_history: Option<Vec<serde_json::Value>>,
    #[serde(rename = "StoryModeChoice")]
    pub story_mode_choice: Option<serde_json::Value>,
    #[serde(rename = "PeriodicMissionCompletions")]
    pub periodic_mission_completions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "KubrowPetEggs")]
    pub kubrow_pet_eggs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "LoreFragmentScans")]
    pub lore_fragment_scans: Option<Vec<serde_json::Value>>,
    #[serde(rename = "EquippedEmotes")]
    pub equipped_emotes: Option<Vec<serde_json::Value>>,
    #[serde(rename = "PendingTrades")]
    pub pending_trades: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Boosters")]
    pub boosters: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Affiliations")]
    pub affiliations: Option<Vec<serde_json::Value>>,
    #[serde(rename = "QualifyingInvasions")]
    pub qualifying_invasions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "FactionScores")]
    pub faction_scores: Option<Vec<serde_json::Value>>,
    #[serde(rename = "SpaceSuits")]
    pub space_suits: Option<Vec<serde_json::Value>>,
    #[serde(rename = "SpaceMelee")]
    pub space_melee: Option<Vec<serde_json::Value>>,
    #[serde(rename = "SpaceGuns")]
    pub space_guns: Option<Vec<serde_json::Value>>,
    #[serde(rename = "PendingSpectreLoadouts")]
    pub pending_spectre_loadouts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "SpectreLoadouts")]
    pub spectre_loadouts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "SentinelWeapons")]
    pub sentinel_weapons: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Sentinels")]
    pub sentinels: Option<Vec<serde_json::Value>>,
    #[serde(rename = "EmailItems")]
    pub email_items: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CompletedSyndicates")]
    pub completed_syndicates: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Wishlist")]
    pub wishlist: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CompletedSorties")]
    pub completed_sorties: Option<Vec<serde_json::Value>>,
    #[serde(rename = "LastSortieReward")]
    pub last_sortie_reward: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Drones")]
    pub drones: Option<Vec<serde_json::Value>>,
    #[serde(rename = "StepSequencers")]
    pub step_sequencers: Option<Vec<serde_json::Value>>,
    #[serde(rename = "KubrowPets")]
    pub kubrow_pets: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ShipDecorations")]
    pub ship_decorations: Option<Vec<serde_json::Value>>,
    #[serde(rename = "OperatorAmpBin")]
    pub operator_amp_bin: Option<EquipmentBin>,
    #[serde(rename = "DailyAffiliationCetus")]
    pub daily_affiliation_cetus: Option<i32>,
    #[serde(rename = "DailyAffiliationQuills")]
    pub daily_affiliation_quills: Option<i32>,
    #[serde(rename = "DiscoveredMarkers")]
    pub discovered_markers: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CompletedJobs")]
    pub completed_jobs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "FocusUpgrades")]
    pub focus_upgrades: Option<Vec<serde_json::Value>>,
    #[serde(rename = "OperatorAmps")]
    pub operator_amps: Option<Vec<serde_json::Value>>,
    #[serde(rename = "HasContributedToDojo")]
    pub has_contributed_to_dojo: Option<bool>,
    #[serde(rename = "KubrowPetPrints")]
    pub kubrow_pet_prints: Option<Vec<serde_json::Value>>,
    #[serde(rename = "PersonalGoalProgress")]
    pub personal_goal_progress: Option<Vec<serde_json::Value>>,
    #[serde(rename = "DailyAffiliationSolaris")]
    pub daily_affiliation_solaris: Option<i32>,
    #[serde(rename = "SpecialItems")]
    pub special_items: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ChallengeInstanceStates")]
    pub challenge_instance_states: Option<Vec<serde_json::Value>>,
    #[serde(rename = "LoginMilestoneRewards")]
    pub login_milestone_rewards: Option<Vec<serde_json::Value>>,
    #[serde(rename = "OperatorLoadOuts")]
    pub operator_loadouts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "DailyAffiliationVentkids")]
    pub daily_affiliation_ventkids: Option<i32>,
    #[serde(rename = "DailyAffiliationVox")]
    pub daily_affiliation_vox: Option<i32>,
    #[serde(rename = "RecentVendorPurchases")]
    pub recent_vendor_purchases: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Hoverboards")]
    pub hoverboards: Option<Vec<serde_json::Value>>,
    #[serde(rename = "NodeIntrosCompleted")]
    pub node_intros_completed: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CompletedJobChains")]
    pub completed_job_chains: Option<Vec<serde_json::Value>>,
    #[serde(rename = "SeasonChallengeHistory")]
    pub season_challenge_history: Option<Vec<SeasonChallenge>>,
    #[serde(rename = "MoaPets")]
    pub moa_pets: Option<Vec<serde_json::Value>>,
    #[serde(rename = "InvasionChainProgress")]
    pub invasion_chain_progress: Option<Vec<serde_json::Value>>,
    #[serde(rename = "DataKnives")]
    pub data_knives: Option<Vec<DataKnife>>,
    #[serde(rename = "NemesisHistory")]
    pub nemesis_history: Option<Vec<serde_json::Value>>,
    #[serde(rename = "PersonalTechProjects")]
    pub personal_tech_projects: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewShips")]
    pub crew_ships: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewShipSalvageBin")]
    pub crew_ship_salvage_bin: Option<EquipmentBin>,
    #[serde(rename = "CrewShipAmmo")]
    pub crew_ship_ammo: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewShipSalvagedWeaponSkins")]
    pub crew_ship_salvaged_weapon_skins: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewShipWeapons")]
    pub crew_ship_weapons: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewShipSalvagedWeapons")]
    pub crew_ship_salvaged_weapons: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewShipWeaponSkins")]
    pub crew_ship_weapon_skins: Option<Vec<serde_json::Value>>,
    #[serde(rename = "PlayedParkourTutorial")]
    pub played_parkour_tutorial: Option<bool>,
    #[serde(rename = "SubscribedToEmailsPersonalized")]
    pub subscribed_to_emails_personalized: Option<i32>,
    #[serde(rename = "MechBin")]
    pub mech_bin: Option<EquipmentBin>,
    #[serde(rename = "DailyAffiliationEntrati")]
    pub daily_affiliation_entrati: Option<i32>,
    #[serde(rename = "DailyAffiliationNecraloid")]
    pub daily_affiliation_necraloid: Option<i32>,
    #[serde(rename = "MechSuits")]
    pub mech_suits: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewMemberBin")]
    pub crew_member_bin: Option<EquipmentBin>,
    #[serde(rename = "CrewShipHarnesses")]
    pub crew_ship_harnesses: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewShipRawSalvage")]
    pub crew_ship_raw_salvage: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CrewMembers")]
    pub crew_members: Option<Vec<serde_json::Value>>,
    #[serde(rename = "AdultOperatorLoadOuts")]
    pub adult_operator_load_outs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "DailyAffiliationZariman")]
    pub daily_affiliation_zariman: Option<i32>,
    #[serde(rename = "NemesisAbandonedRewards")]
    pub nemesis_abandoned_rewards: Option<Vec<serde_json::Value>>,
    #[serde(rename = "DailyAffiliationKahl")]
    pub daily_affiliation_kahl: Option<i32>,
    #[serde(rename = "LastInventorySync")]
    pub last_inventory_sync: Option<ObjectId>,
    #[serde(rename = "NextRefill")]
    pub next_refill: Option<Date>,
    #[serde(rename = "ActiveLandscapeTraps")]
    pub active_landscape_traps: Option<Vec<serde_json::Value>>,
    #[serde(rename = "EvolutionProgress")]
    pub evolution_progress: Option<Vec<serde_json::Value>>,
    #[serde(rename = "RepVotes")]
    pub rep_votes: Option<Vec<serde_json::Value>>,
    #[serde(rename = "LeagueTickets")]
    pub league_tickets: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Quests")]
    pub quests: Option<Vec<serde_json::Value>>,
    #[serde(rename = "Robotics")]
    pub robotics: Option<Vec<serde_json::Value>>,
    #[serde(rename = "UsedDailyDeals")]
    pub used_daily_deals: Option<Vec<serde_json::Value>>,
    #[serde(rename = "LibraryPersonalProgress")]
    pub library_personal_progress: Option<Vec<serde_json::Value>>,
    #[serde(rename = "CollectibleSeries")]
    pub collectible_series: Option<Vec<serde_json::Value>>,
    #[serde(rename = "LibraryAvailableDailyTaskInfo")]
    pub library_available_daily_task_info: Option<LibraryDailyTaskInfo>,
    #[serde(rename = "HasResetAccount")]
    pub has_reset_account: Option<bool>,
    #[serde(rename = "PendingCoupon")]
    pub pending_coupon: Option<PendingCoupon>,
    #[serde(rename = "__v")]
    pub version: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentBin {
    #[serde(rename = "Slots")]
    pub slots: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeProgress {
    #[serde(rename = "Progress")]
    pub progress: i32,
    #[serde(rename = "Name")]
    pub name: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equipment {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "ItemType")]
    pub item_type: serde_json::Value,
    #[serde(rename = "Configs")]
    pub configs: Vec<Config>,
    #[serde(rename = "XP")]
    pub xp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "ItemType")]
    pub item_type: serde_json::Value,
    #[serde(rename = "Configs")]
    pub configs: Vec<Config>,
    #[serde(rename = "XP")]
    pub xp: i32,
    #[serde(rename = "ModularParts")]
    pub polarity: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestKey {
    #[serde(rename = "ItemType")]
    pub item_type: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlavourItem {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "ItemType")]
    pub item_type: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "Skins")]
    pub skins: Vec<serde_json::Value>,
    #[serde(rename = "Upgrades")]
    pub upgrades: Vec<serde_json::Value>,
    #[serde(rename = "PvpUpgrades")]
    pub pvp_upgrades: Vec<serde_json::Value>,
    #[serde(rename = "Songs")]
    pub songs: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XPInfo {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "XP")]
    pub xp: i32,
    #[serde(rename = "Affinity")]
    pub affinity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "ItemCount")]
    pub item_count: i32,
    #[serde(rename = "ItemType")]
    pub item_type: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumable {
    #[serde(rename = "ItemCount")]
    pub item_count: i32,
    #[serde(rename = "ItemType")]
    pub item_type: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataKnife {
    #[serde(rename = "ItemId")]
    pub id: ObjectId,
    #[serde(rename = "ItemType")]
    pub item_type: serde_json::Value,
    #[serde(rename = "XP")]
    pub xp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonChallenge {
    pub id: serde_json::Value,
    pub challenge: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryDailyTaskInfo {
    #[serde(rename = "EnemyTypes")]
    pub enemy_types: Vec<serde_json::Value>,
    #[serde(rename = "EnemyLocTag")]
    pub enemy_loc_tag: serde_json::Value,
    #[serde(rename = "EnemyIcon")]
    pub enemy_icon: serde_json::Value,
    #[serde(rename = "ScansRequired")]
    pub scans_required: i32,
    #[serde(rename = "RewardStoreItem")]
    pub reward_store_item: serde_json::Value,
    #[serde(rename = "RewardQuantity")]
    pub reward_quantity: i32,
    #[serde(rename = "RewardStanding")]
    pub reward_standing: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingCoupon {
    #[serde(rename = "Expiry")]
    pub date: Date,
    #[serde(rename = "Discount")]
    pub discount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Date {
    #[serde(rename = "$date")]
    pub date: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,
    #[serde(rename = "ItemType")]
    pub item_type: serde_json::Value,
}
