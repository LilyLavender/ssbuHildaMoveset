use super::*;

unsafe extern "C" fn ike_frame(fighter: &mut L2CFighterCommon) {
    if is_hilda(fighter.module_accessor) {
        let shortaxe_cooldown = WorkModule::get_int(fighter.module_accessor, FIGHTER_IKE_INSTANCE_WORK_ID_INT_SHORTAXE_COOLDOWN);
        if shortaxe_cooldown > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_IKE_INSTANCE_WORK_ID_INT_SHORTAXE_COOLDOWN);
        }
    }
}

pub fn install() {
    Agent::new("ike")
        .on_line(Main, ike_frame)
        .install();
}
