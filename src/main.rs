use svg::Document;
use svg::node::element::{Circle, Path};
use svg::node::element::path::Data;

const COUNT: usize = 3;
const DEBUG: bool = false;
const WIDTH: f64 = 321.0;
const HEIGHT: f64 = 125.0;
const GRID_WIDTH: f64 = 2.5;
const TOP: f64 = 100.0;
const NEEDLE_HEIGHT1: f64 = 12.4;
const NEEDLE_HEIGHT2: f64 = 14.73;
const NEEDLE_TOP1: f64 = 121.0;
const NEEDLE_TOP2: f64 = 141.75;
const NEEDLE_WIDTH: f64 = 3.0;
const LASER_RADIUS: f64 = 0.5;

fn borders() -> Path {
    let data = Data::new()
        .move_to((0, 0))
        .line_by((WIDTH, 0))
        .line_by((0, HEIGHT))
        .line_by((-WIDTH, 0))
        .close();

    Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 0.1)
        .set("d", data)
}

const HIGH_POSITIONS: [f64;18] = [
    23.5 + 2.5,
    42.5,
    56.7 + 2.5,
    75.0,
    88.3+2.5,
    105.1+2.5,
    122.6+2.5,
    138.3+2.5,
    156.1+2.5,
    172.1+2.5,
    321.0 - 128.5,
    321.0 - 112.4,
    321.0 - 96.3,
    321.0 - 79.2,
    321.0 - 62.3,
    321.0 - 45.0,
    321.0 - 28.7,
    321.0 - 12.0,
];

const LOW_POSITIONS: [f64;18] = [
    25.6,
    43.3,
    60.2,
    76.4,
    93.2,
    110.0,
    126.2,
    143.0,
    161.0,
    176.8,
    320.0 - 126.5,
    320.0 - 109.8,
    320.0 - 95.0,
    320.0 - 78.0,
    320.0 - 60.0,
    320.0 - 45.2,
    320.0 - 27.5,
    320.0 - 11.4,
];

fn grid(i: usize) -> Path {
    let data = Data::new()
        .move_to((LOW_POSITIONS[i] - GRID_WIDTH, 125.0))
        .line_to((HIGH_POSITIONS[i] - GRID_WIDTH, 0.0))
        .line_to((HIGH_POSITIONS[i] + GRID_WIDTH, 0.0))
        .line_to((LOW_POSITIONS[i] + GRID_WIDTH, 125.0))
        .close();
    Path::new()
        .set("fill", "grey")
        .set("stroke", "none")
        .set("d", data)
}

fn y_joint(i: usize, y: f64) -> f64 {
    let low = LOW_POSITIONS[i];
    let high = HIGH_POSITIONS[i];
    let m = (high - low) / HEIGHT;
    let c = low;
    m * y + c
}

fn circle(x: f64, y: f64, color: &'static str) -> Circle {
    Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", 0.7)
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", 0.5)
}

fn right_border(count: usize, i: isize, y: f64, xdiff: f64) -> f64 {
    let mut min = if i < 0 {
        y_joint(0, y) - GRID_WIDTH
    } else {
        y_joint(i as usize, y) - GRID_WIDTH
    };
    for j in 1..count {
        let i2 = i + j as isize;
        if i2 >= 0 {
            let x = y_joint(i2 as usize, y) - GRID_WIDTH - xdiff * j as f64;
            if x < min {
                min = x;
            }
        }
    }
    min
}

fn left_border(count: usize, i: isize, y: f64, xdiff: f64) -> f64 {
    let mut max = if i <= 0 {
        -std::f64::INFINITY
    } else {
        y_joint(i as usize - 1, y) + GRID_WIDTH
    };
    for j in 0..count - 1 {
        let i2 = i + j as isize;
        let x = if i2 < 0 {
            -std::f64::INFINITY
        } else {
            y_joint(i2 as usize, y) + GRID_WIDTH - xdiff * (j + 1) as f64
        };
        if x > max {
            max = x;
        }
    }
    max
}

fn needle_space(count: usize, i: usize, y: f64, xdiff: f64, ydiff: f64) -> (f64, f64) {
    let i2 = i.wrapping_sub(count.wrapping_sub(2)) as isize;
    let r = (
        left_border(count, i2, y, xdiff).max(left_border(count, i2, y + ydiff, xdiff)),
        right_border(count, i2, y, xdiff).min(right_border(count, i2, y + ydiff, xdiff)),
    );
    r
}

fn draw_needle_space(count: usize, i: usize, y: f64, xdiff: f64, ydiff: f64, mut document: Document) -> Document {
    let (left, right) = needle_space(count, i, y, xdiff, ydiff);
    let shift0 = (count - 1) as f64 * xdiff;
    let mut color = "blue";
    for j in 0..count {
        let shift = shift0 - j as f64 * xdiff;
        let (left2, right2) = (left + shift, right + shift);
        let data = Data::new()
            .move_to((left2, HEIGHT - y))
            .line_to((right2, HEIGHT - y))
            .line_to((right2, HEIGHT - y - ydiff))
            .line_to((left2, HEIGHT - y - ydiff))
            .close();
        document = document.add(Path::new()
            .set("fill", color)
            .set("d", data));
        color = "lightblue";
    }
    document
}

fn draw_needle(count: usize, i: usize, y: f64, xdiff: f64, ydiff: f64, mut document: Document) -> Document {
    let (left, right) = needle_space(count, i, y, xdiff, ydiff);
    let shift0 = (count - 1) as f64 * xdiff;
    let center = (left + right) / 2.0 + shift0;
    let left2 = center - NEEDLE_WIDTH / 2.0 + LASER_RADIUS;
    let right2 = center + NEEDLE_WIDTH / 2.0 - LASER_RADIUS;
    let bottom = HEIGHT - y - LASER_RADIUS;
    let top = HEIGHT - y - ydiff + LASER_RADIUS;

    let data = Data::new()
        .move_to((left2, bottom))
        .line_to((right2, bottom))
        .line_to((right2, top))
        .line_to((left2, top))
        .close();
    document = document.add(Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 2.0 * LASER_RADIUS)
        .set("d", data));
    document
}

fn main() {
    let mut document = Document::new()
        .set("viewBox", (-20, -1, WIDTH + 21.0, HEIGHT + 2.0));

    if DEBUG {
        document = document.add(borders());
        for i in 0..18 {
            document = document.add(grid(i));
        }


        let xdiff = 16.2;
        document = document.add(circle(left_border(COUNT, -1, 5.0, xdiff), HEIGHT - 5.0, "yellow"));
        document = document.add(circle(right_border(COUNT, -1, 5.0, xdiff), HEIGHT - 5.0, "green"));

        document = document.add(circle(left_border(COUNT, 0, 5.0, xdiff), HEIGHT - 5.0, "yellow"));
        document = document.add(circle(right_border(COUNT, 0, 5.0, xdiff), HEIGHT - 5.0, "green"));

        document = document.add(circle(left_border(COUNT, 1, 5.0, xdiff), HEIGHT - 5.0, "yellow"));
        document = document.add(circle(right_border(COUNT, 1, 5.0, xdiff), HEIGHT - 5.0, "green"));
    }

    let mut needles: Vec<(usize, f64, f64)> = vec![];
    for (needle_type_ix, i_offset) in (0..5).zip([0, 1, 2, 0, 1]) {
        let y = HEIGHT + TOP - NEEDLE_TOP1 - NEEDLE_HEIGHT1 * (needle_type_ix + 1) as f64;
        for i in (i_offset..17).step_by(COUNT) {
            needles.push((i, y, NEEDLE_HEIGHT1));
        }
    }

    for (needle_type_ix, i_offset) in (0..5).zip([2, 0, 1, 2, 0]) {
        if needle_type_ix < 3 {
            continue;
        }
        let y = HEIGHT + TOP - NEEDLE_TOP2 - NEEDLE_HEIGHT2 * (needle_type_ix + 1) as f64;
        for i in (i_offset..17).step_by(COUNT) {
            needles.push((i, y, NEEDLE_HEIGHT2));
        }
    }

    let xdiff = (156..180).map(|xdiff0| {
        let xdiff = xdiff0 as f64 * 0.1;
        let min_space = needles.iter().map(|&(i, y, ydiff)| {
            let (left, right) = needle_space(COUNT, i, y, xdiff, ydiff);
            right - left
        }).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        (xdiff, min_space)
    }).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0;

    dbg!(xdiff);

    for (i, y, ydiff) in needles {
        if DEBUG {
            document = draw_needle_space(COUNT, i, y, xdiff, ydiff, document);
        }
        document = draw_needle(COUNT, i, y, xdiff, ydiff, document);
    }

    svg::save("image.svg", &document).unwrap();
}
