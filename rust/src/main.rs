use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use std::slice::Iter;

mod board;
use board::{Board, Move};





const MAX_TASKS: i32 = 8;

static TASK_COUNT: AtomicI32 = AtomicI32::new(1);
// static ABORT: AtomicBool = AtomicBool::new(false);


fn game_move(mut board: Box<Board>, mut moves: Vec<Move>, results: Arc<Mutex<Vec<Vec<Move>>>>) {
    if board.is_finished() {
        let mut m = results.lock().expect("Couldn't lock results mutex.");
        (*m).push(moves.clone());
        TASK_COUNT.fetch_sub(1, Ordering::Relaxed);
        return;
    }


    let available_moves = board.get_possible_moves();
    let nb_moves = available_moves.len();
    if nb_moves == 0 { 
        TASK_COUNT.fetch_sub(1, Ordering::Relaxed);
        return; 
    }
    let mut moves_iter = available_moves.iter();


    // if ABORT.load(Ordering::Relaxed) {
    //     TASK_COUNT.fetch_sub(1, Ordering::Relaxed);
    //     return;
    // }

    handle_subdivision(&board, &moves, &results, &mut moves_iter, nb_moves);

    calculate_local(&mut board, &mut moves, &results, &mut moves_iter);

    TASK_COUNT.fetch_sub(1, Ordering::Relaxed);


}


fn game_ref(board: &mut Box<Board>, moves: &mut Vec<Move>, results: &Arc<Mutex<Vec<Vec<Move>>>>) {
    if board.is_finished() {
        let mut m = results.lock().expect("Couldn't lock results mutex.");
        (*m).push(moves.clone());
        return;
    }

    let available_moves = board.get_possible_moves();
    let nb_moves = available_moves.len();
    if nb_moves == 0 { return; }
    let mut moves_iter: Iter<'_, Move> = available_moves.iter();

    
    // if ABORT.load(Ordering::Relaxed) {
    //     return;
    // }

    handle_subdivision(board, moves, results, &mut moves_iter, nb_moves);

    calculate_local(board, moves, results, &mut moves_iter);
}


fn handle_subdivision(board: &Box<Board>, moves: &Vec<Move>, results: &Arc<Mutex<Vec<Vec<Move>>>>, moves_iter: &mut Iter<'_, Move>, nb_moves: usize) {
    loop {
        let old_task_count = TASK_COUNT.load(Ordering::Relaxed);
        if old_task_count >= MAX_TASKS {
            return;
        }
        
        let new_count = std::cmp::min(nb_moves as i32 + old_task_count, MAX_TASKS);
        if let Ok(old_value) = TASK_COUNT.compare_exchange(old_task_count, new_count, Ordering::Release, Ordering::Relaxed) {
            let nb_new_tasks = new_count - old_value;
            for _ in 0..nb_new_tasks {
                let m = moves_iter.next().unwrap();
                let mut board = board.clone();
                let mut moves = moves.clone();
                board.apply_move(&m);
                moves.push(m.clone());
                let results = Arc::clone(results);
    
                rayon::spawn(move || { game_move(board, moves, results); })
            }
            return;
        }
    }

    // slower version
    // if let Ok(old_task_count) = TASK_COUNT.fetch_update(Ordering::Release, Ordering::Relaxed, |x| {
    //     let new_task_count = std::cmp::min(nb_moves as i32 + x, MAX_TASKS);
    //     Some(new_task_count)
    // }) {
    //     let nb_new_tasks = std::cmp::min(nb_moves as i32, MAX_TASKS - old_task_count);
    //     for _ in 0..nb_new_tasks {
    //         let m = moves_iter.next().unwrap();
    //         let mut board = board.clone();
    //         let mut moves = moves.clone();
    //         board.apply_move(&m);
    //         moves.push(m.clone());
    //         let results = Arc::clone(results);

    //         rayon::spawn(move || { game_move(board, moves, results); })
    //     }
    // }
}

fn calculate_local(board: &mut Box<Board>, moves: &mut Vec<Move>, results: &Arc<Mutex<Vec<Vec<Move>>>>, moves_iter: &mut Iter<'_, Move>) {
    for m in moves_iter {
        board.apply_move(m);
        moves.push(m.clone());

        game_ref(board, moves, results);

        board.undo_move(m);
        moves.pop();
    }
}


fn main_game() {
    let mut board = Board::new();
    let moves: Vec<Move> = Vec::new();
    let results = Arc::new(Mutex::new(Vec::<Vec<Move>>::new()));
    let results_ref = Arc::clone(&results);

    rayon::spawn(move || { game_move(Box::new(board), moves, results_ref); });

    loop {
        sleep(Duration::from_millis(10));
        if results.lock().unwrap().len() > 20000 {
            // ABORT.store(true, Ordering::Relaxed);
            break;
        }
    }

    board::print_steps(&mut board, &results.lock().unwrap()[0]);
}



fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    main_game();

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("duration: {}", (end-start).as_millis());

    
}


