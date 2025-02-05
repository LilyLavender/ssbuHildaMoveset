use super::*;

// STATUS
unsafe extern "C" fn ike_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE),  
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG.into(), 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT.into(), 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 
        FIGHTER_STATUS_ATTR_START_TURN.into(), 
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S.into(), 
        0
    );
    0.into()
}

unsafe extern "C" fn ike_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
    let shortaxe_cooldown = WorkModule::get_int(fighter.module_accessor, FIGHTER_IKE_INSTANCE_WORK_ID_INT_SHORTAXE_COOLDOWN);
    if shortaxe_cooldown == 0 {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SUMMON_SHORTAXE);
        WorkModule::set_int(fighter.module_accessor, 180, FIGHTER_IKE_INSTANCE_WORK_ID_INT_SHORTAXE_COOLDOWN);
    }
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_specials_main_loop as *const () as _))
}

unsafe extern "C" fn ike_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    let is_end = MotionModule::is_end(fighter.module_accessor);
    if is_end {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    
    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    if is_changing || fighter.global_table[0x17].get_i32() != fighter.global_table[0x16].get_i32() {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            let status_frame = fighter.global_table[0xe].get_f32();
            if status_frame == 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            let status_frame = fighter.global_table[0xe].get_f32();
            if status_frame == 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn ike_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SUMMON_SHORTAXE);
    return 0.into();
}

// ACMD
unsafe extern "C" fn ike_game_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SUMMON_SHORTAXE) {
            ArticleModule::generate_article(agent.module_accessor, FIGHTER_IKE_GENERATE_ARTICLE_SHORTAXE, false, -1);
        }
    }
}

unsafe extern "C" fn ike_effect_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SUMMON_SHORTAXE) {
            macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), 12, 10, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub fn install() {
    Agent::new("ike")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, ike_specials_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, ike_specials_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, ike_specials_end)
        .game_acmd("game_specials_hilda", ike_game_specials, Default)
        .game_acmd("game_specialairs_hilda", ike_game_specials, Default)
        .effect_acmd("effect_specials_hilda", ike_effect_specials, Default)
        .effect_acmd("effect_specialairs_hilda", ike_effect_specials, Default)
        .install();
}

