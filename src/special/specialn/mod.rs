use super::*;

// STATUS
unsafe extern "C" fn ike_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 
        FIGHTER_STATUS_ATTR_START_TURN.into(), 
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S.into(), 
        0
    );
    return 0.into();
}

unsafe extern "C" fn ike_specialn_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    return 0.into();
}

unsafe extern "C" fn ike_specialn_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if situation_kind == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0) * -1.0;
        let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
        fighter.push_lua_stack(&mut L2CValue::new_num(air_accel_y));
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
        fighter.push_lua_stack(&mut L2CValue::new_num(air_speed_y_stable));
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    }
    return 0.into();
}

unsafe extern "C" fn ike_specialn_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    return 0.into();
}

unsafe extern "C" fn ike_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn ike_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            let status_frame = fighter.global_table[0xe].get_f32();
            if status_frame == 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn ike_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    return 0.into();
}

// ACMD
unsafe extern "C" fn ike_game_specialn(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 2.6);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 11.1, 361, 220, 10, 10, 3.3, 0.0, 8.8, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 11.1, 361, 220, 10, 10, 3.8, 0.0, 3.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 10.3, 361, 220, 10, 10, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 6.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 6.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 6.0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ike_effect_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 7, Hash40::new("sword"), 0.0, 0.1, 0.0, Hash40::new("sword"), -0.0, 15.0, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 22, -4, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn ike_sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

unsafe extern "C" fn ike_expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

pub fn install() {
    Agent::new("ike")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_specialn_pre)
        .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_specialn_exit)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_specialn_exec)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_specialn_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_specialn_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_specialn_end)
        .game_acmd("game_specialn_hilda", ike_game_specialn, Default)
        .game_acmd("game_specialairn_hilda", ike_game_specialn, Default)
        .effect_acmd("effect_specialn_hilda", ike_effect_specialn, Default)
        .effect_acmd("effect_specialairn_hilda", ike_effect_specialn, Default)
        .sound_acmd("sound_specialn_hilda", ike_sound_specialn, Default)
        .sound_acmd("sound_specialairn_hilda", ike_sound_specialn, Default)
        .expression_acmd("expression_specialn_hilda", ike_expression_specialn, Default)
        .expression_acmd("expression_specialairn_hilda", ike_expression_specialn, Default)
        .install();
}
