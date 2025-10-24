//! Beispiel: Exportiere ein SVG aus einer einfachen Zeichnung

use turtle_lib::*;

#[cfg(feature = "svg")]
#[macroquad::main("Export SVG")]
async fn main() {
    let mut plan = create_turtle_plan();
    plan.forward(100.0)
        .right(90.0)
        .forward(100.0)
        .set_pen_color(macroquad::color::GRAY)
        .set_pen_width(8.0)
        .circle_right(50.0, 90.0, 4)
        .begin_fill()
        .forward(100.0)
        .right(90.0)
        .forward(200.0)
        .end_fill()
        .circle_left(200., 180., 24)
        .circle_left(90.0, 180.0, 36)
        .begin_fill()
        .circle_left(90.0, 180.0, 36)
        .circle_left(45.0, 180.0, 26)
        .circle_right(45.0, 180.0, 26)
        .pen_up()
        .right(90.0)
        .forward(37.0)
        .left(90.0)
        .pen_down()
        .circle_right(8.0, 360.0, 12)
        .pen_up()
        .right(90.0)
        .forward(90.0)
        .left(90.0)
        .pen_down()
        .circle_right(8.0, 360.0, 12)
        .end_fill();
    let mut app = TurtleApp::new().with_commands(plan.build());
    use macroquad::{
        input::{is_key_pressed, KeyCode},
        text::draw_text,
        window::{clear_background, next_frame},
    };

    loop {
        clear_background(WHITE);
        app.update();
        app.render();

        draw_text("Drücke E für SVG-Export", 20.0, 40.0, 32.0, BLACK);

        if is_key_pressed(KeyCode::E) {
            match app.export_drawing("test.svg", export::DrawingFormat::Svg) {
                Ok(_) => println!("SVG exportiert nach test.svg"),
                Err(e) => println!("Fehler beim Export: {:?}", e),
            }
        }

        next_frame().await;
    }
}

#[cfg(not(feature = "svg"))]
fn main() {
    println!("SVG-Export ist nicht aktiviert. Baue mit --features svg");
}
