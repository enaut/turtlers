use turtle_lib::*;

#[cfg(feature = "svg")]
#[macroquad::main("Export SVG")]
async fn main() {
    // Create turtle plan
    let mut turtle = create_turtle_plan();

    // Set instant mode so commands execute immediately
    turtle.set_speed(1200).set_pen_width(0.5);

    breadboard(&mut turtle, 65);

    turtle.hide();
    let mut app = TurtleApp::new().with_commands(turtle.build());
    use macroquad::{
        input::{is_key_pressed, KeyCode},
        text::draw_text,
        window::{clear_background, next_frame},
    };

    loop {
        clear_background(WHITE);
        app.update();
        app.render();

        draw_text("Press E for SVG export", 20.0, 40.0, 32.0, BLACK);

        if is_key_pressed(KeyCode::E) {
            match app.export_drawing("test.svg", export::DrawingFormat::Svg) {
                Ok(_) => println!("SVG exported to test.svg"),
                Err(e) => eprintln!("Export error: {:?}", e),
            }
        }

        next_frame().await;
    }
}

#[cfg(not(feature = "svg"))]
fn main() {
    println!("SVG export is not enabled. Build with --features svg");
}

#[cfg(feature = "svg")]
fn pin(t: &mut TurtlePlan, size: f32) {
    t.left(90.0).forward(size / 2.0);
    for _ in 0..5 {
        t.right(90.0).forward(size);
    }
    t.right(90.0).forward(size / 2.0).left(90.0);
}

#[cfg(feature = "svg")]
fn pin_row(t: &mut TurtlePlan, count: usize) {
    for x in 0..count {
        pin(t, 5.0);
        if x < count - 1 {
            t.forward(5.0);
        }
    }
}

#[cfg(feature = "svg")]
fn pin_column(t: &mut TurtlePlan, count: usize, x_coord: f32) {
    for x in 0..count {
        t.pen_up().go_to(vec2(x_coord, x as f32 * 10.0)).pen_down();
        pin_row(t, 5);
    }
}

#[cfg(feature = "svg")]
fn pin_side(t: &mut TurtlePlan, count: usize, x_coord: f32, color: Color) {
    t.pen_up()
        .go_to(vec2(x_coord, -2.5))
        .pen_down()
        .set_pen_color(color)
        .set_heading(90.0);
    for x in 0..count {
        pin(t, 5.0);
        if x < count - 1 {
            t.forward(5.0);
        }
    }
}

#[cfg(feature = "svg")]
fn breadboard(t: &mut TurtlePlan, row_count: usize) {
    pin_column(t, row_count, 0.0);
    pin_column(t, row_count, 65.0);
    pin_side(t, row_count, -15.0, BLUE);
    pin_side(t, row_count, -25.0, RED);
    pin_side(t, row_count, 125.0, BLUE);
    pin_side(t, row_count, 135.0, RED);

    // draw outline
    t.pen_up().go_to(vec2(-30.0, -5.0)).pen_down();
    t.set_pen_color(BLACK)
        .forward(row_count as f32 * 10.0 + 10.0)
        .right(90.0)
        .forward(170.0)
        .right(90.0)
        .forward(row_count as f32 * 10.0 + 10.0)
        .right(90.0)
        .forward(170.0)
        .right(90.0);
}
