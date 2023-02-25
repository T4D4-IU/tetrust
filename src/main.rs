use std::{thread, time};
use getch_rs::{Getch, Key};

// fieldsize
const FIELD_WIDTH: usize = 11 + 2; // field + wall
const FIELD_HEIGHT: usize = 20 + 1; // field + botom
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

// blockの種類
#[derive(Clone, Copy)]
enum BlockKind {
    I,
    // O,
    // S,
    // Z,
    // J,
    // L,
    // T,
}

// blockの形状
type BlockShape = [[usize; 4]; 4];
const BLOCKS: [BlockShape; 7] = [
    // I
    [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
    ],

    // O
    [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],

    // S
    [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
    ],

    // Z
    [
        [0, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],

    // J
    [
        [0, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],

    // L
    [
        [0, 0, 0, 0],
        [0, 0, 1, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],

    // T
    [
        [0, 0, 0, 0],
        [0, 1, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
];

struct Position {
    x: usize,
    y: usize,
}

// ブロックがフィールドに衝突する場合は'true'を返す
fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y+pos.y][x+pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    // fieldの管理1がブロック0が空白
    let field = [
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1],
    ];

    let mut pos = Position { x: 4, y: 0 };
    let g = Getch::new();

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    // 30マス分落としてみる
    loop {

    // 描画用のフィールド生成
    let mut field_buf = field;

    // 自然落下
    let new_pos = Position {
        x: pos.x,
        y: pos.y + 1,
    };

    if !is_collision(&field, &new_pos, BlockKind::I) {
        pos.y += 1;
    }


    // 描画用フィールドにブロックの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[BlockKind::I as usize][y][x] == 1 {
                field_buf[y + pos.y][x + pos.x] = 1;
            }
        }
    }

    // フィールドを描画
    println!("\x1b[H");  // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
        }
        // 1秒間スリープする
        thread::sleep(time::Duration::from_millis(1000));

        // キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    // posの座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Right) => {
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    // posの座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Char('q')) => break,
            _ => {}, // 何もしない
        }
    }

    // カーソルを再表示
    println!("\x1b[?25h");
}
