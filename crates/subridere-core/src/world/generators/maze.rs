use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

/// Клетка в лабиринте (x,z)
pub type Cell = IVec2;

/// Направления и смещения
const DIRS: &[(Cell, Cell)] = &[
    (Cell::new(1, 0), Cell::new(1, 0)),   // право
    (Cell::new(-1, 0), Cell::new(-1, 0)), // лево
    (Cell::new(0, 1), Cell::new(0, 1)),   // вперед (+z)
    (Cell::new(0, -1), Cell::new(0, -1)), // назад (-z)
];

/// Возвращает карту смежности: для каждой клетки — список соседей,
/// с которыми есть «коридор» (т. е. дверь).
pub fn generate_maze(width: i32, height: i32) -> HashMap<Cell, Vec<Cell>> {
    let mut rng = thread_rng();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut adj: HashMap<Cell, Vec<Cell>> = HashMap::new();

    // стартовая клетка
    let start = Cell::new(0, 0);
    visited.insert(start);
    stack.push(start);

    while let Some(cur) = stack.pop() {
        // собираем список не посещённых соседей
        let neighbors = DIRS
            .iter()
            .map(|(delta, _)| cur + *delta)
            .filter(|&c| c.x >= 0 && c.x < width && c.y >= 0 && c.y < height)
            .filter(|c| !visited.contains(c))
            .collect::<Vec<_>>();

        if !neighbors.is_empty() {
            // назад в стек, чтобы вернуться позже
            stack.push(cur);

            // выбираем случайного соседа
            let &next = neighbors.choose(&mut rng).unwrap();
            // отмечаем связь обе стороны
            adj.entry(cur).or_default().push(next);
            adj.entry(next).or_default().push(cur);

            // маркируем и продолжаем
            visited.insert(next);
            stack.push(next);
        }
    }

    adj
}
