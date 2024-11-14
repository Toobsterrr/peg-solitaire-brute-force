package main

import (
	"fmt"
	"strconv"
)

// 1-indexed
type Pos struct {
    row int
    col int
}

type Direction int

const (
    Right Direction = iota
    Down
    Left
    Up
)

func (d *Direction) to_string() string {
    switch *d {
    case Right:
        return "Right"
    case Down:
        return "Down"
    case Left:
        return "Left"
    case Up:
        return "Up"
    }
    return "Error"
}


type Move struct {
    direction Direction
    pos Pos
}


type Board struct {
    pieces [7][7]bool
    nb_pieces int
}


func New_board() Board {
    var b = Board {
        pieces: [7][7]bool{
            {false, false, true, true, true, false, false},
            {false, false, true, true, true, false, false},
            {true, true, true, true, true, true, true},
            {true, true, true, false, true, true, true},
            {true, true, true, true, true, true, true},
            {false, false, true, true, true, false, false},
            {false, false, true, true, true, false, false},
        },
        nb_pieces: 32,
    }

    return b;
}


func (b *Board) is_finished() bool {
    return b.nb_pieces == 1;
}

func is_position_valid(row int, col int) bool {
    return (row >= 3 && row <= 5 && col > 0 && col < 8) || (col >= 3 && col <= 5 && row > 0 && row < 8);
}

func (b *Board) contains_piece(row int, col int) bool {
    return b.pieces[row-1][col-1];
}


func (b *Board) Get_possible_moves() []Move {
    var res = make([]Move, 0);

    for row := 1; row < 8; row++ {
        for col := 1; col < 8; col++ {
            if !b.contains_piece(row, col) {
                continue;
            }
            if is_position_valid(row, col+2) && !b.contains_piece(row, col+2) && b.contains_piece(row, col+1) {
                res = append(res, Move{direction: Right, pos: Pos{row: row, col: col}});
            }
            if is_position_valid(row, col-2) && !b.contains_piece(row, col-2) && b.contains_piece(row, col-1) {
                res = append(res, Move{direction: Left, pos: Pos{row: row, col: col}});
            }
            if is_position_valid(row+2, col) && !b.contains_piece(row+2, col) && b.contains_piece(row+1, col) {
                res = append(res, Move{direction: Down, pos: Pos{row: row, col: col}});
            }
            if is_position_valid(row-2, col) && !b.contains_piece(row-2, col) && b.contains_piece(row-1, col) {
                res = append(res, Move{direction: Up, pos: Pos{row: row, col: col}});
            }
        }    
    }

    return res;
}

func (b *Board) apply_move(m Move) {
    var row = m.pos.row-1;
    var col = m.pos.col-1;

    b.nb_pieces--;

    switch m.direction {
    case Right:
        b.pieces[row][col] = false;
        b.pieces[row][col+1] = false;
        b.pieces[row][col+2] = true;
    case Down:
        b.pieces[row][col] = false;
        b.pieces[row+1][col] = false;
        b.pieces[row+2][col] = true;
    case Left:
        b.pieces[row][col] = false;
        b.pieces[row][col-1] = false;
        b.pieces[row][col-2] = true;
    case Up:
        b.pieces[row][col] = false;
        b.pieces[row-1][col] = false;
        b.pieces[row-2][col] = true;
    }
    
}

func (b *Board) undo_move(m Move) {
    var row = m.pos.row-1;
    var col = m.pos.col-1;

    b.nb_pieces++;

    switch m.direction {
    case Right:
        b.pieces[row][col] = true;
        b.pieces[row][col+1] = true;
        b.pieces[row][col+2] = false;
    case Down:
        b.pieces[row][col] = true;
        b.pieces[row+1][col] = true;
        b.pieces[row+2][col] = false;
    case Left:
        b.pieces[row][col] = true;
        b.pieces[row][col-1] = true;
        b.pieces[row][col-2] = false;
    case Up:
        b.pieces[row][col] = true;
        b.pieces[row-1][col] = true;
        b.pieces[row-2][col] = false;
    }
}


func (b *Board) to_print_string() string {
    var res = "";
    for row := 0; row < 7; row++ {
        res += strconv.Itoa(row+1) + " ";
        for col := 0; col < 6; col++ {
            if b.pieces[row][col] {
                res += "+ ";
            } else {
                res += "  ";
            }
        }

        if b.pieces[row][6] {
            res += "+\n";
        } else {
            res += " \n";
        }    
    }
    res += " ";
    for i := 1; i < 8; i++ {
        res += " " + strconv.Itoa(i);
    }
    return res;
}


func print_steps(b *Board, steps *[]Move) {
    fmt.Println(b.to_print_string())
    for i := 0; i < len(*steps); i++ {
        b.apply_move((*steps)[i]);
        fmt.Println((*steps)[i].direction.to_string(), ": (", (*steps)[i].pos.row, ",", (*steps)[i].pos.col, ") -------------------------");
        fmt.Println(b.to_print_string())
    }
}