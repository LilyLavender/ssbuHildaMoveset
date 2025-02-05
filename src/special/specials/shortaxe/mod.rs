use super::*;

// STATUS
unsafe extern "C" fn ike_shortaxe_fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(),  
        smash::app::GroundCliffCheckKind(0), 
        false, 
        WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG.into(), 
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT.into(), 
        0
    );
    0.into()
}

unsafe extern "C" fn ike_shortaxe_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 120, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 120, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(ike_shortaxe_fly_main_loop as *const () as _))
}

unsafe extern "C" fn ike_shortaxe_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // Could've been a substatus
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 1 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    let facing = PostureModule::lr(weapon.module_accessor);
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);
    let speed_start_x: f32 = if facing == 1.0 { 2.4 } else { -2.4 };
    let deccel_x: f32 = if facing == 1.0 { -0.02 } else { 0.02 };
    let speed_max_y: f32 = 2.0;
    let accel_y: f32 = -0.04;
    let status_frame = weapon.global_table[0xe].get_f32();
    
    if speed_x.abs() > 0.0 {
        speed_x += deccel_x;
    }
    
    if status_frame <= 1.0 {
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let stick_y = ControlModule::get_stick_y(owner_boma);
        let speed_start_y: f32 = 1.2 + stick_y * 0.5;
        speed_y = speed_start_y;
        speed_x = speed_start_x;
    }
    if speed_y.abs() < speed_max_y {
        speed_y += accel_y;
    }
    
    // Set rot
    let rot_z = ((status_frame * 30.0) % 360.0) * facing * -1.0;
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{ x: 0.0, y: 90.0, z: rot_z }, 0);
    
    // Set speed
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);

    0.into()
}

unsafe extern "C" fn ike_shortaxe_fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// ACMD
unsafe extern "C" fn ike_shortaxe_game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("axe"), 15.0, 74, 75, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
    }
}

pub fn install() {
    Agent::new("ike_shortaxe")
        .status(Pre, 0x0, ike_shortaxe_fly_pre)
        .status(Main, 0x0, ike_shortaxe_fly_main)
        .status(End, 0x0, ike_shortaxe_fly_end)
        .game_acmd("game_fly", ike_shortaxe_game_fly, Default)
        .install();
}
