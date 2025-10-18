//! Hangman Game with Threading
//!
//! A classic hangman game where game logic runs in a separate thread
//! while the render loop stays responsive. The user can make guesses
//! while turtle animations play smoothly.
//!
//! Run with: `cargo run --package turtle-lib --example hangman_threaded`

use dialog::DialogBox;
use std::thread;
use turtle_lib::*;

// Word list for the game
const WORDS: &[&str] = &[
    "turtle",
    "graphics",
    "threading",
    "rust",
    "animation",
    "crossbeam",
    "channel",
    "synchronization",
    "parallel",
    "concurrent",
];

#[macroquad::main("Hangman")]
async fn main() {
    let mut app = TurtleApp::new();

    // Create three turtles: hangman, lines, and smiley
    let hangman_tx = app.create_turtle_channel(100);
    let lines_tx = app.create_turtle_channel(100);
    let smiley_tx = app.create_turtle_channel(100);

    // Spawn game logic thread
    let game_thread = thread::spawn({
        let hangman = hangman_tx.clone();
        let lines = lines_tx.clone();
        let smiley = smiley_tx.clone();

        move || {
            run_game_logic(hangman, lines, smiley);
        }
    });

    // Main render loop
    loop {
        // Check for quit
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Escape)
            || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Q)
        {
            break;
        }

        // Clear and render
        macroquad::prelude::clear_background(WHITE);
        app.process_commands();
        app.update();
        app.render();

        macroquad::prelude::next_frame().await;
    }

    // Wait for game thread to finish
    game_thread.join().ok();
    println!("Game ended. Goodbye!");
}

fn run_game_logic(
    hangman_tx: TurtleCommandSender,
    lines_tx: TurtleCommandSender,
    smiley_tx: TurtleCommandSender,
) {
    let secret = choose_word();
    println!("Starting hangman game...");
    println!("Secret word has {} letters", secret.len());

    // hide the smiley at start
    {
        let mut plan = create_turtle_plan();
        plan.hide();
        smiley_tx.send(plan.build()).ok();
    }

    draw_smiley(&smiley_tx, true);

    // Setup: Position hangman turtle and draw base (hill)
    {
        let mut plan = create_turtle_plan();
        setup_hangman(&mut plan);
        draw_hill(&mut plan);
        hangman_tx.send(plan.build()).ok();
    }

    let mut all_guesses = String::new();
    let mut wrong_guesses = 0;
    const MAX_WRONG: usize = 8; // 8 body parts after base

    // Draw current state of lines
    draw_lines_state(&lines_tx, &secret, &all_guesses);
    // Main game loop
    loop {
        // Check if won
        if secret.chars().all(|c| all_guesses.contains(c)) {
            draw_smiley(&smiley_tx, true);
            break;
        }

        // Check if lost
        if wrong_guesses >= MAX_WRONG {
            draw_smiley(&smiley_tx, false);
            break;
        }

        // Ask for guess
        let guess = ask_for_letter();
        let guess_lower = guess.to_lowercase();

        all_guesses.push_str(&guess_lower);

        if secret.contains(&guess_lower) {
            println!("✓ Correct! '{}' is in the word", guess_lower);
            // Draw current state of lines
            draw_lines_state(&lines_tx, &secret, &all_guesses);
        } else {
            println!("✗ Wrong! '{}' is NOT in the word", guess_lower);
            wrong_guesses += 1;

            // Draw next hangman step
            draw_hangman_step(&hangman_tx, wrong_guesses);
            println!("Wrong guesses: {}/{}", wrong_guesses, MAX_WRONG);
        }
    }
}

fn choose_word() -> &'static str {
    WORDS[(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize)
        % WORDS.len()]
}

fn ask_for_letter() -> String {
    loop {
        match dialog::Input::new("Enter a single letter to guess")
            .title("Hangman - Letter Guess")
            .show()
        {
            Ok(Some(input)) => {
                let trimmed = input.trim();
                if trimmed.len() == 1 && trimmed.chars().all(|c| c.is_alphabetic()) {
                    return trimmed.to_lowercase();
                } else {
                    // Invalid input, show error and retry
                    let _ = dialog::Message::new("Please enter exactly one letter (a-z).")
                        .title("Invalid Input")
                        .show();
                    // Loop continues to ask again
                }
            }
            Ok(None) => {
                // User cancelled - ask again
                continue;
            }
            Err(e) => {
                // Dialog system failed - exit game
                eprintln!("Dialog error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn setup_hangman(plan: &mut TurtlePlan) {
    plan.hide()
        .set_speed(1001) // Instant mode
        .set_pen_width(3.0) // Thicker lines for visibility
        .set_pen_color(BLACK)
        .pen_up()
        .go_to(vec2(-100.0, -100.0)) // More centered position
        .pen_down();
}

fn draw_hangman_step(tx: &TurtleCommandSender, step: usize) {
    let mut plan = create_turtle_plan();
    plan.set_speed(1001); // Instant mode

    match step {
        1 => draw_mast(&mut plan),
        2 => draw_bar(&mut plan),
        3 => draw_support(&mut plan),
        4 => draw_rope(&mut plan),
        5 => draw_head(&mut plan),
        6 => draw_arms(&mut plan),
        7 => draw_body(&mut plan),
        8 => draw_legs(&mut plan),
        _ => {}
    }

    tx.send(plan.build()).ok();
}

// Hangman drawing functions (scaled down for visibility)
fn draw_hill(plan: &mut TurtlePlan) {
    plan.left(135.0)
        .circle_left(100.0, 90.0, 36)
        .circle_left(100.0, -45.0, 36)
        .right(90.0);
}

fn draw_mast(plan: &mut TurtlePlan) {
    plan.forward(150.0);
}

fn draw_bar(plan: &mut TurtlePlan) {
    plan.right(90.0).forward(75.0);
}

fn draw_support(plan: &mut TurtlePlan) {
    plan.backward(50.0)
        .right(135.0)
        .forward(35.355)
        .backward(35.355)
        .left(135.0)
        .forward(50.0);
}

fn draw_rope(plan: &mut TurtlePlan) {
    plan.set_pen_width(2.0).right(90.0).forward(35.0);
}

fn draw_head(plan: &mut TurtlePlan) {
    plan.left(90.0).circle_right(15.0, 540.0, 72);
}

fn draw_arms(plan: &mut TurtlePlan) {
    plan.left(60.0)
        .forward(50.0)
        .backward(50.0)
        .left(60.0)
        .forward(50.0)
        .backward(50.0)
        .right(30.0);
}

fn draw_body(plan: &mut TurtlePlan) {
    plan.forward(50.0);
}

fn draw_legs(plan: &mut TurtlePlan) {
    plan.right(20.0)
        .forward(60.0)
        .backward(60.0)
        .left(40.0)
        .forward(60.0)
        .backward(60.0)
        .right(20.0);
}

fn draw_lines_state(tx: &TurtleCommandSender, secret: &str, all_guesses: &str) {
    let mut plan = create_turtle_plan();
    plan.reset()
        //.hide()
        .set_pen_color(BLACK)
        .set_pen_width(2.0)
        .pen_up()
        .go_to(vec2(-100.0, 100.0))
        .pen_down();

    // Draw underscores/circles for each letter with text labels
    for letter in secret.chars() {
        if all_guesses.contains(letter) {
            // Draw green circle for revealed letter
            plan.set_pen_color(GREEN)
                .write_text(&letter.to_uppercase().to_string(), 32u16)
                .forward(15.0);
        } else {
            // Draw black underscore
            plan.set_pen_color(BLACK).forward(15.0);
        }
        plan.pen_up().forward(15.0).pen_down();
    }

    tx.send(plan.build()).ok();
}

fn draw_smiley(tx: &TurtleCommandSender, won: bool) {
    let mut plan = create_turtle_plan();
    plan.reset()
        .hide()
        .set_speed(1001) // Instant mode
        .pen_up()
        .go_to(vec2(100.0, -100.0)) // Right side of screen
        .pen_down()
        .set_pen_color(if won { GREEN } else { RED })
        .left(90.);

    // Face
    plan.circle_right(50.0, 360.0, 72);

    // Left eye
    plan.pen_up()
        .forward(10.0)
        .right(90.0)
        .forward(27.5)
        .pen_down()
        .circle_left(3.0, 360.0, 24);

    // Right eye
    plan.pen_up()
        .forward(45.0)
        .pen_down()
        .circle_left(3.0, 360.0, 24);

    // Mouth
    plan.pen_up()
        .backward(42.5)
        .left(90.0)
        .backward(30.0)
        .right(90.0)
        .pen_down();

    if won {
        // Smile
        plan.right(45.0).circle_left(32.5, 90.0, 36);
    } else {
        // Frown
        plan.left(45.0).circle_right(32.5, 90.0, 36);
    }

    tx.send(plan.build()).ok();
}
