// Config Data
#[allow(dead_code)]
pub mod config {
    pub const CONFIG_PATH: &str = "./config/";
    pub const CONFIG_JSON_PATH: &str = "./config/config.json";
    pub const MAILLIST_PATH: &str = "./config/mails.json";
    pub const RLV2_CONFIG_PATH: &str = "./config/rlv2Config.json";
    pub const ASSIST_JSON_PATH: &str = "./config/assist.json";
    pub const SQUADS_PATH: &str = "./config/squads.json";
    pub const SYNC_DATA_TEMPLATE_PATH: &str = "./config/syncData.json";
}

// User Data
#[allow(dead_code)]
pub mod user {
    pub const USER_JSON_PATH: &str = "./data/user/user.json";
    pub const BATTLE_REPLAY_JSON_PATH: &str = "./data/user/battleReplays.json";
    pub const RLV2_JSON_PATH: &str = "./data/user/rlv2.json";
    pub const RLV2_TEMPBUFF_JSON_PATH: &str = "./data/user/rlv2TempBuffs.json";
    pub const RLV2_USER_SETTINGS_PATH: &str = "data/user/rlv2UserSettings.json";
    pub const CRISIS_JSON_BASE_PATH: &str = "./data/crisis/";
    pub const CRISIS_V2_JSON_BASE_PATH: &str = "./data/crisisV2/";
    pub const RUNE_JSON_PATH: &str = "./data/user/rune.json";
    pub const BUILDING_JSON_PATH: &str = "./data/user/building.json";
    pub const GACHA_TEMPLATE_JSON_PATH: &str = "./data/gacha/gacha.json";
}

// RLV2 Options
#[allow(dead_code)]
pub mod rlv2 {
    pub const RLV2_CHOICEBUFFS: &str = "./data/rlv2/choiceBuffs.json";
    pub const RLV2_RECRUITGROUPS: &str = "./data/rlv2/recruitGroups.json";
    pub const RLV2_NODESINFO: &str = "./data/rlv2/nodesInfo.json";
}

// TOWER Data
#[allow(dead_code)]
pub mod tower {
    pub const TOWERDATA_PATH: &str = "./data/tower/towerData.json";
}

#[allow(dead_code)]
pub mod templates {
    pub const SANDBOX_TEMPLATE: &str = "./data/sandbox/sandbox.json";
}

#[allow(dead_code)]
pub mod sandbox {
    pub const SANDBOX_JSON_PATH: &str = "./data/user/sandbox.json";
    pub const SANDBOX_TEMP_JSON_PATH: &str = "./data/user/sandboxTemp.json";
}

#[allow(dead_code)]
pub mod url {
    use constcat::concat;
    // BASE_URL
    pub const BASE_URL: &str = "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata";

    // TABLE Urls
    pub const ACTIVITY_TABLE_URL: &str = concat!(BASE_URL, "/excel/activity_table.json");
    pub const CHARM_TABLE_URL: &str = concat!(BASE_URL, "/excel/charm_table.json");
    pub const SKIN_TABLE_URL: &str = concat!(BASE_URL, "/excel/skin_table.json");
    pub const CHARACTER_TABLE_URL: &str = concat!(BASE_URL, "/excel/character_table.json");
    pub const BATTLEEQUIP_TABLE_URL: &str = concat!(BASE_URL, "/excel/battle_equip_table.json");
    pub const EQUIP_TABLE_URL: &str = concat!(BASE_URL, "/excel/uniequip_table.json");
    pub const STORY_TABLE_URL: &str = concat!(BASE_URL, "/excel/story_table.json");
    pub const STAGE_TABLE_URL: &str = concat!(BASE_URL, "/excel/stage_table.json");
    pub const RL_TABLE_URL: &str = concat!(BASE_URL, "/excel/roguelike_topic_table.json");
    pub const DM_TABLE_URL: &str = concat!(BASE_URL, "/excel/display_meta_table.json");
    pub const RETRO_TABLE_URL: &str = concat!(BASE_URL, "/excel/retro_table.json");
    pub const HANDBOOK_INFO_TABLE_URL: &str = concat!(BASE_URL, "/excel/handbook_info_table.json");
    pub const TOWER_TABLE_URL: &str = concat!(BASE_URL, "/excel/climb_tower_table.json");
    pub const BUILDING_TABLE_URL: &str = concat!(BASE_URL, "/excel/building_data.json");
    pub const SANDBOX_TABLE_URL: &str = concat!(BASE_URL, "/excel/sandbox_perm_table.json");
    pub const STORY_REVIEW_TABLE_URL: &str = concat!(BASE_URL, "/excel/story_review_table.json");
    pub const STORY_REVIEW_META_TABLE_URL: &str = concat!(BASE_URL, "/excel/story_review_meta_table.json");
    pub const ENEMY_HANDBOOK_TABLE_URL: &str = concat!(BASE_URL, "/excel/enemy_handbook_table.json");
    pub const MEDAL_TABLE_URL: &str = concat!(BASE_URL, "/excel/medal_table.json");
    pub const CHARWORD_TABLE_URL: &str = concat!(BASE_URL, "/excel/charword_table.json");
    pub const GACHA_TABLE_URL: &str = concat!(BASE_URL, "/excel/gacha_table.json");
    pub const GAMEDATA_CONST_URL: &str = concat!(BASE_URL, "/excel/gamedata_const.json");
}

pub mod ascii {
    pub const TITLE: &str = r#"
█████ █████ ████  ████   ███    ████   ████
  █   █     █   █ █   █ █   █   █   █ █
  █   █████ ████  ████  █████   ████   ███
  █   █     █   █ █   █ █   █   █         █
  █   █████ █   █ █   █ █   █   █     ████
"#;
}
