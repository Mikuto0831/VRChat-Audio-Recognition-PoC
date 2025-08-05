use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    // 1. オーディオホストの初期化
    // WindowsではWASAPIがデフォルトホストになります。
    let host = cpal::default_host();

    // 2. デフォルトのオーディオ出力デバイスを取得
    // VRChatの音声が再生されるデバイス（通常はシステムのデフォルトスピーカー）を選択します。
    // cpalでは、出力デバイスのループバックをキャプチャすることでシステム音声を録音します。
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No default output device available"))?;

    println!("Capturing from device: {}", device.name()?);

    // 3. サポートされているストリーム設定を取得
    // デバイスがサポートする形式の中から、Python側で扱いやすいものを探します。
    // 一般的に、16kHz, モノラル, 16bit整数(i16)が音声認識でよく使われます。
    let mut supported_configs_range = device.supported_output_configs()?;
    let config = supported_configs_range
        .find(|c| c.sample_format() == cpal::SampleFormat::F32 && c.channels() >= 1)
        .ok_or_else(|| anyhow::anyhow!("No supported F32 config found"))?
        .with_max_sample_rate();

    println!("Using config: {:?}", config);

    // 音声レベル監視用のカウンター
    let sample_count = Arc::new(AtomicUsize::new(0));
    let sample_count_clone = sample_count.clone();
    let last_print = Arc::new(std::sync::Mutex::new(Instant::now()));
    let last_print_clone = last_print.clone();

    // 4. オーディオストリームの構築
    let stream = device.build_input_stream(
        &config.config(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // このクロージャは、オーディオバッファが満たされるたびに呼び出されます。
            // `data`はf32形式（-1.0から1.0）のサンプル配列です。

            // 音声レベルの計算（RMS - Root Mean Square）
            let rms = if !data.is_empty() {
                let sum_squares: f32 = data.iter().map(|&x| x * x).sum();
                (sum_squares / data.len() as f32).sqrt()
            } else {
                0.0
            };

            // サンプル数をカウント
            sample_count_clone.fetch_add(data.len(), Ordering::Relaxed);

            // 1秒ごとに音声レベルを表示
            let mut last_print_time = last_print_clone.lock().unwrap();
            if last_print_time.elapsed() >= Duration::from_secs(1) {
                let total_samples = sample_count_clone.load(Ordering::Relaxed);
                let amplitude_percent = (rms * 100.0).min(100.0);

                // 音声レベルを視覚的に表示
                let bar_length = (amplitude_percent / 5.0) as usize; // 5%ごとに1文字
                let bar = "█".repeat(bar_length);

                println!(
                    "音声レベル: {:6.2}% |{:<20}| サンプル数: {}",
                    amplitude_percent, bar, total_samples
                );

                *last_print_time = Instant::now();
            }

            // TODO: 将来的にはここでPythonプロセスにデータを送信
            // 現在はPoC段階なので、音声が正常にキャプチャできていることを確認するだけ
        },
        |err| {
            // ストリームでエラーが発生した場合の処理
            eprintln!("An error occurred on the audio stream: {}", err);
        },
        None, // タイムアウトなし
    )?;

    // 5. ストリームの再生（キャプチャ開始）
    stream.play()?;

    println!("Audio capture started. Press Enter to stop.");
    println!("音声レベルモニタリング開始...");
    println!("VRChatや他のアプリケーションで音声を再生すると、レベルが表示されます。");
    println!("終了するにはEnterキーを押してください...");

    // Enterキーの入力を待機
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    println!("音声キャプチャを停止しています...");
    drop(stream); // ストリームを明示的に停止
    println!("停止完了。");
    
    Ok(())
}
