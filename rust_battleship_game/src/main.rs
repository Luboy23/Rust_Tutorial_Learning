// 导入必要的包和组件
use std::{ io::{self, Write}, usize};
use rand::Rng;

// 定义游戏棋盘的大小和常量
const BOARD_SIZE: usize = 10;

// 定义游戏的棋盘结构体
struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE], // 棋盘上的网格
    ships: Vec<(usize, usize)>, // 舰艇位置的集合
}

// 定义棋盘中每个格子的状态
#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,  // 空白
    Ship,   // 舰艇位置
    Hit,    // 击中
    Miss    // 未击中
}

impl Board {
    // 初始化一个新的棋盘
    fn new() -> Self {
        Board {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE], // 将棋盘初始化为空
            ships: Vec::new(), // 初始化舰艇为空
        }
    }

    // 随机在棋盘上摆放舰艇
    fn place_ship(&mut self, size: usize) {
        let mut rng = rand::thread_rng(); // 随机数生成器

        loop {
            let row = rng.gen_range(0..BOARD_SIZE); // 随机行
            let col = rng.gen_range(0..BOARD_SIZE); // 随机列
            let direction = rng.gen::<bool>(); // 随机方向 (true 为水平，false 为垂直)

            // 如果舰艇可以放置在指定位置
            if self.can_place_ship(row, col, size, direction) {
                for i in 0..size {
                    let (r, c) = if direction {
                        (row, col + i) // 水平放置
                    } else {
                        (row + i, col) // 垂直放置
                    };
                    self.grid[r][c] = CellState::Ship; // 标记舰艇
                    self.ships.push((r, c)); // 记录舰艇位置
                }
                break; // 成功放置舰艇后跳出循环
            }
        }
    }

    // 检查舰艇是否可以放置在指定位置
    fn can_place_ship(&self, row: usize, col: usize, size: usize, direction: bool) -> bool {
        if direction {
            if col + size > BOARD_SIZE { return false; } // 如果舰艇超出棋盘，返回 false
            for i in 0..size {
                if self.grid[row][col + i] != CellState::Empty { return false; } // 如果位置已被占用，返回 false
            }
        } else {
            if row + size > BOARD_SIZE { return false; } // 垂直方向同样检查
            for i in 0..size {
                if self.grid[row + i][col] != CellState::Empty { return false; }
            }
        }
        true // 可以放置舰艇
    }
    
    // 玩家在指定位置开火
    fn fire(&mut self, row: usize, col: usize) -> CellState {
        match self.grid[row][col] {
            CellState::Empty => {
                self.grid[row][col] = CellState::Miss; // 如果是空的，标记为未击中
                CellState::Miss
            },
            CellState::Ship => {
                self.grid[row][col] = CellState::Hit; // 如果是舰艇，标记为击中
                CellState::Hit
            },
            _ => CellState::Miss, // 其他情况，返回未击中
        }
    }

    // 显示游戏棋盘的方法
    fn display(&self, hide_ships: bool) {
        print!("  ");
        for i in 0..BOARD_SIZE { print!(" {} ", i); } // 显示列号
        println!();
        for (i, row) in self.grid.iter().enumerate() {
            print!("{:2}", i); // 显示行号
            for cell in row {
                match cell {
                    CellState::Empty => {
                        if hide_ships {
                            print!("   "); // 隐藏空格子
                        } else {
                            print!(" \u{25A1} ");  // 显示空白方块
                        }
                    }
                    CellState::Ship => {
                        if hide_ships { print!("   "); } else { print!(" \u{25A0} "); }  // 显示玩家舰艇位置为实心方块
                    }
                    CellState::Hit => print!("\x1b[31m \u{25CF} \x1b[0m"),  // 显示命中为红色圆点
                    CellState::Miss => print!(" \u{25CB} "), // 显示未击中为空心圆
                }
            }
            println!();
        }
    }

    // 判断游戏是否结束
    fn is_game_over(&self) -> bool {
        self.ships.iter().all(|&(r, c)| self.grid[r][c] == CellState::Hit) // 检查是否所有舰艇都被击中
    }
}

fn main() {
    let mut player_board = Board::new(); // 创建玩家棋盘
    let mut opponent_board = Board::new(); // 创建对手棋盘

    // 玩家和对手分别放置舰艇
    player_board.place_ship(5);
    player_board.place_ship(4);
    player_board.place_ship(3);
    player_board.place_ship(3);
    player_board.place_ship(2);

    opponent_board.place_ship(5);
    opponent_board.place_ship(4);
    opponent_board.place_ship(3);
    opponent_board.place_ship(3);
    opponent_board.place_ship(2);

    // 游戏主循环
    loop {
        println!("\x1b[2J\x1b[1;H"); // 清屏

        println!("\x1b[1;37m你的棋盘:\x1b[0m");
        player_board.display(false); // 显示玩家棋盘
        println!("\x1b[1;37m对手的棋盘:\x1b[0m");
        opponent_board.display(true); // 显示对手棋盘（隐藏舰艇）

        // 获取玩家的开火坐标
        let (player_row, player_col) = get_player_input();
        let result = opponent_board.fire(player_row, player_col);
        match result {
            CellState::Miss => println!("\x1b[36m未击中!\x1b[0m"),
            CellState::Hit => println!("\x1b[36m击中对手的舰艇！\x1b[0m"),
            _ => (),
        }
        println!("按下回车键以继续...");
        io::stdin().read_line(&mut String::new()).expect("读取失败！");

        if opponent_board.is_game_over() {
            println!("\x1b[1;32m恭喜你！你击沉了对手的全部舰艇！\x1b[0m");
            break;
        }

        // 生成对手的开火坐标
        let (opponent_row, opponent_col) = generate_opponent_move();
        let result = player_board.fire(opponent_row, opponent_col);
        match result {
            CellState::Miss => println!("\x1b[36m对手未击中！\x1b[0m"),
            CellState::Hit => println!("\x1b[36m对手击中了你的舰艇！\x1b[0m"),
            _ => (),
        }
        println!("按下回车键以继续...");
        io::stdin().read_line(&mut String::new()).expect("读取失败！");

        if player_board.is_game_over() {
            println!("\x1b[1;31m非常遗憾！你的舰艇被对手全部击沉！\x1b[0m");
            break;
        }
    }
}

// 获取玩家输入的开火坐标
fn get_player_input() -> (usize, usize) {
    loop {
        print!("\x1b[1;37m输入准确坐标进行打击(行, 列): \x1b[0m");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("读取失败！");
        let coordinates: Vec<usize> = input.trim().split(',')
            .map(|s: &str| s.trim().parse().expect("错误输入！")).collect();
        if coordinates.len() == 2 && coordinates[0] < BOARD_SIZE && coordinates[1] < BOARD_SIZE {
            return (coordinates[0], coordinates[1]);
        } else {
            println!("\x1b[1;31m错误输入！请输入正确的行列坐标，用逗号进行分隔。\x1b[0m");
        }
    }
}

// 生成对手随机的开火坐标
fn generate_opponent_move() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE))
}