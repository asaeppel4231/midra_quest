use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::time::Duration;

fn main() {
    // 1. WAV-Datei komplett als i16-Samples in den RAM laden
    let mut reader = hound::WavReader::open("soundtest/Track 1.wav").expect("WAV nicht gefunden");
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    // 2. Standard-Soundkarte & passende Konfiguration holen
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Kein Ausgabegerät gefunden");
    let config = cpal::StreamConfig {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        buffer_size: cpal::BufferSize::Default,
    };

    // 3. Echtzeit-Stream bauen: Kopiert Stück für Stück aus `samples` zur Soundkarte
    let mut pos = 0;
    let stream = device
        .build_output_stream(
            config,
            move |output: &mut [i16], _| {
                for out_sample in output.iter_mut() {
                    *out_sample = *samples.get(pos).unwrap_or(&0);
                    pos += 1;
                }
            },
            |err| eprintln!("Audio-Fehler: {}", err),
            None,
        )
        .expect("Stream konnte nicht gebaut werden");

    // 4. Abspielen starten und 10 Sekunden laufen lassen
    stream.play().expect("Wiedergabe fehlgeschlagen");
    println!(
        "Spiele 'Track 1.wav' ({} Hz, {} Kanäle)...",
        spec.sample_rate, spec.channels
    );
    std::thread::sleep(Duration::from_secs(10));
}
