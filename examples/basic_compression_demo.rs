//! Базовая демонстрация арифметического сжатия 🎯
//!
//! Этот пример показывает основные возможности библиотеки:
//! - Простое сжатие и восстановление данных
//! - Анализ эффективности сжатия
//! - Работу с различными типами данных

use arithmetic_compression_wizard::prelude::*;
use arithmetic_compression_wizard::statistics::analyze_compression;

fn main() {
    println!("🧙‍♂️ Демонстрация арифметического сжатия\n");

    // Пример 1: Простой текст
    demo_text_compression();

    // Пример 2: Повторяющиеся данные
    demo_repetitive_data();

    // Пример 3: Бинарные данные
    demo_binary_data();

    // Пример 4: Реальный текст с анализом
    demo_analysis();

    println!("✨ Демонстрация завершена!");
}

/// Сжатие простого текста
fn demo_text_compression() {
    println!("📝 Пример 1: Простой текст");

    let original_text = b"Hello, world! This is a demonstration of arithmetic compression in Rust.";

    // Сжимаем
    let compressed = compress_data(original_text);

    // Восстанавливаем
    let restored = decompress_data(compressed.clone());

    // Проверяем корректность
    assert_eq!(original_text.as_slice(), restored.as_slice());

    println!("   Исходный размер: {} байт", original_text.len());
    println!("   Сжатый размер:   {} байт", compressed.len());
    println!(
        "   Коэффициент:     {:.1}%",
        (1.0 - compressed.len() as f64 / original_text.len() as f64) * 100.0
    );
    println!("   ✅ Данные восстановлены корректно\n");
}

/// Сжатие повторяющихся данных
fn demo_repetitive_data() {
    println!("🔄 Пример 2: Повторяющиеся данные");

    let repetitive_data = b"aaaaaabbbbbbccccccddddddeeeeeeffffffgggggg";

    let compressed = compress_data(repetitive_data);
    let restored = decompress_data(compressed.clone());

    assert_eq!(repetitive_data.as_slice(), restored.as_slice());

    println!("   Исходный размер: {} байт", repetitive_data.len());
    println!("   Сжатый размер:   {} байт", compressed.len());
    println!(
        "   Коэффициент:     {:.1}%",
        (1.0 - compressed.len() as f64 / repetitive_data.len() as f64) * 100.0
    );
    println!("   ✅ Отличное сжатие повторяющихся данных\n");
}

/// Сжатие бинарных данных
fn demo_binary_data() {
    println!("📦 Пример 3: Бинарные данные");

    let binary_data: Vec<u8> = (0..=255).cycle().take(100).collect();

    let compressed = compress_data(&binary_data);
    let restored = decompress_data(compressed.clone());

    assert_eq!(binary_data.as_slice(), restored.as_slice());

    println!("   Исходный размер: {} байт", binary_data.len());
    println!("   Сжатый размер:   {} байт", compressed.len());
    println!(
        "   Коэффициент:     {:.1}%",
        (1.0 - compressed.len() as f64 / binary_data.len() as f64) * 100.0
    );
    println!("   ✅ Бинарные данные обработаны корректно\n");
}

/// Подробный анализ сжатия
fn demo_analysis() {
    println!("📊 Пример 4: Анализ эффективности");

    let sample_text = b"the quick brown fox jumps over the lazy dog. \
                       the quick brown fox jumps over the lazy dog. \
                       the quick brown fox jumps over the lazy dog. \
                       the quick brown fox jumps over the lazy dog. \
                       the quick brown fox jumps over the lazy dog.";

    let analysis = analyze_compression(sample_text);

    println!("   Исходный размер:     {} байт", analysis.original_size);
    println!("   Сжатый размер:       {} байт", analysis.compressed_size);
    println!("   Коэффициент сжатия:  {:.1}%", analysis.compression_ratio);
    println!(
        "   Энтропия Шеннона:    {:.2} бит/символ",
        analysis.shannon_entropy
    );
    println!(
        "   Плотность сжатия:    {:.2} бит/символ",
        analysis.compression_density
    );
    println!("   Слов в словаре:      {}", analysis.word_dictionary_size);

    println!("   Топ символов:");
    for (symbol, freq) in analysis.top_symbols.iter().take(5) {
        if *symbol < 256 {
            println!("     '{}': {} раз", *symbol as u8 as char, freq);
        } else {
            println!("     [слово {}]: {} раз", symbol - 256, freq);
        }
    }

    // Проверяем корректность
    let compressed = compress_data(sample_text);
    let restored = decompress_data(compressed);
    assert_eq!(sample_text.as_slice(), restored.as_slice());
    println!("   ✅ Анализ завершен, данные корректны\n");
}
