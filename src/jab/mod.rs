use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script( agent = "ike", script = "game_attack11", category = ACMD_GAME, low_priority )]
pub fn ike_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
    frame(Frame=5)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=4.5, Angle=80, KBG=30, FKB=0, BKB=55, Size=5.8, X=6.0, Y=13.0, Z=1.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=3.2, Angle=80, KBG=30, FKB=0, BKB=55, Size=3.8, X=0.0, Y=0.0, Z=1.7, X2=9.0, Y2=10.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
}
frame(Frame=11)
if(is_excute){
AttackModule::clear_all()
}
frame(Frame=16)
if(is_excute){
WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
}
frame(Frame=19)
if(is_excute){
WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
}
    });
}

#[acmd_script( agent = "ike", script = "game_attack12", category = ACMD_GAME, low_priority )]
pub fn ike_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
    frame(Frame=6)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=3.5, X=0.0, Y=9.0, Z=10.0, X2=0.0, Y2=14.0, Z2=9.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=4.0, X=0.0, Y=9.0, Z=16.0, X2=0.0, Y2=14.0, Z2=9.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=4.0, X=0.0, Y=9.0, Z=16.0, X2=0.0, Y2=14.0, Z2=9.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=2.5, X=9.0, Y=0.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=2.5, X=3.5, Y=0.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
AttackModule::set_add_reaction_frame(ID=0, Frames=7.0, Unk=false)
AttackModule::set_add_reaction_frame(ID=1, Frames=7.0, Unk=false)
}
wait(Frames=4)
if(is_excute){
AttackModule::clear_all()
WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
}
frame(Frame=14)
if(is_excute){
WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
}
	});
}

#[acmd_script( agent = "ike", script = "game_attack13", category = ACMD_GAME, low_priority )]
pub fn ike_jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
    frame(Frame=5)
if(is_excute){
ATTACK(ID=0, Part=0, Bone=hash40("sword"), Damage=4.0, Angle=45, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=12.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
ATTACK(ID=1, Part=0, Bone=hash40("sword"), Damage=4.0, Angle=45, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=9.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
ATTACK(ID=2, Part=0, Bone=hash40("sword"), Damage=4.0, Angle=45, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=5.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
ATTACK(ID=3, Part=0, Bone=hash40("sword"), Damage=4.0, Angle=45, KBG=100, FKB=0, BKB=60, Size=3.3, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_IKE, Type=ATTACK_REGION_SWORD)
}
wait(Frames=4)
if(is_excute){
AttackModule::clear_all()
}

    });
}

pub fn install() {
    install_acmd_scripts!(
        ike_jab1,
		ike_jab2,
		ike_jab3,
    );
}
