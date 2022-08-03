use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchParams {
    pub null_move_pruning_depth: u8,
    pub null_move_pruning_reduction: u8,

    pub multi_cut_pruning_depth: u8,
    pub multi_cut_pruning_reduction: u8,
    pub multi_cut_pruning_move_count: u8,
    pub multi_cut_pruning_cut_count: u8,

    pub internal_iterative_deepening_depth: u8,
    pub internal_iterative_deepening_reduction: u8,

    pub futility_pruning_depth: u8,
    pub futility_pruning_margin: i32,

    pub razoring_depth: u8,
    pub razoring_margin: i32,

    pub singular_extension_margin: i32,

    pub aspiration_window_size: i32,

    pub delta_margin: i32,

    pub sorting_capture_base_val: i32,
    pub sorting_history_base_val: i32,
    pub sorting_check_capture_bonus: i32,
    pub sorting_counter_move_val: i32,
    pub sorting_killer_primary_val: i32,
    pub sorting_killer_secondary_val: i32,
    pub sorting_checker_val: i32,
    pub sorting_passer_val: i32,

    pub late_move_reductions_depth: u8,
    pub late_move_reductions_move_count: usize,
}

impl SearchParams {
    pub const fn default() -> Self {
        SearchParams {
            null_move_pruning_depth: 6,
            null_move_pruning_reduction: 2,

            multi_cut_pruning_depth: 8,
            multi_cut_pruning_reduction: 6,
            multi_cut_pruning_move_count: 6,
            multi_cut_pruning_cut_count: 3,

            internal_iterative_deepening_depth: 7,
            internal_iterative_deepening_reduction: 2,

            futility_pruning_depth: 7,
            futility_pruning_margin: 85,

            razoring_depth: 2,
            razoring_margin: 524,

            singular_extension_margin: 132,

            aspiration_window_size: 50,

            delta_margin: 240,

            sorting_capture_base_val: 100000000,
            sorting_history_base_val: 100000,
            sorting_check_capture_bonus: 50,
            sorting_counter_move_val: 60,
            sorting_checker_val: 50,
            sorting_killer_primary_val: 40,
            sorting_passer_val: 30,
            sorting_killer_secondary_val: 20,

            late_move_reductions_depth: 2,
            late_move_reductions_move_count: 1,
        }
    }

    pub fn from_config(config_map: &HashMap<String, String>) -> Self {
        SearchParams {
            null_move_pruning_depth: config_map.get("null_move_pruning_depth").unwrap().parse::<u8>().unwrap(),
            null_move_pruning_reduction: config_map.get("null_move_pruning_reduction").unwrap().parse::<u8>().unwrap(),

            multi_cut_pruning_depth: config_map.get("multi_cut_pruning_depth").unwrap().parse::<u8>().unwrap(),
            multi_cut_pruning_reduction: config_map.get("multi_cut_pruning_reduction").unwrap().parse::<u8>().unwrap(),
            multi_cut_pruning_move_count: config_map.get("multi_cut_pruning_move_count").unwrap().parse::<u8>().unwrap(),
            multi_cut_pruning_cut_count: config_map.get("multi_cut_pruning_cut_count").unwrap().parse::<u8>().unwrap(),

            internal_iterative_deepening_depth: config_map.get("internal_iterative_deepening_depth").unwrap().parse::<u8>().unwrap(),
            internal_iterative_deepening_reduction: config_map.get("internal_iterative_deepening_reduction").unwrap().parse::<u8>().unwrap(),

            futility_pruning_depth: config_map.get("futility_pruning_depth").unwrap().parse::<u8>().unwrap(),
            futility_pruning_margin: config_map.get("futility_pruning_margin").unwrap().parse::<i32>().unwrap(),

            razoring_depth: config_map.get("razoring_depth").unwrap().parse::<u8>().unwrap(),
            razoring_margin: config_map.get("razoring_margin").unwrap().parse::<i32>().unwrap(),

            singular_extension_margin: config_map.get("singular_extension_margin").unwrap().parse::<i32>().unwrap(),

            aspiration_window_size: config_map.get("aspiration_window_size").unwrap().parse::<i32>().unwrap(),

            delta_margin: config_map.get("delta_margin").unwrap().parse::<i32>().unwrap(),

            sorting_capture_base_val: config_map.get("sorting_capture_base_val").unwrap().parse::<i32>().unwrap(),
            sorting_history_base_val: config_map.get("sorting_history_base_val").unwrap().parse::<i32>().unwrap(),
            sorting_check_capture_bonus: config_map.get("sorting_check_capture_bonus").unwrap().parse::<i32>().unwrap(),
            sorting_counter_move_val: config_map.get("sorting_counter_move_val").unwrap().parse::<i32>().unwrap(),
            sorting_killer_primary_val: config_map.get("sorting_killer_primary_val").unwrap().parse::<i32>().unwrap(),
            sorting_killer_secondary_val: config_map.get("sorting_killer_secondary_val").unwrap().parse::<i32>().unwrap(),
            sorting_checker_val: config_map.get("sorting_checker_val").unwrap().parse::<i32>().unwrap(),
            sorting_passer_val: config_map.get("sorting_passer_val").unwrap().parse::<i32>().unwrap(),

            late_move_reductions_depth: config_map.get("late_move_reductions_depth").unwrap().parse::<u8>().unwrap(),
            late_move_reductions_move_count: config_map.get("late_move_reductions_move_count").unwrap().parse::<usize>().unwrap(),
        }
    }
}
