#include <iostream>
#include <bitset>
#include <vector>
#include <algorithm>
#include <omp.h>

using namespace std;

#define BOX_SIZE 4
#define BOARD_SIZE BOX_SIZE * BOX_SIZE

class Vec2D {
public:
    int x, y;

    Vec2D() {
        x = 0;
        y = 0;
    }

    Vec2D(int a, int b) {
        x = a;
        y = b;
    }

    Vec2D operator+(Vec2D other) {
        return Vec2D(x + other.x, y + other.y);
    }

    Vec2D operator*(int factor) {
        return Vec2D(x * factor, y * factor);
    }

    Vec2D operator/(int factor) {
        return Vec2D(x / factor, y / factor);
    }
};

ostream& operator<<(ostream& os, Vec2D vec) {
    os << '(' << vec.x << ", " << vec.y << ')';
    return os;
}

enum CellType {
    Empty, Given, Guess
};

class Cell {
public:
    int value;
    CellType cell_type;

    Cell() {
        cell_type = CellType::Empty;
        value = 0;
    }

    static Cell given(int value) {
        Cell cell;
        cell.cell_type = CellType::Given;
        cell.value = value;
        return cell;
    }
};

ostream& operator<<(ostream& os, Cell cell) {
    switch (cell.cell_type) {
        case CellType::Empty:
            os << "\033[1;37m" << "-" << "\033[0m";
            break;
        case CellType::Given:
            os << "\033[1;36m" << cell.value << "\033[0m";
            break;
        case CellType::Guess:
            os << "\033[1;33m" << cell.value << "\033[0m";
            break;
    }
    return os;
}

class GameBoard {
public:
    Cell board[BOARD_SIZE][BOARD_SIZE];

    Vec2D box_position(Vec2D cell_pos) {
        return (cell_pos / BOX_SIZE) * BOX_SIZE;
    }

    vector<Vec2D> box_cell_positions(Vec2D cell_pos) {
        Vec2D box_pos = box_position(cell_pos);

        vector<Vec2D> result;
        result.reserve(BOARD_SIZE);
        for (int i = 0; i < BOX_SIZE; ++i) {
            for (int j = 0; j < BOX_SIZE; ++j)
                result.push_back(box_pos + Vec2D(i, j));
        }

        return result;
    }

    void print() {
        for (int i = 0; i < BOARD_SIZE; ++i) {
            for (int j = 0; j < BOARD_SIZE; ++j)
                cout << board[i][j] << "\t";
            cout << endl;
        }
        cout << endl;
    }

    void print_normal() {
        for (int i = 0; i < BOARD_SIZE; ++i) {
            for (int j = 0; j < BOARD_SIZE; ++j) {
                if (board[i][j].cell_type == CellType::Empty)
                    cout << "-" << "\t";
                else
                    cout << board[i][j].value << "\t";
            }
            cout << endl;
        }
        cout << endl;
    }
};

class CellTile {
    bitset<BOARD_SIZE + 1> tiles;

public:
    void flip() {
        tiles.flip();
    }

    void remove(int tile) {
        tiles[tile] = false;
    }

    int count() {
        return tiles.count() - tiles.test(0);
    }

    bool has(int tile) {
        return tiles.test(tile);
    }

    int first_set_tile() {
        for (int k = 1; k <= BOARD_SIZE; ++k) {
            if (has(k))
                return k;
        }
        return -1;
    }
};


GameBoard exploratory_solution;
bool found_soln;
double time_to_soln;

class WaveState {
public:
    GameBoard gameboard;
    CellTile entropy_board[BOARD_SIZE][BOARD_SIZE];

    static WaveState build(GameBoard gameboard) {
        WaveState simulation;
        simulation.gameboard = gameboard;

        for (int i = 0; i < BOARD_SIZE; ++i) {
            for (int j = 0; j < BOARD_SIZE; ++j) {
                if (simulation.gameboard.board[i][j].cell_type == CellType::Empty)
                    simulation.entropy_board[i][j].flip();
            }
        }

        for (int i = 0; i < BOARD_SIZE; ++i) {
            for (int j = 0; j < BOARD_SIZE; ++j) {
                Cell cell = simulation.gameboard.board[i][j];
                if (cell.cell_type == CellType::Given)
                    simulation.apply_heuristics(Vec2D(i, j), cell.value);
            }
        }

        return simulation;
    }

    void heuristics_on_cell(Vec2D pos, int removing_tile) {
        entropy_board[pos.x][pos.y].remove(removing_tile);
    }

    void apply_heuristics(Vec2D pos, int given_tile) {
        for (int i = 0; i < BOARD_SIZE; ++i) {
            heuristics_on_cell(Vec2D(i, pos.y), given_tile);
        }

        for (int i = 0; i < BOARD_SIZE; ++i) {
            heuristics_on_cell(Vec2D(pos.x, i), given_tile);
        }

        for (auto pos : gameboard.box_cell_positions(pos)) {
            heuristics_on_cell(pos, given_tile);
        }
    }

    Vec2D* least_entropy_cell() {
        int min_entropy = BOARD_SIZE + 1;
        Vec2D* result = NULL;

        for (int i = 0; i < BOARD_SIZE; ++i) {
            for (int j = 0; j < BOARD_SIZE; ++j) {
                if (
                    gameboard.board[i][j].cell_type == CellType::Empty &&
                    entropy_board[i][j].count() < min_entropy
                    ) {
                    result = new Vec2D(i, j);
                    min_entropy = entropy_board[i][j].count();
                }
            }
        }

        return result;
    }

    GameBoard* recursive_decomposition() {
        Vec2D* least_cell;

        while (true) {
            least_cell = least_entropy_cell();
            if (least_cell == NULL)
                return &gameboard;

            CellTile* tiles = &entropy_board[least_cell->x][least_cell->y];

            if (tiles->count() == 0)
                return NULL;
            else if (tiles->count() > 1)
                break;
            else {
                int only_tile = tiles->first_set_tile();
                gameboard.board[least_cell->x][least_cell->y].cell_type = CellType::Guess;
                gameboard.board[least_cell->x][least_cell->y].value = only_tile;
                apply_heuristics(*least_cell, only_tile);
            }
        }

        CellTile* tiles = &entropy_board[least_cell->x][least_cell->y];

        for (int k = 1; k <= BOARD_SIZE; ++k) {
            if (tiles->has(k)) {
                WaveState cloned_sim = *this;

                cloned_sim.gameboard.board[least_cell->x][least_cell->y].cell_type = CellType::Guess;
                cloned_sim.gameboard.board[least_cell->x][least_cell->y].value = k;
                cloned_sim.apply_heuristics(*least_cell, k);

                GameBoard* solution = cloned_sim.recursive_decomposition();
                if (solution != NULL)
                    return solution;
            }
        }

        return NULL;
    }

    void exploratory_decomposition_helper() {
#pragma omp task
        {
#pragma omp cancellation point taskgroup

            Vec2D* least_cell;

            while (true) {
#pragma omp cancellation point taskgroup

                if (found_soln) {
#pragma omp cancel taskgroup
                    break;
                }

                least_cell = least_entropy_cell();
                if (least_cell == NULL) {
#pragma omp critical
                    if (!found_soln) {
                        found_soln = true;
                        exploratory_solution = gameboard;
                        time_to_soln = omp_get_wtime();
                    }
#pragma omp cancel taskgroup
                    break;
                }


#pragma omp cancellation point taskgroup

                CellTile* tiles = &entropy_board[least_cell->x][least_cell->y];

                if (tiles->count() == 0)
                    break;
                else if (tiles->count() > 1)
                    break;
                else {
                    int only_tile = tiles->first_set_tile();
                    gameboard.board[least_cell->x][least_cell->y].cell_type = CellType::Guess;
                    gameboard.board[least_cell->x][least_cell->y].value = only_tile;
                    apply_heuristics(*least_cell, only_tile);
                }
            }

#pragma omp cancellation point taskgroup

            CellTile* tiles = &entropy_board[least_cell->x][least_cell->y];

#pragma omp taskgroup
            for (int k = 1; k <= BOARD_SIZE; ++k) {
                if (tiles->has(k)) {
                    WaveState cloned_sim = *this;

                    cloned_sim.gameboard.board[least_cell->x][least_cell->y].cell_type = CellType::Guess;
                    cloned_sim.gameboard.board[least_cell->x][least_cell->y].value = k;
                    cloned_sim.apply_heuristics(*least_cell, k);

                    cloned_sim.exploratory_decomposition_helper();
                }
            }
        }
    }

    void exploratory_decomposition() {
#pragma omp parallel
#pragma omp single
        exploratory_decomposition_helper();
    }

    void print() {
        gameboard.print();
        cout << endl;

        for (int i = 0; i < BOARD_SIZE; ++i) {
            for (int j = 0; j < BOARD_SIZE; ++j) {
                for (int k = 1; k <= BOARD_SIZE; ++k) {
                    if (entropy_board[i][j].has(k))
                        cout << k << ",";
                    else
                        cout << "_" << ",";
                }
                cout << "\t|\t";
            }
            cout << endl;
        }
        cout << endl;
    }
};

int main() {
    GameBoard gameboard;
    gameboard.board[0][0] = Cell::given(9);
    gameboard.board[0][1] = Cell::given(1);
    gameboard.board[0][2] = Cell::given(2);
    gameboard.board[1][0] = Cell::given(3);
    gameboard.board[1][1] = Cell::given(4);
    gameboard.board[1][2] = Cell::given(5);
    gameboard.board[2][0] = Cell::given(6);
    gameboard.board[2][1] = Cell::given(7);
    gameboard.board[4][1] = Cell::given(8);

    gameboard.print_normal();

    WaveState simulation;
    double t1, t2;
    GameBoard res;
    GameBoard* solution;

    simulation = WaveState::build(gameboard);

    t1 = omp_get_wtime();

    solution = simulation.recursive_decomposition();
    if (solution == NULL)
        return -1;

    t2 = omp_get_wtime();

    res = *solution;
    res.print_normal();

    cout << "Time Taken :\t" << t2 - t1 << endl;


    simulation = WaveState::build(gameboard);

    t1 = omp_get_wtime();

    simulation.exploratory_decomposition();
    if (!found_soln)
        return -1;

    t2 = omp_get_wtime();

    exploratory_solution.print_normal();

    cout << "Time to Solution :\t" << time_to_soln - t1 << endl;
    cout << "Time Taken :\t" << t2 - t1 << endl;

    return -1;
}
