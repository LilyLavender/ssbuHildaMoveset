use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script( agent = "ike", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
pub fn ike_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
        frame(Frame=3)
if(is_excute){
WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
}
frame(Frame=5)
FT_MOTION_RATE(FSM=1.3)
frame(Frame=15)
FT_MOTION_RATE(FSM=1)
frame(Frame=16)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=21.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=3.7, X=0.0, Y=3.5, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=21.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=4.8, X=0.0, Y=13.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
}
wait(Frames=3)
if(is_excute){
AttackModule::clear_all()
}
frame(Frame=26)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=21.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=3.7, X=0.0, Y=3.5, Z=-11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=21.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=4.8, X=0.0, Y=13.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
}
wait(Frames=3)
if(is_excute){
AttackModule::clear_all()
}
    });
}

pub fn install() {
    install_acmd_scripts!(
        ike_dsmash,
    );
}