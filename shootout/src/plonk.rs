pub fn sudoku() -> sudoku_plonk::JubSudoku {
    sudoku_plonk::make_circuit([
        [7, 6, 0, 5, 3, 8, 1, 2, 4],
        [2, 4, 3, 7, 1, 0, 6, 5, 8],
        [8, 5, 1, 4, 6, 2, 0, 7, 3],
        [4, 8, 6, 0, 7, 5, 3, 1, 2],
        [5, 3, 7, 6, 2, 1, 4, 8, 0],
        [1, 0, 2, 8, 4, 3, 7, 6, 5],
        [6, 1, 8, 3, 5, 4, 2, 0, 7],
        [0, 7, 4, 2, 8, 6, 5, 3, 1],
        [3, 2, 5, 1, 0, 7, 8, 4, 6],
    ])
}
