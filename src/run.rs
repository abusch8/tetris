use std::io::Result;
use crossterm::event::EventStream;
use futures::{stream::StreamExt, FutureExt};
use tokio::{pin, select, time::{interval, sleep, Duration, Interval, Instant}};

use crate::config;
use crate::display::Display;
use crate::event::handle_event;
use crate::game::{Game, ShiftDirection};

pub const LOCK_RESET_LIMIT: u8 = 15;
pub const LOCK_DURATION: Duration = Duration::from_millis(500);

fn calc_drop_interval(level: u32) -> Interval {
    let drop_rate = (0.8 - (level - 1) as f32 * 0.007).powf((level - 1) as f32);
    let drop_duration = Duration::from_nanos((drop_rate * 1_000_000_000f32) as u64);

    interval(if drop_duration.is_zero() { Duration::from_nanos(1) } else { drop_duration })
}

pub async fn run(game: &mut Game) -> Result<()> {
    let mut reader = EventStream::new();

    let display = &mut Display::new()?;

    display.draw()?;

    let frame_duration = Duration::from_nanos(if *config::MAX_FRAME_RATE > 0 {
        1_000_000_000 / *config::MAX_FRAME_RATE
    } else {
        1
    });

    let mut render_interval = interval(frame_duration);
    let mut drop_interval = calc_drop_interval(game.level);

    let mut prev_level = game.level;

    pin! {
        let lock_delay = sleep(Duration::ZERO);
    }

    let mut debug_frame_interval = interval(Duration::from_secs(1));
    let mut debug_frame = 0u64;

    Ok(loop {
        select! {
            Some(event) = reader.next().fuse() => {
                match event {
                    Ok(event) => handle_event(event, game, display, &mut lock_delay)?,
                    Err(error) => panic!("{}", error),
                };
            },
            _ = &mut lock_delay, if game.locking => {
                game.place();
            },
            _ = drop_interval.tick() => {
                if !game.hitting_bottom(&game.falling) {
                    lock_delay.as_mut().reset(Instant::now() + LOCK_DURATION);
                }
                game.shift(ShiftDirection::Down);
            },
            _ = render_interval.tick() => {
                display.render(game)?;
                debug_frame += *config::DISPLAY_FRAME_RATE as u64;
            },
            _ = debug_frame_interval.tick(), if *config::DISPLAY_FRAME_RATE => {
                display.render_debug_info(debug_frame)?;
                debug_frame = 0;
            },
            _ = async {}, if game.level != prev_level => {
                prev_level = game.level;
                drop_interval = calc_drop_interval(game.level);
            },
            _ = async {}, if game.end => {
                break;
            },
        }
    })
}

