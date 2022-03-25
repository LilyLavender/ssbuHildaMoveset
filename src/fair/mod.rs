use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script( agent = "ike", script = "game_attackairf", category = ACMD_GAME, low_priority )]
pub fn ike_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
    if(is_excute){
WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
}
FT_MOTION_RATE(FSM=0.66)
frame(Frame=12)
FT_MOTION_RATE(FSM=1)
frame(Frame=13)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("sword"), Damage=5.5, Angle=361, KBG=80, FKB=0, BKB=60, Size=5.5, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
ATTACK(ID=1, Part=0, Bone=hash40("sword"), Damage=4.0, Angle=361, KBG=70, FKB=0, BKB=35, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=8.0, Z2=0.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
}
wait(Frames=3)
if(is_excute){
AttackModule::clear_all()
}
frame(Frame=34)
if(is_excute){
WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
}
    });
}

pub fn install() {
    install_acmd_scripts!(
        ike_fair,
    );
}
