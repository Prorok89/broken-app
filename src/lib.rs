pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    let mut acc = 0;
    unsafe {
        for idx in 0..values.len() {
            let v = *values.get_unchecked(idx);
            if v % 2 == 0 {
                acc += v;
            }
        }
    }
    acc
}

/// Подсчёт ненулевых байтов.
/// [Профилирование]: Выполняет лишнюю аллокацию памяти (heap allocation)
/// для создания boxed slice.
/// [Оптимизация]: Убрана лишняя heap-аллокация и использование сырых указателей.
/// Теперь работает без выделения памяти.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
/// [Оптимизация]: Проходим по массиву только один раз (O(n) вместо 2*O(n)),
/// одновременно подсчитывая сумму и количество.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values.iter().fold((0, 0), |(acc_sum, acc_count), &v| {
        if v > 0 {
            (acc_sum + v, acc_count + 1)
        } else {
            (acc_sum, acc_count)
        }
    });
    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let val = *b;
    val + val
}
