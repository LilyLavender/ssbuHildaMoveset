use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
	std::f32::consts::E
};

#[acmd_script( agent = "ike", scripts = [ "game_specialsattack", "game_specialairsattack" ], category = ACMD_GAME, low_priority )]
unsafe fn ike_specialsattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(10.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "ike", scripts = [ "game_speciallwhit", "game_specialairlwhit" ], category = ACMD_GAME, low_priority )]
unsafe fn ike_counter(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		ReflectModule::set_attack_mul(fighter.module_accessor, 0.1);
	}
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 90, 0, 10, 10, 9.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 63.0, false);
	}
	frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 1.3);
	if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
		ReflectModule::set_attack_mul(fighter.module_accessor, 1.0);
	}
	frame(fighter.lua_state_agent, 60.0);
	if macros::is_excute(fighter) {
		ReflectModule::set_attack_mul(fighter.module_accessor, 1.4/(1.0 + E.powf((-0.05 * (DamageModule::damage(fighter.module_accessor, 0))) + 3.1)) + 0.1);
	}
	frame(fighter.lua_state_agent, 61.0);
	if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 83, 0, 60, 5.7, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 67.0);
	if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
		ReflectModule::set_attack_mul(fighter.module_accessor, 1.0);
	}
}

#[acmd_script( agent = "ike", scripts = ["effect_speciallwhit", "effect_specialairlwhit"], category = ACMD_EFFECT, low_priority )]
unsafe fn ike_counter_fx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
		macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 68.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 3);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		ike_counter,
		ike_counter_fx,
		ike_specialsattack,
    );
}