use web_sys::AudioContext;

/// A short three note chime, played when a step comes due.
pub fn chime() {
    let Ok(ctx) = AudioContext::new() else { return };
    let t0 = ctx.current_time();
    for (i, freq) in [523.25_f32, 659.25, 783.99, 1046.5].into_iter().enumerate() {
        let (Ok(osc), Ok(gain)) = (ctx.create_oscillator(), ctx.create_gain()) else { return };
        osc.frequency().set_value(freq);
        let start = t0 + i as f64 * 0.16;
        let g = gain.gain();
        let _ = g.set_value_at_time(0.0001, start);
        let _ = g.exponential_ramp_to_value_at_time(0.3, start + 0.02);
        let _ = g.exponential_ramp_to_value_at_time(0.0001, start + 0.7);
        let _ = osc.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&ctx.destination());
        let _ = osc.start_with_when(start);
        let _ = osc.stop_with_when(start + 0.75);
    }
}
