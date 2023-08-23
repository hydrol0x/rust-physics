use kiss3d::window::Window;
use std::thread::{current, sleep};
use std::time::{Duration, Instant};

// TODO: make this a struct so that
/*  let mut previous_time = Instant::now();
   let time_interval = Duration::from_secs_f32(1.0 / 60.0); // Assuming 60 FPS for the time interval, adjust as needed
   let mut accumulator = Duration::new(0, 0);
*/
// can be run only once on init
// and not every time loop runs

pub struct discrete_time_step_sim {
    previous_time: Instant,
    time_interval: Duration,
    accumulator: Duration,
}

// pub type PhysicsFunction<T, U> = Fn(&mut T, &mut U);
impl discrete_time_step_sim {
    pub fn new(previous_time: Instant, time_interval: Duration, accumulator: Duration) -> Self {
        Self {
            previous_time: previous_time,
            time_interval: time_interval,
            accumulator: accumulator,
        }
    }

    pub fn run_sim<F, T1>(&mut self, springs: &mut T1, window: &mut Window, physics_calc: F)
    where
        F: Fn(&mut T1, &mut Window),
    {
        let current_time = Instant::now();
        let elapsed_time = current_time - self.previous_time;
        self.previous_time = current_time;

        self.accumulator += elapsed_time;

        while self.accumulator >= self.time_interval {
            // DO physics calculations
            physics_calc(springs, window);

            self.accumulator -= self.time_interval;
        }

        let sleep_duration = if self.time_interval > (Instant::now() - current_time) {
            self.time_interval - (Instant::now() - current_time)
        } else {
            Duration::from_secs(0)
        };

        sleep(sleep_duration);
    }
}

// pub fn discrete_time_step_sim<F>(
//     mut physics_calc: F,
//     springs: &mut Vec<Spring>,
//     window: &mut Window,
//     mut previous_time: Instant,
//     mut time_interval: Duration,
//     mut accumulator: Duration,
// ) where
//     F: FnMut(),
// {
//     let current_time = Instant::now();
//     let elapsed_time = current_time - previous_time;
//     previous_time = current_time;

//     accumulator += elapsed_time;

//     while accumulator >= time_interval {
//         physics_calc();

//         accumulator -= time_interval;
//     }

//     let sleep_duration = if time_interval > (Instant::now() - current_time) {
//         time_interval - (Instant::now() - current_time)
//     } else {
//         Duration::from_secs(0)
//     };

//     sleep(sleep_duration);
// }
