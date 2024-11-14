package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)


const MAX_TASKS = 8;


type results struct {
    mutex sync.Mutex
    values [][]Move
}

func (res *results) append_solution(moves []Move) {
    res.mutex.Lock()
    res.values = append(res.values, moves)
    res.mutex.Unlock()
}




// todo: make mutex passed by reference
// todo: test atomic and results passed by value
func game_move(b Board, moves []Move, res *results, task_count *atomic.Int32) {
    if b.is_finished() {
        var moves_copy = make([]Move, len(moves))
        copy(moves_copy, moves)
        res.append_solution(moves_copy)
        task_count.Add(-1)
        return
    }

    var available_moves = b.Get_possible_moves()
    if len(available_moves) == 0 {
        task_count.Add(-1)
        return
    }

    handle_subdivision(&b, &moves, &available_moves, res, task_count)

    calculate_local(&b, &moves, &available_moves, res, task_count)

    task_count.Add(-1)
}


func game_ref(b *Board, moves *[]Move, res *results, task_count *atomic.Int32) {

    if b.is_finished() {
        var moves_copy = make([]Move, len(*moves))
        copy(moves_copy, *moves)
        res.append_solution(moves_copy)
        return
    }

    var available_moves = b.Get_possible_moves()
    if len(available_moves) == 0 {
        return
    }

    handle_subdivision(b, moves, &available_moves, res, task_count)

    calculate_local(b, moves, &available_moves, res, task_count)
}



func handle_subdivision(b *Board, moves *[]Move, available_moves *[]Move, res *results, task_count *atomic.Int32) {
    for {
        var count = task_count.Load()
        if count >= MAX_TASKS {
            return
        }

        var new_value = min(int32(len(*available_moves)) + count, MAX_TASKS)
        var swapped = task_count.CompareAndSwap(count, new_value)
        if swapped {
            var nb_new_tasks = new_value - count
            for i := 0; i < int(nb_new_tasks); i++ {
                var m = (*available_moves)[i]
                var new_board = *b
                var new_moves = *moves
                new_board.apply_move(m)
                new_moves = append(new_moves, m)

                go game_move(new_board, new_moves, res, task_count)
            }
            return;
        }
    }
}

func calculate_local(b *Board, moves *[]Move, available_moves *[]Move, res *results, task_count *atomic.Int32) {
    for i := 0; i < len(*available_moves); i++ {
        var m = (*available_moves)[i]
        b.apply_move(m)
        *moves = append(*moves, m)

        game_ref(b, moves, res, task_count)

        b.undo_move(m)
        *moves = (*moves)[:len(*moves)-1]
    }
}






func main_game() {
    var b = New_board();
    var res results
    var tasks atomic.Int32
    tasks.Store(1)

    go game_move(b, []Move{}, &res, &tasks);

    for {
        time.Sleep(1_000_000_000 * 0.01)    // second_in_nanos * #seconds
        res.mutex.Lock()
        if len(res.values) > 20000 {
            break;
        }
        res.mutex.Unlock()
    }

    print_steps(&b, &res.values[0])
    res.mutex.Unlock()
    

}



func main() {

    var start = time.Now();

    main_game();

    var end = time.Now();

    fmt.Println("duration:", end.UnixMilli() - start.UnixMilli());

}
