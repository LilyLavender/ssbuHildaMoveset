use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use std::f32::consts::E;
use smash::lua2cpp::L2CFighterCommon;

#[acmd_script( agent = "ike", script = "game_speciallwhit", category = ACMD_GAME, low_priority )]
pub fn ike_counter(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
    frame(Frame=4)
if(is_excute){
ReflectModule::set_attack_mul(0.1);
}
frame(Frame=5)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=90, KBG=0, FKB=10, BKB=10, Size=9.0, X=0.0, Y=8.0, Z=18.0, X2=0.0, Y2=8.0, Z2=5.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
AttackModule::set_add_reaction_frame(ID=0, Frames=63.0, Unk=false);
}
frame(Frame=7)
FT_MOTION_RATE(FSM=1.3)
if(is_excute){
AttackModule::clear_all();
ReflectModule::set_attack_mul(1.0);
}
frame(Frame=60)
if(is_excute){
ReflectModule::set_attack_mul(1.4/(1.0 + E.powf((-0.05 * (DamageModule::damage(fighter.module_accessor, 0))) + 3.1)) + 0.1);
}
frame(Frame=61)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=1.0, Angle=361, KBG=83, FKB=0, BKB=60, Size=5.7, X=0.0, Y=14.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
frame(Frame=67)
if(is_excute){
AttackModule::clear_all()
ReflectModule::set_attack_mul(1.0);
}
}
    });
}

#[acmd_script( agent = "ike", script = "game_specialairlwhit", category = ACMD_GAME, low_priority )]
pub fn ike_counter_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
    frame(Frame=4)
if(is_excute){
ReflectModule::set_attack_mul(0.1);
}
frame(Frame=5)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=90, KBG=0, FKB=110, BKB=110, Size=9.0, X=0.0, Y=8.0, Z=18.0, X2=0.0, Y2=8.0, Z2=5.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
AttackModule::set_add_reaction_frame(ID=0, Frames=20.0, Unk=false);
}
frame(Frame=7)
FT_MOTION_RATE(FSM=1.3)
if(is_excute){
AttackModule::clear_all();
ReflectModule::set_attack_mul(1.0);
}
frame(Frame=60)
if(is_excute){
ReflectModule::set_attack_mul(1.4/(1.0 + E.powf((-0.052 * (DamageModule::damage(fighter.module_accessor, 0))) + 3.1)) + 0.1);
}
frame(Frame=61)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=1.0, Angle=361, KBG=83, FKB=0, BKB=60, Size=5.7, X=0.0, Y=14.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=25, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
}
frame(Frame=67)
if(is_excute){
AttackModule::clear_all()
ReflectModule::set_attack_mul(1.0);
}
    });
}

pub fn install() {
    install_acmd_scripts!(
        ike_counter,
		ike_counter_air,
    );
}
