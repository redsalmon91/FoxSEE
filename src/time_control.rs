/*
 * Copyright (C) 2020 Zixiao Han
 */

static OVERHEAD_TIME: u128 = 50;

pub struct TimeCapacity {
    pub main_time_millis: u128,
    pub extra_time_millis: u128,
}

pub fn calculate_time_capacity(total_time_millis: u128, moves_to_go: u128, increment: u128) -> TimeCapacity {
    let main_time_millis = total_time_millis / (1 + moves_to_go * 8 / 10) + increment / 2;

    let extra_time_millis = if total_time_millis > main_time_millis {
        (total_time_millis - main_time_millis) / moves_to_go
    } else {
        0
    };

    if main_time_millis > OVERHEAD_TIME {
        TimeCapacity {
            main_time_millis: main_time_millis - OVERHEAD_TIME,
            extra_time_millis: extra_time_millis,
        }
    } else {
        TimeCapacity {
            main_time_millis: main_time_millis / 2,
            extra_time_millis: 0,
        }
    }
}
