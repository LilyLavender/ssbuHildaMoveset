use super::*;

// STATUS
unsafe extern "C" fn ike_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
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

unsafe extern "C" fn ike_speciallw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
    return 0.into();
}

unsafe extern "C" fn ike_speciallw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
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

unsafe extern "C" fn ike_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
    return 0.into();
}

unsafe extern "C" fn ike_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn ike_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            let status_frame = fighter.global_table[0xe].get_f32();
            if status_frame == 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn ike_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_hilda(fighter.module_accessor) {
        return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
    return 0.into();
}

// ACMD
unsafe extern "C" fn ike_game_speciallw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.7);
	frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0); 
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 14.0, 25, 36, 0, 90, 2.9, 0.0, 8.8, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 14.0, 25, 36, 0, 90, 3.4, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 10.0, 25, 36, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 35.0, 55, 46, 0, 110, 3.3, 0.0, 8.8, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 35.0, 55, 46, 0, 110, 3.8, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 30.0, 55, 46, 0, 110, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 80, 120, 0, 64, 15.0, 0.0, 8.0, 24.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 9.0, 80, 120, 0, 64, 10.0, 0.0, 25.0, 24.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0); 
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
        AttackModule::clear(agent.module_accessor, 2, false);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ike_game_specialairlw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.7);
	frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 14.0, 25, 36, 0, 64, 2.9, 0.0, 8.8, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 14.0, 25, 36, 0, 64, 3.4, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 10.0, 25, 36, 0, 64, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 35.0, 55, 46, 0, 110, 3.3, 0.0, 8.8, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 35.0, 55, 46, 0, 110, 3.8, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 30.0, 55, 46, 0, 110, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 35.0, 55, 46, 0, 110, 3.3, 0.0, 8.8, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 35.0, 55, 46, 0, 110, 3.8, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 30.0, 55, 46, 0, 110, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ike_effect_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("brave_devil_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 7, Hash40::new("sword"), 0.0, 0.1, 0.0, Hash40::new("sword"), -0.0, 15.0, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0, 24, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("ike_volcano"), Hash40::new("top"), 0, 0, 24, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("ike_volcano_add3"), Hash40::new("top"), 0, 0, 24, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("ike_volcano_flash_g"), Hash40::new("top"), 0, 0, 24, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, 24, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.2, 0, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("brave_devil_ground"), Hash40::new("top"), 1.2, 0, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("brave_devil_sword"), false, true);
    }
}

unsafe extern "C" fn ike_effect_specialairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("brave_devil_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 7, Hash40::new("sword"), 0.0, 0.1, 0.0, Hash40::new("sword"), -0.0, 15.0, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("brave_devil_sword"), false, true);
    }
}

unsafe extern "C" fn ike_sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l21"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ike_attack07"));
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l21_02"));
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_brave_special_l21_02"));
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l21_04"));
    }
    frame(agent.lua_state_agent, 82.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l21_03"));
    }
}

unsafe extern "C" fn ike_expression_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y_MINUS));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 25, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn ike_expression_specialairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y_MINUS));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 25, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
}

pub fn install() {
    Agent::new("ike")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_speciallw_pre)
        .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_speciallw_exit)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_speciallw_exec)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_speciallw_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_speciallw_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_speciallw_end)
        .game_acmd("game_speciallw_hilda", ike_game_speciallw, Default)
        .game_acmd("game_specialairlw_hilda", ike_game_specialairlw, Default)
        .effect_acmd("effect_speciallw_hilda", ike_effect_speciallw, Default)
        .effect_acmd("effect_specialairlw_hilda", ike_effect_specialairlw, Default)
        .sound_acmd("sound_speciallw_hilda", ike_sound_speciallw, Default)
        .sound_acmd("sound_specialairlw_hilda", ike_sound_speciallw, Default)
        .expression_acmd("expression_speciallw_hilda", ike_expression_speciallw, Default)
        .expression_acmd("expression_specialairlw_hilda", ike_expression_specialairlw, Default)
        .install();
}
