pub fn efficiency(p_success: f32, cost: f32) -> f32 {
    if cost == 0.0 { 0.0 } else { p_success / cost }
}
