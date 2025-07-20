//! Быстрый старт с арифметическим сжатием 🚀
//!
//! Самый простой пример для начала работы с библиотекой.
//! Показывает основные операции за 30 секунд.

use arithmetic_compression_wizard::simple_api::{compress_data, decompress_data};

fn main() {
    println!("🚀 Быстрый старт с арифметическим сжатием\n");

    // 1. Простое сжатие текста
    let original = "Привет, мир! Это демонстрация сжатия на Rust.".as_bytes();

    println!(
        "📝 Исходный текст: \"{}\"",
        String::from_utf8_lossy(original)
    );
    println!("   Размер: {} байт", original.len());

    // 2. Сжимаем данные
    let compressed = compress_data(original);
    println!("\n📦 Сжато в {} байт", compressed.len());

    // 3. Восстанавливаем данные
    let restored = decompress_data(compressed.clone());
    println!("🔄 Восстановлено {} байт", restored.len());

    // 4. Проверяем корректность
    if original == restored.as_slice() {
        println!("✅ Данные восстановлены без потерь!");
    } else {
        println!("❌ Ошибка восстановления!");
    }

    // 5. Показываем эффективность
    let ratio = if original.len() > 0 {
        (1.0 - compressed.len() as f64 / original.len() as f64) * 100.0
    } else {
        0.0
    };

    println!("\n📊 Результат:");
    if ratio > 0.0 {
        println!("   Сжатие: {:.1}% экономии", ratio);
    } else {
        println!("   Данные увеличились на {:.1}% (малый размер)", -ratio);
    }

    println!("\n🎯 Попробуйте другие примеры:");
    println!("   cargo run --example basic_compression_demo");
    println!("   cargo run --example interactive_demo");
    println!("   cargo run --example file_compression_demo");
    println!("   cargo run --example advanced_features_demo");
}
