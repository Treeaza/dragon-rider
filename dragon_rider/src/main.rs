/*
 * main.rs of dragon_rider crate
 *
 * Dragon fractal generator written by Jasper Rubin (www.jasperrubin.com).
 * 
 * Copyright 2021, Jasper Rubin
 * All rights reserved
 *
**/

mod vectors;

use draw::*;

const LINE_LENGTH: i32 = 20;
const LINE_THICKNESS: u32 = 2;
const CORNER_FACTOR: f32 = 0.25;

fn next_fractal_step(current_step: &Vec<vectors::TurnDirection>) -> Vec<vectors::TurnDirection> {
    // vec to hold inverted elements
    let mut inv = Vec::new();
    let mut tmp = current_step.to_vec();
    tmp.reverse();
    let mut ret = current_step.to_vec();

    // invert elements
    for element in tmp.iter() {
        inv.push(element.opposite());
    }

    // add to the end of previous step
    ret.push(vectors::TurnDirection::RIGHT);
    ret.append(&mut inv);

    ret
}

fn fractal_n_step(n: u32) -> Vec<vectors::TurnDirection> {
    if n > 1 {
        return next_fractal_step(&fractal_n_step(n - 1));
    } else {
        return vec![vectors::TurnDirection::RIGHT];
    }
}

fn create_fractal_lines(current_step: &Vec<vectors::TurnDirection>) -> (Vec<vectors::Vector2i>, i32, i32, i32, i32) {
    // create our starting point
    let mut facing = vectors::Direction::NORTH;
    let mut point = vectors::Vector2i { x: 0, y: 0};
    let (mut min_x, mut max_x, mut min_y, mut max_y): (i32, i32, i32, i32) = (0, 0, 0, 0);

    // vec to hold the output
    let mut points: Vec<vectors::Vector2i> = Vec::new();

    // iterate over directions
    for seg in current_step.iter() {
        // find the move to the next point
        let change = facing.cartesian_move() * LINE_LENGTH;

        // draw line from point to point + change
        points.push(vectors::Vector2i { x: point.x + change.x, y: point.y + change.y} );

        // update the point and direction
        point += change;
        facing = facing.next_direction(*seg);

        // check bounds
        if point.x < min_x {
            min_x = point.x
        } else if point.x > max_x {
            max_x = point.x;
        }

        if point.y < min_y {
            min_y = point.y;
        } else if point.y > max_y {
            max_y = point.y;
        }
    }

    // return points and bounds
    (points, min_x, min_y, max_x, max_y)
}

fn create_fractal_lines_chamfered(current_step: &Vec<vectors::TurnDirection>) -> (Vec<vectors::Vector2i>, i32, i32, i32, i32) {
    // create our starting point
    let mut facing = vectors::Direction::NORTH;
    let mut point = vectors::Vector2i { x: 0, y: 0};
    let (mut min_x, mut max_x, mut min_y, mut max_y): (i32, i32, i32, i32) = (0, 0, 0, 0);

    // vec to hold the output
    let mut points: Vec<vectors::Vector2i> = vec![vectors::Vector2i { x: 0, y: 0 }];

    // iterate over directions
    for seg in current_step.iter() {
        // find the move to the next point
        let change = facing.cartesian_move() * LINE_LENGTH;

        // draw line from point to point + change
        let f_point = vectors::Vector2i {x: point.x + (change.x as f32 * (1.0 - CORNER_FACTOR)) as i32, y: point.y + (change.y as f32 * (1.0 - CORNER_FACTOR)) as i32};
        points.push(f_point);

        // update the point and direction
        point += change;
        facing = facing.next_direction(*seg);

        // check bounds
        if point.x < min_x {
            min_x = point.x
        } else if point.x > max_x {
            max_x = point.x;
        }

        if point.y < min_y {
            min_y = point.y;
        } else if point.y > max_y {
            max_y = point.y;
        }

        let m_change = facing.cartesian_move() * LINE_LENGTH;
        let m_point = vectors::Vector2i { x: point.x + (m_change.x as f32 * CORNER_FACTOR) as i32, y: point.y + (m_change.y as f32 * CORNER_FACTOR) as i32};

        // push the next point, to chamfer the corner
        points.push(m_point);
    }

    // return points and bounds
    (points, min_x, min_y, max_x, max_y)
}

fn main() {
    let l = fractal_n_step(16);

    let (points, min_x, min_y, max_x, max_y) = create_fractal_lines_chamfered(&l);
    let mut line = LineBuilder::new((points[0].x - min_x) as f32, (points[0].y - min_y) as f32);

    for p in points.iter() {
        line = line.line_to((p.x - min_x) as f32, (p.y - min_y) as f32);
    }

    let drawing = Drawing::new().with_shape(line.build()).with_style(Style::stroked(LINE_THICKNESS, Color::black()));
    let mut canvas = Canvas::new((max_x - min_x) as u32, (max_y - min_y) as u32);
    canvas.display_list.add(drawing);

    render::save(&canvas, &format!("dragon_f.svg"), SvgRenderer::new()).expect("Failed to save");
}

