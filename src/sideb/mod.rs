use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script( agent = "ike", script = "game_specialsattack", category = ACMD_GAME, low_priority )]
pub fn ike_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
    frame(Frame=1)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=70, KBG=120, FKB=0, BKB=36, Size=6.5, X=0.0, Y=8.4, Z=14.8, X2=0.0, Y2=8.4, Z2=10.7, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
WorkModule::on_flag(Flag=FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END)
}
frame(Frame=4)
if(is_excute){
AttackModule::clear_all()
}
    });
}

pub fn install() {
    install_acmd_scripts!(
        ike_sideb,
    );
}
