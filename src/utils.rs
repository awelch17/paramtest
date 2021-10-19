

fn note_frequency(note_number: i32) -> f64 {
    return 440f64 * 2f64.powf((note_number - 69) as f64 / 12f64);
}

fn period_samples(note_number: i32, sample_rate: f64) -> f64 {
    let freq = note_frequency(note_number);
    
    let period_seconds = 1f64 / freq;
    
    return period_seconds * sample_rate;
}

