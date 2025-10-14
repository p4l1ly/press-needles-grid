use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

const COUNT: usize = 3;
const DEBUG: bool = false;
const MM: f64 = 3.7795;
const WIDTH: f64 = 321.0 * MM;
const HEIGHT: f64 = 125.0 * MM;
const GRID_WIDTH: f64 = 2.5 * MM;
const TOP: f64 = 100.0 * MM;
const NEEDLE_HEIGHTS: [f64;8] = [
    MM * 11.5,
    MM * 11.9,
    MM * 11.9,
    MM * 11.9,
    MM * 11.9,
    MM * 11.9,
    MM * 11.9,
    MM * 11.9,
];
const NEEDLE_TOP: f64 = 121.0 * MM;
const NEEDLE_WIDTHS: [f64; 8] = [
    MM * 2.1,
    MM * 2.5,
    MM * 2.5,
    MM * 2.5,
    MM * 2.5,
    MM * 2.5,
    MM * 2.5,
    MM * 2.5,
];
const LASER_RADIUS: f64 = -0.0 * MM;

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
        .set("stroke-width", (2.0 * LASER_RADIUS).max(0.3))
        .set("d", data)
}

const HIGH_POSITIONS: [f64;18] = [
    MM * (23.5 + 2.5),
    MM * (42.5),
    MM * (56.7 + 2.5),
    MM * (75.0),
    MM * (88.3+2.5),
    MM * (105.1+2.5),
    MM * (122.6+2.5),
    MM * (138.3+2.5),
    MM * (156.1+2.5),
    MM * (172.1+2.5),
    MM * (321.0 - 128.5),
    MM * (321.0 - 112.4),
    MM * (321.0 - 96.3),
    MM * (321.0 - 79.2),
    MM * (321.0 - 62.3),
    MM * (321.0 - 45.0),
    MM * (321.0 - 28.7),
    MM * (321.0 - 12.0),
];

const LOW_POSITIONS: [f64;18] = [
    MM * (25.6),
    MM * (43.3),
    MM * (60.2),
    MM * (76.4),
    MM * (93.2),
    MM * (110.0),
    MM * (126.2),
    MM * (143.0),
    MM * (161.0),
    MM * (176.8),
    MM * (320.0 - 126.5),
    MM * (320.0 - 109.8),
    MM * (320.0 - 95.0),
    MM * (320.0 - 78.0),
    MM * (320.0 - 60.0),
    MM * (320.0 - 45.2),
    MM * (320.0 - 27.5),
    MM * (320.0 - 11.4),
];

fn grid(i: usize) -> Path {
    let data = Data::new()
        .move_to((LOW_POSITIONS[i] - GRID_WIDTH, HEIGHT))
        .line_to((HIGH_POSITIONS[i] - GRID_WIDTH, 0.0))
        .line_to((HIGH_POSITIONS[i] + GRID_WIDTH, 0.0))
        .line_to((LOW_POSITIONS[i] + GRID_WIDTH, HEIGHT))
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

fn draw_needle(count: usize, i: usize, y: f64, xdiff: f64, height: f64, width: f64, mut document: Document) -> Document {
    let (left, right) = needle_space(count, i, y, xdiff, height);
    let shift0 = (count - 1) as f64 * xdiff;
    let center = (left + right) / 2.0 + shift0;
    let left2 = center - width / 2.0 + LASER_RADIUS;
    let right2 = center + width / 2.0 - LASER_RADIUS;
    let bottom = HEIGHT - y - LASER_RADIUS;
    let top = HEIGHT - y - height + LASER_RADIUS;

    let data = Data::new()
        .move_to((left2, bottom))
        .line_to((right2, bottom))
        .line_to((right2, top))
        .line_to((left2, top))
        .close();
    document = document.add(Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", (2.0 * LASER_RADIUS).max(0.3))
        .set("d", data));
    document
}

fn main() {
    let mut document = Document::new()
        .set("viewBox", (-20.0 * MM, -1.0 * MM, WIDTH + 21.0 * MM, HEIGHT + 2.0 * MM));
    document = document.add(borders());

    if DEBUG {
        for i in 0..18 {
            document = document.add(grid(i));
        }
    }

    let mut needles: Vec<(usize, f64, f64, f64)> = vec![];
    let mut needle_top = NEEDLE_TOP;
    for ((i_offset, needle_height), needle_width) in
        [0, 1, 2].into_iter().cycle().zip(NEEDLE_HEIGHTS).zip(NEEDLE_WIDTHS)
    {
        needle_top += needle_height;
        let y = HEIGHT + TOP - needle_top;
        for i in (i_offset..17).step_by(COUNT) {
            needles.push((i, y, needle_height, needle_width));
        }
    }

    let xdiff = (590..680).map(|xdiff0| {
        let xdiff = xdiff0 as f64 * 0.1;
        let min_space = needles.iter().map(|&(i, y, height, _width)| {
            let (left, right) = needle_space(COUNT, i, y, xdiff, height);
            right - left
        }).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        (xdiff, min_space)
    }).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0;

    dbg!(xdiff);

    for (i, y, height, width) in needles {
        if DEBUG {
            document = draw_needle_space(COUNT, i, y, xdiff, height, document);
        }
        document = draw_needle(COUNT, i, y, xdiff, height, width, document);
    }

    svg::save("image.svg", &document).unwrap();
}
