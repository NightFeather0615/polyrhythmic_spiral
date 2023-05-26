use raylib::prelude::*;
use rodio::{OutputStream, source::{SineWave, Amplify, TakeDuration, Mix, FadeIn}, Source};
use std::time::{SystemTime, Duration};

fn gen_notes(arcs: usize) -> Vec<Amplify<Mix<TakeDuration<TakeDuration<SineWave>>, FadeIn<TakeDuration<TakeDuration<SineWave>>>>>> {
    // Define the base frequency of A4 (440 Hz)
    let base_frequency = 220.0;

    // Define the number of octaves
    let num_octaves = f32::ceil(arcs as f32 / 12.0) as usize;

    // Define the duration of each note (in milliseconds)
    let note_duration_ms = 150;

    // Define the volume of each note (0.0 to 1.0)
    let note_volume = 0.5;

    // Calculate the frequency ratio for each semitone
    let semitone_ratio = 2.0_f32.powf(1.0 / 12.0);

    // Create a HashMap to store the note frequencies
    let mut note_frequencies: Vec<Amplify<Mix<TakeDuration<TakeDuration<SineWave>>, FadeIn<TakeDuration<TakeDuration<SineWave>>>>>> = Vec::new();

    // Generate the note frequencies
    for octave in 0..num_octaves {
        for i in 0..12 {
            let note_frequency = base_frequency * semitone_ratio.powi((octave * 12 + i) as i32);
            let source = rodio::source::SineWave::new(note_frequency)
                .take_duration(
                    Duration::from_millis(note_duration_ms)
                )
                .take_crossfade_with(
                    SineWave::new(0.0).take_duration(Duration::from_secs(0)), 
                    Duration::from_millis(50)
                )
                .amplify(note_volume);
            note_frequencies.push(source);
        }
    }

    return note_frequencies;
}

fn gen_color_gradient(arcs: usize, color_stops: Vec<Vector3>) -> Vec<Color> {
    let mut colors: Vec<Color> = vec![];
    let color_div = f32::ceil((arcs / (color_stops.len() - 1)) as f32);
    for color_stop_index in 1..color_stops.len() {
        let current_color = color_stops[color_stop_index];
        let prev_color = color_stops[color_stop_index - 1];
        let color_diff = (current_color - prev_color) / color_div;
        for i in 0..(color_div + 1.0) as i32 {
            let color_offset = prev_color + color_diff * i as f32;
            colors.push(
                Color::new(
                    (color_offset.x * 255.0) as u8,
                    (color_offset.y * 255.0) as u8,
                    (color_offset.z * 255.0) as u8,
                    255
                )
            );
        }
    }
    return colors;
}

fn main() {
    let start_time = SystemTime::now();
    let window_width = 1600;
    let window_height = 900;
    let max_angle = std::f32::consts::PI * 2.0;
    let init_radius = 35.0;
    let arcs = 32;
    let loops = 75.0;
    let full_loop = std::f32::consts::PI * 2.0;
    let velocity = full_loop * loops / 900.0;
    let spacing = (((window_width - 200) as f32 - init_radius) / arcs as f32) / 2.0;

    let mut arc_impact_times = vec![0.0; arcs];


    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let notes = gen_notes(arcs);

    let color_stops = vec![
        Vector3::new(1.000, 1.000, 1.000),
        Vector3::new(0.384, 0.529, 0.875),
        Vector3::new(0.851, 0.634, 0.835),
        Vector3::new(1.000, 0.925, 0.792),
        Vector3::new(1.000, 1.000, 1.000),
    ];
    let colors = gen_color_gradient(arcs, color_stops);

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("polyrhythmic_spiral")
        .msaa_4x()
        .vsync()
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_line_ex(
            Vector2::new(
                100.0, 
                (window_height - 100) as f32
            ), 
            Vector2::new(
                (window_width - 100) as f32, 
                (window_height - 100) as f32
            ),
            2.0,
            Color::WHITE
        );
        
        for i in 0..arcs {
            d.draw_ring(
                Vector2::new(
                    (window_height - 100) as f32,
                    (window_height - 100) as f32
                ), 
                init_radius - 2.0 + i as f32 * spacing,
                init_radius + i as f32 * spacing,
                270.0, 
                90.0, 
                350, 
                colors[i]
            );
        }
        
        let elapsed_time = start_time.elapsed().unwrap().as_secs_f32();

        for i in 0..arcs {
            let arc_velocity = velocity - i as f32 * 0.003;
            if elapsed_time >= arc_impact_times[i] {
                stream_handle.play_raw(notes[i].clone().convert_samples()).unwrap();
                arc_impact_times[i] += std::f32::consts::PI / arc_velocity; 
            }
            let distance = std::f32::consts::PI + elapsed_time * arc_velocity;
            let mod_distance = distance % max_angle;
            let adjusted_distance = if mod_distance >= std::f32::consts::PI {
                mod_distance
            } else {
                max_angle - mod_distance
            };
            d.draw_circle_v(
                Vector2::new(
                    (window_height - 100) as f32 + (init_radius - 1.0 + i as f32 * spacing) * adjusted_distance.cos(),
                    (window_height - 100) as f32 + (init_radius - 1.0 + i as f32 * spacing) * adjusted_distance.sin()
                ),
                6.5, 
                Color::WHITE
            )
        }
    }
}