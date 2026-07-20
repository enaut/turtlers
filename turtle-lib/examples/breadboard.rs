use turtle_lib::*;
#[cfg(feature = "svg")]
#[macroquad::main("Export SVG")]

async fn main() {
    // Create turtle plan
    let mut turtle = create_turtle_plan();

    // Set instant mode so commands execute imqmediately
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

fn pin(t: &mut TurtlePlan, size: f32) {
    t.left(90.0).forward(size / 2.0);
    for _ in 0..5 {
        t.right(90.0).forward(size);
    }
    t.right(90.0).forward(size / 2.0).left(90.0);
}

fn pin_reihe(t: &mut TurtlePlan, anzahl: usize) {
    for x in 0..anzahl {
        pin(t, 5.0);
        if x < anzahl - 1 {
            t.forward(5.0);
        }
    }
}

fn pin_spalte(t: &mut TurtlePlan, anzahl: usize, x_coord: f32) {
    for x in 0..anzahl {
        t.pen_up().go_to(vec2(x_coord, x as f32 * 10.0)).pen_down();
        pin_reihe(t, 5);
    }
}

fn pin_seite(t: &mut TurtlePlan, anzahl: usize, x_coord: f32, color: Color) {
    t.pen_up()
        .go_to(vec2(x_coord, -2.5))
        .pen_down()
        .set_pen_color(color)
        .set_heading(90.0);
    for x in 0..anzahl {
        pin(t, 5.0);
        if x < anzahl - 1 {
            t.forward(5.0);
        }
    }
}

fn breadboard(t: &mut TurtlePlan, anzahl_reihen: usize) {
    pin_spalte(t, anzahl_reihen, 0.0);
    pin_spalte(t, anzahl_reihen, 65.0);
    pin_seite(t, anzahl_reihen, -15.0, BLUE);
    pin_seite(t, anzahl_reihen, -25.0, RED);
    pin_seite(t, anzahl_reihen, 125.0, BLUE);
    pin_seite(t, anzahl_reihen, 135.0, RED);

    // draw outline
    t.pen_up().go_to(vec2(-30.0, -5.0)).pen_down();
    t.set_pen_color(BLACK)
        .forward(anzahl_reihen as f32 * 10.0 + 10.0)
        .right(90.0)
        .forward(170.0)
        .right(90.0)
        .forward(anzahl_reihen as f32 * 10.0 + 10.0)
        .right(90.0)
        .forward(170.0)
        .right(90.0);
}
