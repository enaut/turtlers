//! Beispiel: Exportiere ein SVG aus einer einfachen Zeichnung

use turtle_lib::*;

#[cfg(feature = "svg")]
#[macroquad::main("Export SVG")]
async fn main() {
    let mut plan = create_turtle_plan();
    plan.forward(100.0)
        .right(90.0)
        .forward(100.0)
        .circle_left(50.0, 90.0, 4)
        .begin_fill()
        .forward(100.0)
        .right(90.0)
        .forward(200.0)
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
