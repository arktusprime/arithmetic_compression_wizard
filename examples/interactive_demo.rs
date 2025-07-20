//! Интерактивная демонстрация сжатия 🎮
//!
//! Этот пример позволяет пользователю:
//! - Вводить собственный текст для сжатия
//! - Сравнивать разные алгоритмы
//! - Экспериментировать с параметрами
//! - Анализировать результаты в реальном времени

use arithmetic_compression_wizard::prelude::*;
use arithmetic_compression_wizard::statistics::analyze_compression;
use std::io::{self, Write};

fn main() {
    println!("🎮 Интерактивная демонстрация арифметического сжатия");
    println!("═══════════════════════════════════════════════════════");
    println!();

    loop {
        show_menu();

        let choice = get_user_input("Выберите опцию (1-5): ");

        match choice.trim() {
            "1" => demo_custom_text(),
            "2" => demo_text_comparison(),
            "3" => demo_entropy_analysis(),
            "4" => demo_compression_limits(),
            "5" => {
                println!("👋 До свидания!");
                break;
            }
            _ => println!("❌ Неверный выбор. Попробуйте снова.\n"),
        }
    }
}

/// Показывает главное меню
fn show_menu() {
    println!("🔧 Доступные опции:");
    println!("  1️⃣  Сжать свой текст");
    println!("  2️⃣  Сравнение типов текста");
    println!("  3️⃣  Анализ энтропии");
    println!("  4️⃣  Пределы сжатия");
    println!("  5️⃣  Выход");
    println!();
}

/// Получает ввод от пользователя
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

/// Демонстрация сжатия пользовательского текста
fn demo_custom_text() {
    println!("\n📝 Сжатие пользовательского текста");
    println!("─────────────────────────────────────");

    let text = get_user_input("Введите текст для сжатия: ");
    let text_bytes = text.trim().as_bytes();

    if text_bytes.is_empty() {
        println!("❌ Пустой текст. Возврат в меню.\n");
        return;
    }

    println!("\n🔄 Выполняется сжатие...");

    // Сжимаем текст
    let compressed = compress_data(text_bytes);
    let restored = decompress_data(compressed.clone());

    // Проверяем корректность
    let is_correct = text_bytes == restored.as_slice();

    // Выводим результаты
    println!("\n📊 Результаты:");
    println!("   Исходный размер:    {} байт", text_bytes.len());
    println!("   Сжатый размер:      {} байт", compressed.len());

    if text_bytes.len() > 0 {
        let ratio = (1.0 - compressed.len() as f64 / text_bytes.len() as f64) * 100.0;
        println!("   Коэффициент сжатия: {:.1}%", ratio);

        if ratio > 0.0 {
            println!("   🎉 Сжатие эффективно!");
        } else {
            println!("   📈 Данные увеличились (накладные расходы)");
        }
    }

    println!(
        "   Корректность:       {}",
        if is_correct { "✅" } else { "❌" }
    );

    // Показываем детали, если текст достаточно длинный
    if text_bytes.len() > 20 {
        let analysis = analyze_compression(text_bytes);
        println!("\n🔍 Детальный анализ:");
        println!(
            "   Энтропия Шеннона:  {:.2} бит/символ",
            analysis.shannon_entropy
        );
        println!(
            "   Плотность сжатия:  {:.2} бит/символ",
            analysis.compression_density
        );
        println!("   Слов в словаре:    {}", analysis.word_dictionary_size);

        if analysis.word_dictionary_size > 0 {
            println!("   💡 Найдены повторяющиеся слова - хорошо для сжатия!");
        }
    }

    println!();
}

/// Сравнение разных типов текста
fn demo_text_comparison() {
    println!("\n📊 Сравнение типов текста");
    println!("───────────────────────────");

    let test_cases = vec![
        (
            "Повторяющийся",
            "aaaaaabbbbbbccccccddddddeeeeee".to_string(),
        ),
        ("Случайный", generate_random_text(30)),
        (
            "Структурированный",
            r#"{"name":"user","id":123,"active":true}"#.repeat(3),
        ),
        (
            "Естественный",
            "Rust - отличный язык программирования для системного уровня.".to_string(),
        ),
    ];

    println!("Тип текста        | Размер | Сжато | Коэффициент | Энтропия");
    println!("─────────────────|────────|───────|─────────────|─────────");

    for (name, text) in test_cases {
        let text_bytes = text.as_bytes();
        let compressed = compress_data(text_bytes);
        let analysis = analyze_compression(text_bytes);

        let ratio = if text_bytes.len() > 0 {
            (1.0 - compressed.len() as f64 / text_bytes.len() as f64) * 100.0
        } else {
            0.0
        };

        println!(
            "{:<17}| {:>6} | {:>5} | {:>9.1}% | {:>7.2}",
            name,
            text_bytes.len(),
            compressed.len(),
            ratio,
            analysis.shannon_entropy
        );
    }

    println!("\n💡 Наблюдения:");
    println!("   • Повторяющиеся данные сжимаются лучше всего");
    println!("   • Случайные данные практически не сжимаются");
    println!("   • Структурированные данные показывают средние результаты");
    println!("   • Естественный язык обычно хорошо сжимается\n");
}

/// Анализ энтропии разных текстов
fn demo_entropy_analysis() {
    println!("\n🧮 Анализ энтропии Шеннона");
    println!("────────────────────────────");

    let text = get_user_input("Введите текст для анализа энтропии: ");
    let text_bytes = text.trim().as_bytes();

    if text_bytes.is_empty() {
        println!("❌ Пустой текст. Возврат в меню.\n");
        return;
    }

    let analysis = analyze_compression(text_bytes);

    println!("\n📈 Результаты анализа:");
    println!("   Размер текста:      {} байт", text_bytes.len());
    println!(
        "   Энтропия Шеннона:   {:.3} бит/символ",
        analysis.shannon_entropy
    );
    println!(
        "   Теоретический минимум: {:.1} байт",
        text_bytes.len() as f64 * analysis.shannon_entropy / 8.0
    );

    // Классификация по энтропии
    let entropy_class = match analysis.shannon_entropy {
        e if e < 2.0 => "🔵 Очень низкая (отлично сжимается)",
        e if e < 4.0 => "🟢 Низкая (хорошо сжимается)",
        e if e < 6.0 => "🟡 Средняя (умеренно сжимается)",
        e if e < 7.0 => "🟠 Высокая (плохо сжимается)",
        _ => "🔴 Очень высокая (почти не сжимается)",
    };

    println!("   Классификация:      {}", entropy_class);

    // Показываем топ символов
    println!("\n🔤 Наиболее частые символы:");
    for (symbol, freq) in analysis.top_symbols.iter().take(5) {
        if *symbol < 256 {
            let ch = *symbol as u8 as char;
            if ch.is_ascii_graphic() || ch == ' ' {
                println!("     '{}': {} раз", ch, freq);
            } else {
                println!("     [{}]: {} раз", symbol, freq);
            }
        } else {
            println!("     [слово {}]: {} раз", symbol - 256, freq);
        }
    }

    println!();
}

/// Демонстрация пределов сжатия
fn demo_compression_limits() {
    println!("\n🎯 Пределы сжатия");
    println!("──────────────────");

    println!("Тестируем разные размеры данных:\n");

    let sizes = vec![10, 50, 100, 500, 1000];

    for size in sizes {
        // Создаем текст с повторениями (хорошо сжимается)
        let repetitive_text = "Rust ".repeat(size / 5);
        let rep_bytes = repetitive_text.as_bytes();
        let rep_compressed = compress_data(rep_bytes);

        // Создаем псевдослучайный текст (плохо сжимается)
        let random_text = generate_pseudo_random_text(size);
        let rand_bytes = random_text.as_bytes();
        let rand_compressed = compress_data(rand_bytes);

        let rep_ratio = if rep_bytes.len() > 0 {
            (1.0 - rep_compressed.len() as f64 / rep_bytes.len() as f64) * 100.0
        } else {
            0.0
        };

        let rand_ratio = if rand_bytes.len() > 0 {
            (1.0 - rand_compressed.len() as f64 / rand_bytes.len() as f64) * 100.0
        } else {
            0.0
        };

        println!("📏 Размер {} байт:", size);
        println!("   Повторяющийся: {:.1}% сжатия", rep_ratio);
        println!("   Случайный:     {:.1}% сжатия", rand_ratio);
        println!();
    }

    println!("💡 Выводы:");
    println!("   • Маленькие файлы могут увеличиваться из-за накладных расходов");
    println!("   • Эффективность растет с размером файла");
    println!("   • Тип данных критически важен для результата\n");
}

/// Генерирует псевдослучайный текст заданной длины
fn generate_random_text(length: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 "
        .chars()
        .collect();

    (0..length)
        .map(|i| chars[(i * 7 + 13) % chars.len()]) // Простая псевдослучайность
        .collect()
}

/// Генерирует псевдослучайный текст с лучшим распределением
fn generate_pseudo_random_text(length: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz ".chars().collect();

    (0..length)
        .map(|i| {
            let seed = (i * 31 + 17) % chars.len();
            chars[seed]
        })
        .collect()
}
