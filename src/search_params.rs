use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchParams {
    pub null_move_pruning_depth: u8,
    pub null_move_pruning_reduction: u8,

    pub internal_iterative_deepening_depth: u8,
    pub internal_iterative_deepening_reduction: u8,

    pub futility_pruning_depth: u8,
    pub futility_pruning_margin: i32,

    pub razoring_depth: u8,
    pub razoring_margin: i32,

    pub singular_extension_margin: i32,

    pub aspiration_window_size: i32,

    pub delta_margin: i32,

    pub butterfly_pruning_count: i32,

    pub sorting_capture_base_val: i32,

    pub sorting_mvv_lva_extra_base_val: i32,

    pub sorting_good_history_base_val: i32,
    pub sorting_normal_history_base_val: i32,
    pub sorting_weak_history_base_val: i32,

    pub sorting_checker_bonus: i32,

    pub sorting_counter_move_val: i32,
    pub sorting_killer_primary_val: i32,
    pub sorting_killer_secondary_val: i32,

    pub late_move_reductions_depth: u8,
    pub late_move_reductions_move_count: usize,
}

impl SearchParams {
    pub const fn default() -> Self {
        SearchParams {
            null_move_pruning_depth: 6,
            null_move_pruning_reduction: 2,

            internal_iterative_deepening_depth: 7,
            internal_iterative_deepening_reduction: 2,

            futility_pruning_depth: 7,
            futility_pruning_margin: 85,

            razoring_depth: 2,
            razoring_margin: 524,

            singular_extension_margin: 132,

            aspiration_window_size: 50,

            delta_margin: 240,

            butterfly_pruning_count: 16,

            sorting_capture_base_val: 1_000_000_000,
            sorting_mvv_lva_extra_base_val: 500,

            sorting_good_history_base_val: 100_000_000,
            sorting_normal_history_base_val: 10_000_000,
            sorting_weak_history_base_val: 1_000_000,

            sorting_checker_bonus: 50,

            sorting_counter_move_val: -20,
            sorting_killer_primary_val: -30,
            sorting_killer_secondary_val: -40,

            late_move_reductions_depth: 2,
            late_move_reductions_move_count: 1,
        }
    }

    pub fn from_config(config_map: &HashMap<String, String>) -> Self {
        SearchParams {
            null_move_pruning_depth: config_map.get("null_move_pruning_depth").unwrap().parse::<u8>().unwrap(),
            null_move_pruning_reduction: config_map.get("null_move_pruning_reduction").unwrap().parse::<u8>().unwrap(),

            internal_iterative_deepening_depth: config_map.get("internal_iterative_deepening_depth").unwrap().parse::<u8>().unwrap(),
            internal_iterative_deepening_reduction: config_map.get("internal_iterative_deepening_reduction").unwrap().parse::<u8>().unwrap(),

            futility_pruning_depth: config_map.get("futility_pruning_depth").unwrap().parse::<u8>().unwrap(),
            futility_pruning_margin: config_map.get("futility_pruning_margin").unwrap().parse::<i32>().unwrap(),

            razoring_depth: config_map.get("razoring_depth").unwrap().parse::<u8>().unwrap(),
            razoring_margin: config_map.get("razoring_margin").unwrap().parse::<i32>().unwrap(),

            singular_extension_margin: config_map.get("singular_extension_margin").unwrap().parse::<i32>().unwrap(),

            aspiration_window_size: config_map.get("aspiration_window_size").unwrap().parse::<i32>().unwrap(),

            delta_margin: config_map.get("delta_margin").unwrap().parse::<i32>().unwrap(),

            butterfly_pruning_count: config_map.get("butterfly_pruning_count").unwrap().parse::<i32>().unwrap(),

            sorting_capture_base_val: config_map.get("sorting_capture_base_val").unwrap().parse::<i32>().unwrap(),

            sorting_mvv_lva_extra_base_val: config_map.get("sorting_mvv_lva_extra_base_val").unwrap().parse::<i32>().unwrap(),

            sorting_good_history_base_val: config_map.get("sorting_good_history_base_val").unwrap().parse::<i32>().unwrap(),
            sorting_normal_history_base_val: config_map.get("sorting_normal_history_base_val").unwrap().parse::<i32>().unwrap(),
            sorting_weak_history_base_val: config_map.get("sorting_weak_history_base_val").unwrap().parse::<i32>().unwrap(),

            sorting_checker_bonus: config_map.get("sorting_checker_bonus").unwrap().parse::<i32>().unwrap(),

            sorting_counter_move_val: config_map.get("sorting_counter_move_val").unwrap().parse::<i32>().unwrap(),
            sorting_killer_primary_val: config_map.get("sorting_killer_primary_val").unwrap().parse::<i32>().unwrap(),
            sorting_killer_secondary_val: config_map.get("sorting_killer_secondary_val").unwrap().parse::<i32>().unwrap(),

            late_move_reductions_depth: config_map.get("late_move_reductions_depth").unwrap().parse::<u8>().unwrap(),
            late_move_reductions_move_count: config_map.get("late_move_reductions_move_count").unwrap().parse::<usize>().unwrap(),
        }
    }
}
