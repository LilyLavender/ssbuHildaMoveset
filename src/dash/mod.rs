use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script( agent = "ike", script = "game_dash", category = ACMD_GAME, low_priority )]
pub fn ike_run(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
    frame(Frame=1)
ReflectModule::set_attack_mul(1.0);
frame(Frame=11)
if(is_excute){
WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN)
}

    });
}

pub fn install() {
    install_acmd_scripts!(
        ike_run,
    );
}
