//! Мастер арифметического сжатия 🧙‍♂️
//! Демонстрация алгоритмического программирования на Rust

use std::collections::HashMap;
use std::fs;

// Импорт библиотеки сжатия
use arithmetic_compression_wizard::prelude::*;

/// Вычисляет энтропию Шеннона - теоретический предел сжатия 🎯
/// H(X) = -Σ p(x) * log₂(p(x)) - минимум битов на символ
fn calculate_shannon_entropy_wisdom(mystical_bytes: &[u8]) -> f64 {
    // HashMap для O(1) подсчета частот
    let mut frequency_grimoire = HashMap::new();

    // Подсчет частот всех байтов
    for &enchanted_byte in mystical_bytes {
        *frequency_grimoire.entry(enchanted_byte).or_insert(0u64) += 1;
    }

    let total_symbols = mystical_bytes.len() as f64;
    if total_symbols == 0.0 {
        return 0.0;
    }

    let mut entropy_accumulator = 0.0;

    for symbol_count in frequency_grimoire.values() {
        let probability = (*symbol_count as f64) / total_symbols;
        entropy_accumulator -= probability * probability.log2();
    }

    entropy_accumulator
}

/// Демонстрация сжатия с детальной статистикой 🎭
/// Использует заимствование для избежания копирования
fn perform_compression_spectacle(original_manuscript: &[u8], performance_title: &str) {
    let enchanted_result = weave_compression_spell(original_manuscript);
    let restored_manuscript = unweave_compression_spell(enchanted_result.clone());

    if original_manuscript == restored_manuscript.as_slice() {
        println!("✅ Успешное восстановление ({}).", performance_title);
    } else {
        println!("❌ Ошибка! Повреждение данных ({}).", performance_title);
    }

    // Безопасная проверка границ
    let preview_length = std::cmp::min(10, original_manuscript.len());

    // Предварительный просмотр для отладки
    print!("📜 Исходные байты ({}): ", performance_title);
    for i in 0..preview_length {
        print!("{:3} ", original_manuscript[i]);
    }
    println!();

    print!("🎯 Восстановленные байты ({}): ", performance_title);
    for i in 0..preview_length {
        print!("{:3} ", restored_manuscript[i]);
    }
    println!();

    // Методы на примитивах
    let theoretical_entropy = calculate_shannon_entropy_wisdom(original_manuscript);

    // Статистика сжатия с приведением типов
    let metadata_overhead = enchanted_result.mystical_word_grimoire.len() * 17; // Оценка накладных расходов
    let total_compressed_size = metadata_overhead + enchanted_result.compressed_bit_stream.len();

    println!(
        "📊 Исходный размер ({}): {} байт",
        performance_title,
        original_manuscript.len()
    );
    println!(
        "📦 Сжатый размер ({}): {} байт",
        performance_title, total_compressed_size
    );
    println!(
        "🧮 Энтропия Шеннона ({}): {:.2} бит/символ",
        performance_title, theoretical_entropy
    );
    println!(
        "⚡ Плотность сжатия ({}): {:.2} бит/символ",
        performance_title,
        total_compressed_size as f64 * 8.0 / original_manuscript.len() as f64
    );

    // Точная арифметика с приведением типов
    let compression_efficiency =
        (1.0 - total_compressed_size as f64 / original_manuscript.len() as f64) * 100.0;
    println!(
        "🎯 Эффективность сжатия ({}): {:.2}%\n",
        performance_title, compression_efficiency
    );
    println!();
}

/// Точка входа с обработкой ошибок через Result 🎯
fn main() {
    println!("🧙‍♂️ Добро пожаловать в мастерскую арифметического сжатия!");
    println!("🦀 Демонстрация силы Rust в системном программировании\n");

    // Тест 1: Простые данные с литералами b""
    perform_compression_spectacle(b"AB", "Простые данные");

    // Тест 2: Повторяющиеся паттерны
    perform_compression_spectacle(b"ABCAABACLLDLLMLLCABA", "Паттерны");

    // Тест 3: Реальный текст с обработкой ошибок через Result
    let hamlet_manuscript = match fs::read("src/Shakespeare William. Hamlet Prince of Denmark.txt")
    {
        Ok(manuscript_data) => manuscript_data,
        Err(reading_curse) => {
            // Форматированный вывод ошибок
            eprintln!("📚 Не удалось прочитать Гамлета: {}", reading_curse);
            return; // Ранний возврат
        }
    };

    perform_compression_spectacle(&hamlet_manuscript, "Шекспир");

    println!("🎭 Демонстрация завершена! Спасибо за внимание к магии Rust! 🦀");
}
