//! Демонстрация продвинутых возможностей 🚀
//!
//! Этот пример показывает:
//! - Работу с низкоуровневым API
//! - Создание собственных артефактов сжатия
//! - Анализ внутренних структур данных
//! - Оптимизацию для специфических случаев

use arithmetic_compression_wizard::prelude::*;
use arithmetic_compression_wizard::statistics::analyze_compression;

fn main() {
    println!("🚀 Демонстрация продвинутых возможностей");
    println!("═══════════════════════════════════════════");

    // Демонстрация работы с артефактами
    demo_artifact_inspection();

    // Анализ словаря
    demo_dictionary_analysis();

    // Сравнение с теоретическими пределами
    demo_theoretical_limits();

    // Оптимизация для специфических данных
    demo_data_optimization();

    // Стресс-тест больших данных
    demo_stress_test();

    println!("✨ Демонстрация продвинутых возможностей завершена!");
}

/// Инспекция внутренней структуры артефактов
fn demo_artifact_inspection() {
    println!("\n🔍 Инспекция артефактов сжатия");
    println!("─────────────────────────────────");

    let sample_text = b"the quick brown fox jumps over the lazy dog. \
                       the quick brown fox jumps over the lazy dog. \
                       the quick brown fox jumps over the lazy dog.";

    // Создаем артефакт с помощью низкоуровневого API
    let artifact = weave_compression_spell(sample_text);

    println!("📦 Структура артефакта:");
    println!("   Исходный размер:      {} байт", sample_text.len());
    println!(
        "   Битовый поток:        {} байт",
        artifact.compressed_bit_stream.len()
    );
    println!(
        "   Таблица частот:       {} записей",
        artifact.mystical_frequency_codex.len()
    );
    println!(
        "   Словарь слов:         {} слов",
        artifact.mystical_word_grimoire.len()
    );
    println!(
        "   Общее количество:     {} символов",
        artifact.total_frequency_essence
    );

    // Анализируем таблицу частот
    println!("\n📊 Топ-5 символов по частоте:");
    let mut freq_sorted = artifact.mystical_frequency_codex.clone();
    freq_sorted.sort_by_key(|(_, freq, _)| std::cmp::Reverse(*freq));

    for (symbol, freq, start) in freq_sorted.iter().take(5) {
        if *symbol < 256 {
            let ch = *symbol as u8 as char;
            if ch.is_ascii_graphic() || ch == ' ' {
                println!("   '{}': {} раз (позиция {})", ch, freq, start);
            } else {
                println!("   [{}]: {} раз (позиция {})", symbol, freq, start);
            }
        } else {
            let word_idx = (*symbol - 256) as usize;
            if let Some(word) = artifact.mystical_word_grimoire.get(word_idx) {
                println!("   \"{}\": {} раз (позиция {})", word, freq, start);
            }
        }
    }

    // Проверяем восстановление
    let restored = unweave_compression_spell(artifact);
    let is_correct = sample_text == restored.as_slice();
    println!("\n✅ Корректность восстановления: {}", is_correct);
    println!();
}

/// Анализ работы словаря
fn demo_dictionary_analysis() {
    println!("📚 Анализ работы словаря");
    println!("─────────────────────");

    // Текст с разной степенью повторяемости
    let texts = vec![
        ("Без повторов", "abcdefghijklmnopqrstuvwxyz"),
        ("Мало повторов", "hello world how are you today"),
        (
            "Средние повторы",
            "the cat sat on the mat and the dog ran to the cat",
        ),
        (
            "Много повторов",
            "the the the quick quick brown fox fox jumps over the lazy dog dog",
        ),
    ];

    println!("Тип текста       | Размер | Словарь | Эффект | Коэффициент");
    println!("──────────────── |────────|─────────|────────|────────────");

    for (name, text) in texts {
        let bytes = text.as_bytes();
        let artifact = weave_compression_spell(bytes);

        let dict_size = artifact.mystical_word_grimoire.len();
        let dict_effect = if dict_size > 0 { "🟢" } else { "🔴" };

        let total_compressed = estimate_total_size(&artifact);
        let ratio = (1.0 - total_compressed as f64 / bytes.len() as f64) * 100.0;

        println!(
            "{:<16} | {:>6} | {:>7} | {:>6} | {:>9.1}%",
            name,
            bytes.len(),
            dict_size,
            dict_effect,
            ratio
        );
    }

    println!("\n💡 Наблюдения:");
    println!("   🟢 = словарь используется эффективно");
    println!("   🔴 = словарь не создан или неэффективен");
    println!();
}

/// Сравнение с теоретическими пределами Шеннона
fn demo_theoretical_limits() {
    println!("🧮 Сравнение с теоретическими пределами");
    println!("────────────────────────────────────────");

    let test_data = "Rust является системным языком программирования, \
                     который обеспечивает безопасность памяти и \
                     высокую производительность. Rust предотвращает \
                     множество ошибок времени выполнения."
        .repeat(3);

    let bytes = test_data.as_bytes();
    let analysis = analyze_compression(bytes);

    // Теоретический минимум по Шеннону
    let shannon_minimum = (bytes.len() as f64 * analysis.shannon_entropy / 8.0).ceil() as usize;

    // Наш результат
    let our_result = analysis.compressed_size;

    // Эффективность относительно теории
    let efficiency = shannon_minimum as f64 / our_result as f64 * 100.0;

    println!("📏 Анализ эффективности:");
    println!("   Исходный размер:        {} байт", bytes.len());
    println!(
        "   Энтропия Шеннона:       {:.3} бит/символ",
        analysis.shannon_entropy
    );
    println!("   Теоретический минимум:  {} байт", shannon_minimum);
    println!("   Наш результат:          {} байт", our_result);
    println!("   Эффективность:          {:.1}%", efficiency);

    // Анализ потерь
    let overhead = our_result as i32 - shannon_minimum as i32;
    println!("   Накладные расходы:      {} байт", overhead);

    if efficiency > 90.0 {
        println!("   🎉 Отличный результат!");
    } else if efficiency > 70.0 {
        println!("   👍 Хороший результат");
    } else {
        println!("   📈 Есть место для улучшений");
    }

    println!();
}

/// Оптимизация для специфических типов данных
fn demo_data_optimization() {
    println!("⚙️ Оптимизация для разных типов данных");
    println!("──────────────────────────────────────");

    // Разные типы данных для тестирования
    let data_types = vec![
        ("Код программы", generate_code_sample()),
        ("JSON данные", generate_json_sample()),
        ("Логи сервера", generate_log_sample()),
        ("Двоичные данные", generate_binary_sample()),
    ];

    for (name, data) in data_types {
        let bytes = data.as_bytes();
        let analysis = analyze_compression(bytes);

        println!("📋 {}: ", name);
        println!("   Размер:           {} байт", bytes.len());
        println!(
            "   Энтропия:         {:.2} бит/символ",
            analysis.shannon_entropy
        );
        println!("   Сжатый размер:    {} байт", analysis.compressed_size);
        println!("   Коэффициент:      {:.1}%", analysis.compression_ratio);
        println!(
            "   Словарь:          {} слов",
            analysis.word_dictionary_size
        );

        // Рекомендации по оптимизации
        provide_optimization_tips(&analysis, name);
        println!();
    }
}

/// Стресс-тест для больших объемов данных
fn demo_stress_test() {
    println!("💪 Стресс-тест производительности");
    println!("──────────────────────────────");

    let sizes = vec![1_000, 10_000, 100_000, 500_000];

    println!("Размер    | Время сжатия | Время восст. | Коэффициент | Скорость");
    println!("─────────|──────────────|──────────────|─────────────|─────────");

    for size in sizes {
        let test_data = generate_test_data(size);
        let bytes = test_data.as_bytes();

        // Измеряем сжатие
        let start = std::time::Instant::now();
        let compressed = compress_data(bytes);
        let compression_time = start.elapsed();

        // Измеряем восстановление
        let start = std::time::Instant::now();
        let restored = decompress_data(compressed.clone());
        let decompression_time = start.elapsed();

        // Проверяем корректность
        assert_eq!(bytes, restored.as_slice());

        let ratio = (1.0 - compressed.len() as f64 / bytes.len() as f64) * 100.0;
        let speed = bytes.len() as f64 / compression_time.as_secs_f64() / 1_000_000.0;

        println!(
            "{:>8} | {:>11.2}ms | {:>11.2}ms | {:>9.1}% | {:>6.1} МБ/с",
            format_size(size),
            compression_time.as_millis(),
            decompression_time.as_millis(),
            ratio,
            speed
        );
    }

    println!("\n💡 Результаты показывают масштабируемость алгоритма");
    println!();
}

/// Вспомогательные функции

fn estimate_total_size(artifact: &CompressionArtifact) -> usize {
    let dict_size: usize = artifact
        .mystical_word_grimoire
        .iter()
        .map(|w| w.len() + 4)
        .sum();
    let freq_table_size = artifact.mystical_frequency_codex.len() * 20; // Примерная оценка
    dict_size + freq_table_size + artifact.compressed_bit_stream.len() + 8
}

fn provide_optimization_tips(
    analysis: &arithmetic_compression_wizard::statistics::CompressionAnalysis,
    data_type: &str,
) {
    match data_type {
        "Код программы" => {
            if analysis.word_dictionary_size > 10 {
                println!("   💡 Много ключевых слов - отличный кандидат для сжатия");
            }
        }
        "JSON данные" => {
            if analysis.compression_ratio > 50.0 {
                println!("   💡 JSON хорошо сжимается из-за повторяющихся ключей");
            }
        }
        "Логи сервера" => {
            if analysis.shannon_entropy < 4.0 {
                println!("   💡 Низкая энтропия - типично для логов");
            }
        }
        "Двоичные данные" => {
            if analysis.compression_ratio < 10.0 {
                println!("   ⚠️ Двоичные данные сжимаются плохо");
            }
        }
        _ => {}
    }
}

fn generate_code_sample() -> String {
    "fn main() {\n    println!(\"Hello, world!\");\n    let x = 42;\n    let y = x + 1;\n}"
        .repeat(20)
}

fn generate_json_sample() -> String {
    r#"{"id": 1, "name": "user", "email": "test@example.com", "active": true}"#.repeat(50)
}

fn generate_log_sample() -> String {
    "2023-01-01 10:00:00 INFO Server started on port 8080\n".repeat(30)
        + "2023-01-01 10:00:01 INFO New connection from 192.168.1.1\n"
        + "2023-01-01 10:00:02 ERROR Database connection failed\n"
}

fn generate_binary_sample() -> String {
    (0..=255u8).cycle().take(1000).map(|b| b as char).collect()
}

fn generate_test_data(size: usize) -> String {
    let base = "Rust programming language systems safety performance ";
    base.repeat(size / base.len() + 1)[..size].to_string()
}

fn format_size(size: usize) -> String {
    if size >= 1_000_000 {
        format!("{:.1}M", size as f64 / 1_000_000.0)
    } else if size >= 1_000 {
        format!("{}K", size / 1_000)
    } else {
        size.to_string()
    }
}
