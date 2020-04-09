/*
 * Copyright (C) 2020 Zixiao Han
 */

static OVERHEAD_TIME: u128 = 50;

pub struct TimeCapacity {
    pub main_time_millis: u128,
    pub extra_time_millis: u128,
}

pub fn calculate_time_capacity(total_time_millis: u128, moves_to_go: u128, increment: u128) -> TimeCapacity {
    let main_time_millis = (total_time_millis + increment * moves_to_go) / moves_to_go;

    let extra_time_millis = if total_time_millis > main_time_millis {
        (total_time_millis - main_time_millis) / moves_to_go
    } else {
        0
    };

    if main_time_millis > OVERHEAD_TIME {
        TimeCapacity {
            main_time_millis: main_time_millis - OVERHEAD_TIME,
            extra_time_millis: extra_time_millis - OVERHEAD_TIME,
        }
    } else {
        TimeCapacity {
            main_time_millis,
            extra_time_millis: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_time_capacity() {
        let time_capacity = calculate_time_capacity(180_000, 40, 0);
        assert_eq!(4450, time_capacity.main_time_millis);
        assert_eq!(4337, time_capacity.extra_time_millis);

        let time_capacity = calculate_time_capacity(5000, 28, 1000);
        assert_eq!(1128, time_capacity.main_time_millis);
        assert_eq!(86, time_capacity.extra_time_millis);
    }
}
