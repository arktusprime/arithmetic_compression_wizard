//! Демонстрация сжатия файлов 📁
//!
//! Этот пример показывает:
//! - Сжатие и восстановление файлов
//! - Работу с большими объемами данных
//! - Сравнение с исходными файлами
//! - Измерение производительности

use arithmetic_compression_wizard::prelude::*;
use arithmetic_compression_wizard::statistics::analyze_compression;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

fn main() -> io::Result<()> {
    println!("📁 Демонстрация сжатия файлов\n");

    // Создаем тестовые файлы
    create_test_files()?;

    // Демонстрируем сжатие разных типов файлов
    demo_text_file()?;
    demo_structured_data()?;
    demo_performance_test()?;

    println!("🧹 Очистка временных файлов...");
    cleanup_test_files()?;

    println!("✨ Демонстрация завершена!");
    Ok(())
}

/// Создает тестовые файлы для демонстрации
fn create_test_files() -> io::Result<()> {
    println!("📝 Создание тестовых файлов...");

    // Создаем простой текстовый файл
    let text_content = "Rust - это системный язык программирования, \
                       который работает невероятно быстро, \
                       предотвращает ошибки сегментирования и \
                       гарантирует потокобезопасность. \
                       Rust обеспечивает нулевую стоимость абстракций, \
                       перемещающую семантику, гарантированную безопасность памяти, \
                       потоки без гонок данных, обобщения на основе типажей, \
                       сопоставление с образцом, вывод типов, \
                       минимальную среду выполнения и эффективные C-привязки.";

    fs::write("test_text.txt", text_content.repeat(50))?;

    // Создаем структурированные данные (JSON-подобные)
    let mut structured_data = String::new();
    for i in 0..100 {
        structured_data.push_str(&format!(
            "{{\"id\": {}, \"name\": \"user_{}\", \"email\": \"user_{}@example.com\", \"active\": true}}\n",
            i, i, i
        ));
    }
    fs::write("test_data.json", structured_data)?;

    // Создаем файл с повторяющимися паттернами
    let pattern = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".repeat(200);
    fs::write("test_pattern.txt", pattern)?;

    println!("   ✅ Тестовые файлы созданы\n");
    Ok(())
}

/// Демонстрация сжатия текстового файла
fn demo_text_file() -> io::Result<()> {
    println!("📖 Сжатие текстового файла");

    let filename = "test_text.txt";
    let original_data = fs::read(filename)?;

    println!("   Файл: {}", filename);
    println!("   Размер: {} байт", original_data.len());

    // Измеряем время сжатия
    let start = Instant::now();
    let compressed = compress_data(&original_data);
    let compression_time = start.elapsed();

    // Измеряем время декомпрессии
    let start = Instant::now();
    let restored = decompress_data(compressed.clone());
    let decompression_time = start.elapsed();

    // Проверяем корректность
    assert_eq!(original_data, restored);

    // Сохраняем сжатый файл
    let compressed_filename = "test_text.txt.compressed";
    fs::write(compressed_filename, &compressed)?;

    println!("   Сжатый размер: {} байт", compressed.len());
    println!(
        "   Коэффициент: {:.1}%",
        (1.0 - compressed.len() as f64 / original_data.len() as f64) * 100.0
    );
    println!("   Время сжатия: {:?}", compression_time);
    println!("   Время восстановления: {:?}", decompression_time);
    println!("   ✅ Файл сжат и восстановлен корректно\n");

    Ok(())
}

/// Демонстрация сжатия структурированных данных
fn demo_structured_data() -> io::Result<()> {
    println!("🗂️ Сжатие структурированных данных");

    let filename = "test_data.json";
    let original_data = fs::read(filename)?;

    println!("   Файл: {}", filename);
    println!("   Размер: {} байт", original_data.len());

    // Анализируем данные
    let analysis = analyze_compression(&original_data);

    println!(
        "   Энтропия Шеннона: {:.2} бит/символ",
        analysis.shannon_entropy
    );
    println!(
        "   Плотность сжатия: {:.2} бит/символ",
        analysis.compression_density
    );
    println!("   Слов в словаре: {}", analysis.word_dictionary_size);

    // Сжимаем
    let compressed = compress_data(&original_data);
    let restored = decompress_data(compressed.clone());

    assert_eq!(original_data, restored);

    // Сохраняем результат
    fs::write("test_data.json.compressed", &compressed)?;

    println!("   Сжатый размер: {} байт", compressed.len());
    println!(
        "   Коэффициент: {:.1}%",
        (1.0 - compressed.len() as f64 / original_data.len() as f64) * 100.0
    );
    println!("   ✅ Структурированные данные обработаны\n");

    Ok(())
}

/// Тест производительности на больших данных
fn demo_performance_test() -> io::Result<()> {
    println!("⚡ Тест производительности");

    let filename = "test_pattern.txt";
    let original_data = fs::read(filename)?;

    println!("   Файл: {}", filename);
    println!("   Размер: {} байт", original_data.len());

    // Многократное сжатие для измерения средней производительности
    let iterations = 5;
    let mut total_compression_time = std::time::Duration::new(0, 0);
    let mut total_decompression_time = std::time::Duration::new(0, 0);
    let mut compressed_size = 0;

    print!("   Выполняется {} итераций", iterations);
    io::stdout().flush()?;

    for _i in 0..iterations {
        print!(".");
        io::stdout().flush()?;

        // Сжатие
        let start = Instant::now();
        let compressed = compress_data(&original_data);
        total_compression_time += start.elapsed();
        compressed_size = compressed.len();

        // Декомпрессия
        let start = Instant::now();
        let restored = decompress_data(compressed);
        total_decompression_time += start.elapsed();

        // Проверка корректности
        assert_eq!(original_data, restored);
    }

    println!();

    let avg_compression = total_compression_time / iterations;
    let avg_decompression = total_decompression_time / iterations;

    println!("   Среднее время сжатия: {:?}", avg_compression);
    println!("   Среднее время восстановления: {:?}", avg_decompression);
    println!(
        "   Скорость сжатия: {:.2} МБ/с",
        (original_data.len() as f64 / 1_000_000.0) / avg_compression.as_secs_f64()
    );
    println!(
        "   Скорость восстановления: {:.2} МБ/с",
        (original_data.len() as f64 / 1_000_000.0) / avg_decompression.as_secs_f64()
    );
    println!(
        "   Итоговый коэффициент: {:.1}%",
        (1.0 - compressed_size as f64 / original_data.len() as f64) * 100.0
    );
    println!("   ✅ Тест производительности завершен\n");

    Ok(())
}

/// Очистка временных файлов
fn cleanup_test_files() -> io::Result<()> {
    let files_to_remove = [
        "test_text.txt",
        "test_text.txt.compressed",
        "test_data.json",
        "test_data.json.compressed",
        "test_pattern.txt",
    ];

    for file in &files_to_remove {
        if let Err(_) = fs::remove_file(file) {
            // Игнорируем ошибки удаления
        }
    }

    Ok(())
}
