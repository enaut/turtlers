//! Example: Game Logic in Separate Thread
//!
//! This example demonstrates how to run game logic in a separate thread
//! while keeping the render loop responsive on the main thread.
//!
//! The main thread handles rendering and animation, while game logic
//! threads can perform blocking operations (like fetching data) and
//! send turtle commands via channels.

use std::thread;
use std::time::Duration;
use turtle_lib::*;

#[macroquad::main("Game Logic Threading")]
async fn main() {
    let mut app = TurtleApp::new();

    // Create two turtles and get their command senders
    let turtle1_tx = app.create_turtle_channel(100);
    let turtle2_tx = app.create_turtle_channel(100);

    // Spawn first game logic thread
    let _thread1 = thread::spawn({
        let tx = turtle1_tx.clone();
        move || {
            // Simulate some blocking work (e.g., network request, calculation)
            println!("Thread 1: Starting work...");
            thread::sleep(Duration::from_millis(500));

            // Now send turtle commands
            let mut plan = create_turtle_plan();
            plan.set_pen_color(BLUE)
                .forward(100.0)
                .right(90.0)
                .forward(100.0)
                .right(90.0)
                .forward(100.0)
                .right(90.0)
                .forward(100.0);

            tx.send(plan.build())
                .expect("Failed to send commands for turtle 1");
            println!("Thread 1: Commands sent!");

            // Send more commands in a loop
            for i in 0..10 {
                thread::sleep(Duration::from_millis(300));
                let mut step = create_turtle_plan();
                step.right(36.0).forward(50.0);
                let _ = tx.try_send(step.build());
                println!("Thread 1: Step {} sent", i + 1);
            }
        }
    });

    // Spawn second game logic thread
    let _thread2 = thread::spawn({
        let tx = turtle2_tx.clone();
        move || {
            // Different timing than thread1
            println!("Thread 2: Starting work...");
            thread::sleep(Duration::from_millis(1000));

            // Draw a circle with turtle2
            let mut plan = create_turtle_plan();
            plan.set_pen_color(RED).circle_left(75.0, 360.0, 72);

            tx.send(plan.build())
                .expect("Failed to send commands for turtle 2");
            println!("Thread 2: Circle sent!");
        }
    });

    // Main render loop
    let mut frame_count = 0;
    loop {
        // Check for quit
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Escape)
            || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Q)
        {
            break;
        }

        // Clear background
        macroquad::prelude::clear_background(WHITE);

        // Process incoming commands from game logic threads
        // This drains all pending commands from turtle channels
        app.process_commands();

        // Update animation state (tweening, etc.)
        app.update();

        // Render the turtles
        app.render();

        frame_count += 1;
        if frame_count % 60 == 0 {
            println!("Rendered {} frames", frame_count);
        }

        macroquad::prelude::next_frame().await;
    }

    println!("Finished!");
}
