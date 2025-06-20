use std::env;

use chrono::{Datelike, Days, FixedOffset, TimeZone, Weekday};
use cmd_lib::run_cmd as run;
use colored::*;
use image::{Rgb, RgbImage};

mod config;

const FORMAT_STR: &str = "%a %b %e %T %Y %z";

const BLOCK: &str = "██";

const WIDTH: usize = 52;
const HEIGHT: usize = 7;

fn main() -> std::io::Result<()> {
    // image_gen();

    run! {
        rm -rf repo;
        cp -r skeleton repo;
        cd repo;
        git init > /dev/null;
        git add .;
    }?;

    let dt = FixedOffset::east_opt(5 * 3600 + 30 * 60)
        .unwrap()
        .with_ymd_and_hms(config::YEAR, 1, 1, 1, 1, 1)
        .unwrap();

    let offset = match dt.weekday() {
        Weekday::Sun => 0,
        w => w as u32 + 1,
    };

    let img = image::open(config::IMAGE_PATH).unwrap().into_rgb8();

    let w = img.width();
    let h = img.height();

    assert_eq!(w, WIDTH as _);
    assert_eq!(h, HEIGHT as _);

    for j in 0..h {
        for i in 0..w {
            if i == 0 && j < offset {
                print!("{}", BLOCK.red());
                continue;
            }

            let dt = dt
                .checked_add_days(Days::new((i * 7 + j - offset) as _))
                .unwrap();

            let formatted = dt.format(FORMAT_STR).to_string();

            env::set_var("GIT_AUTHOR_DATE", &formatted);
            env::set_var("GIT_COMMITTER_DATE", &formatted);

            let [r, g, b] = img.get_pixel(i, j).0;
            let brightness = (r as f32 + g as f32 + b as f32) / 3.0;
            let count = config::COMMITS_PER_WHITE_PIXEL - (brightness / (255.0 / config::COMMITS_PER_WHITE_PIXEL as f32)) as u32;

            for _ in 0..count {
                run! {
                    cd repo;
                    git commit -am ":)" --allow-empty > /dev/null;
                }?;
            }

            print!(
                "{}",
                if count != 0 {
                    BLOCK.green()
                } else {
                    BLOCK.black()
                }
            );
        }

        println!();
    }

    Ok(())
}
