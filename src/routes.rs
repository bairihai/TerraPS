use crate::{
    core::{
        asset, general_v1_server_time, prod,
        user::{self, app, business_card},
    },
    game::{
        account, background, building, campaignv2,
        char_manager::{char, char_build, charm},
        crisis_manager::crisis_v2,
        deep_sea, online, pay,
        quest_manager::{april_fools, bossrush, quest, story_review},
        shop, social, story,
    },
    utils::json::JSON,
};
use axum::{
    extract::Request,
    routing::{get, post},
    Json, Router,
};

use serde_json::json;
use tower_http::trace::{DefaultMakeSpan as DefMakeSpan, DefaultOnResponse, TraceLayer as Tracer};
use tracing::{debug, Level, Span};

pub fn routes() -> Router {
    let trace_layer = Tracer::new_for_http()
        .make_span_with(DefMakeSpan::default())
        .on_request(|req: &Request<_>, _span: &Span| {
            debug!("Received request: {:?}", req.uri());
        })
        .on_response(DefaultOnResponse::default().level(Level::DEBUG))
        .on_failure(())
        .on_eos(());

    Router::new()
        .nest("/app", app_routes())
        .nest("/account", account_routes())
        .nest("/activity", activity_routes())
        .nest("/act25side", act25side_routes())
        .nest("/aprilFool", april_fools_routes())
        .nest("/building", building_routes())
        .nest("/businessCard", business_card_routes())
        .nest("/campaignV2", campaignv2_routes())
        .nest("/char", char_routes())
        .nest("/charBuild", char_build_routes())
        .nest("/config/prod", config_routes())
        .nest("/crisisV2", crisis_v2_routes())
        .nest("/deepSea", deep_sea_routes())
        .nest("/online", online_routes())
        .nest("/quest", quest_routes())
        .nest("/retro", retro_routes())
        .nest("/shop", shop_routes())
        .nest("/social", social_routes())
        .nest("/story", story_routes())
        .nest("/storyreview", story_review_routes())
        .nest("/u8", u8_routes())
        .nest("/user", user_routes())
        .nest("/debug", debug_routes())
        .merge(misc_routes())
        .fallback(fallback)
        .layer(trace_layer)
}

fn app_routes() -> Router {
    Router::new()
        .route("/v1/config", get(app::app_v1_config))
        .route("/getSettings", post(app::app_get_settings))
        .route("/getCode", post(app::app_get_code))
}

fn account_routes() -> Router {
    Router::new()
        .route("/login", post(account::account_login))
        .route("/syncStatus", post(account::account_sync_status))
        .route("/syncData", post(account::account_sync_data))
        .route("/yostar_auth_request", post(account::account_yostar_auth_req))
        .route("/yostar_auth_submit", post(account::account_yostar_auth_submit))
}

fn activity_routes() -> Router {
    Router::new()
        .route("/bossRush/relicSelect", post(bossrush::relic_select))
        .route("/bossRush/battleStart", post(quest::quest_battle_start))
        .route("/act24side/battleStart", post(quest::quest_battle_start))
        .route("/act24side/battleFinish", post(quest::quest_battle_finish))
        .route("/act24side/setTool", post(quest::set_tool))
}

fn act25side_routes() -> Router {
    Router::new()
        .route("/battleStart", post(quest::quest_battle_start))
        .route("/battleFinish", post(quest::quest_battle_finish))
}

fn april_fools_routes() -> Router {
    Router::new()
        .route("/act5fun/battleStart", post(quest::quest_battle_start))
        .route("/act5fun/battleFinish", post(april_fools::act5_fun_battle_finish))
        .route("/act4fun/battleStart", post(quest::quest_battle_start))
        .route("/act4fun/battleFinish", post(april_fools::act4_fun_battle_finish))
        .route("/act4fun/liveSettle", post(april_fools::act4_fun_live_settle))
        .route("/act3fun/battleStart", post(quest::quest_battle_start))
        .route("/act3fun/battleFinish", post(quest::quest_battle_finish))
}

fn building_routes() -> Router {
    Router::new()
        .route("/sync", post(building::building_sync))
        .route("/getRecentVisitors", post(building::building_get_recent_visitors))
        .route("/getInfoShareVisitorsNum", post(building::building_get_info_share_visitor_num))
        .route("/getAssistReport", post(building::building_get_assist_report))
        .route("/changeDiySolution", post(building::building_change_diy_solution))
        .route("/assignChar", post(building::building_assign_char))
        .route("/setBuildingAssist", post(building::building_set_building_assist))
}

fn business_card_routes() -> Router {
    Router::new()
        .route("/changeNameCardComponent", post(business_card::change_name_component))
        .route("/changeNameCardSkin", post(business_card::change_card_skin))
}

fn campaignv2_routes() -> Router {
    Router::new()
        .route("/battleStart", post(campaignv2::campaignv2_battle_start))
        .route("/battleFinish", post(campaignv2::campaignv2_battle_finish))
        .route("/battleSweep", post(campaignv2::campaignv2_battle_sweep))
}

fn char_routes() -> Router {
    Router::new().route("/changeMarkStar", post(char::char_change_mark_star))
}

fn char_build_routes() -> Router {
    Router::new()
        .route("/addonStage/battleStart", post(quest::quest_battle_start))
        .route("/addonStage/battleFinish", post(quest::quest_battle_finish))
        .route("/addonStory/unlock", post(char_build::char_build_addon_story_unlock))
        .route("/batchSetCharVoiceLan", post(char_build::char_build_batch_set_char_voice_lan))
        .route("/setCharVoiceLan", post(char_build::char_build_set_char_voice_lan))
        .route("/setDefaultSkill", post(char_build::char_build_set_char_default_skill))
        .route("/changeCharSkin", post(char_build::char_build_change_char_skin))
        .route("/setEquipment", post(char_build::char_build_set_char_equipment))
        .route("/changeCharTemplate", post(char_build::char_build_change_char_template))
}

fn config_routes() -> Router {
    Router::new()
        .route("/official/Android/version", get(prod::prod_android_version))
        .route("/official/network_config", get(prod::prod_network_config))
        .route("/official/remote_config", get(prod::prod_remote_config))
        .route("/official/refresh_config", get(prod::prod_refresh_config))
        .route("/announce_meta/Android/announcement.meta.jsons", get(prod::prod_announcement))
        .route("/announce_meta/Android/preannouncement.meta.json", get(prod::prod_pre_announcement))
}

fn crisis_v2_routes() -> Router {
    Router::new()
        .route("/getInfo", post(crisis_v2::crisis_v2_get_info))
        .route("/battleStart", post(crisis_v2::crisis_v2_battle_start))
        .route("/battleFinish", post(crisis_v2::crisis_v2_battle_finish))
        .route("/getSnapshot", post(crisis_v2::crisis_v2_get_snapshot))
}

fn deep_sea_routes() -> Router {
    Router::new()
        .route("/branch", post(deep_sea::deep_sea_branch))
        .route("/event", post(deep_sea::deep_sea_event))
}

fn online_routes() -> Router {
    Router::new()
        .route("/v1/ping", post(online::online_v1_ping))
        .route("/v1/loginout", post(online::online_v1_login_out))
}

fn quest_routes() -> Router {
    Router::new()
        .route("/battleStart", post(quest::quest_battle_start))
        .route("/battleFinish", post(quest::quest_battle_finish))
        .route("/changeSquadName", post(quest::squad_change_name))
        .route("/squadFormation", post(quest::squad_set_formation))
        .route("/saveBattleReplay", post(quest::quest_save_battle_replay))
        .route("/getBattleReplay", post(quest::quest_get_battle_replay))
}

fn retro_routes() -> Router {
    Router::new()
        .route("/typeAct20side/competitionStart", post(quest::act_20_competition_start))
        .route("/typeAct20side/competitionFinish", post(quest::act_20_competition_finish))
}

fn shop_routes() -> Router {
    Router::new().route("/getSkinGoodList", post(shop::pay_get_unconfirmed_order_id_list))
}

fn social_routes() -> Router {
    Router::new()
        .route("/setAssistCharList", post(social::social_set_assist_char_list))
        .route("/setCardShowMedal", post(social::social_set_card_medal))
        .route("/getSortListInfo", post(social::social_get_sort_list_info))
        .route("/searchPlayer", post(social::social_search_player))
}

fn story_routes() -> Router {
    Router::new().route("/finishStory", post(story::story_finish_story))
}

fn story_review_routes() -> Router {
    Router::new()
        .route("/markStoryAcceKnown", post(story_review::mark_story_acce_known))
        .route("/readStory", post(story_review::read_story))
}

fn u8_routes() -> Router {
    Router::new()
        .route("/user/auth/v1/agreement_version", get(user::agreement_version))
        .route("/user/v1/getToken", post(user::user_v1_get_token))
        .route("/pay/getAllProductList", post(pay::pay_get_all_prod_list))
}

fn user_routes() -> Router {
    Router::new()
        .route("/auth", post(user::user_auth))
        .route("/auth/v1/token_by_phone_password", post(user::auth_v1_token_by_phone_password))
        .route("/agreement", get(user::user_agreement))
        .route("/checkIn", get(user::user_check_in))
        .route("/changeAvatar", post(user::user_change_avatar))
        .route("/changeSecretary", post(user::user_change_secretary))
        .route("/info/v1/need_cloud_auth", post(user::user_need_cloud_auth))
        .route("/info/v1/basic", get(user::info_v1_basic))
        .route("/login", post(user::user_login))
        .route("/oauth2/v1/grant", post(user::user_oauth2_v1_grant))
        .route("/oauth2/v2/grant", post(user::user_oauth2_v2_grant))
        .route("/yostar_createlogin", post(user::user_yostar_create_login))
}

fn misc_routes() -> Router {
    Router::new()
        .route("/general/v1/server_time", get(general_v1_server_time))
        .route("/pay/getUnconfirmedOrderIdList", post(pay::pay_get_unconfirmed_order_id_list))
        .route("/background/setBackground", post(background::background_set_bg))
        .route("/homeTheme/change", post(background::home_theme_change))
        .route("/charm/setSquad", post(charm::charm_set_squad))
        .route("/car/confirmBattleCar", post(quest::confirm_battle_car))
        .route("/templateTrap/setTrapSquad", post(quest::set_trap_squad))
        .route("/assetbundle/official/Android/assets/:hash/:name", get(asset::get_file))
}

fn debug_routes() -> Router {
    Router::new()
}

async fn fallback() -> JSON {
    Json(json!({
        "playerDataDelta": {
            "deleted": {},
            "modified": {}
        }
    }))
}
