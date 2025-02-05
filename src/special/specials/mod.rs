use super::*;

pub mod ike;
pub mod shortaxe;

pub fn install() {
    ike::install();
    shortaxe::install();
}