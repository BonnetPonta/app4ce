use std::io::{self, Write};

#[repr(C)]
struct Player {
    hp: i32,
    mp: i32,
    gold: i32,
    exp: i32,
    level: i32,
    x: f32,
    y: f32,
}

fn clear() {
    print!("\x1B[2J\x1B[H");
}

fn main() {
    let exe = std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let mut player = Box::new(Player {
        hp: 100,
        mp: 50,
        gold: 1000,
        exp: 0,
        level: 1,
        x: 100.0,
        y: 200.0,
    });

    let mut hint = false;

    loop {
        clear();

        println!("==============================");
        println!(" Cheat Engine Practice");
        println!("==============================");
        println!("Process : {}", exe);
        println!("PID     : {}", std::process::id());
        println!();

        println!("HP    : {}", player.hp);
        println!("MP    : {}", player.mp);
        println!("Gold  : {}", player.gold);
        println!("EXP   : {}", player.exp);
        println!("Level : {}", player.level);
        println!("X     : {:.2}", player.x);
        println!("Y     : {:.2}", player.y);

        if hint {
            println!("\n===== Hint =====");
            println!("Player : {:p}", &*player);
            println!("HP     : {:p}", &player.hp);
            println!("MP     : {:p}", &player.mp);
            println!("Gold   : {:p}", &player.gold);
            println!("EXP    : {:p}", &player.exp);
            println!("Level  : {:p}", &player.level);
            println!("X      : {:p}", &player.x);
            println!("Y      : {:p}", &player.y);
        }

        println!();
        println!("1: HP-10");
        println!("2: HP+10");
        println!("3: Gold-100");
        println!("4: Gold+100");
        println!("5: EXP+50");
        println!("6: Move");
        println!("7: Hint ON/OFF");
        println!("0: Exit");

        print!("> ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();

        match cmd.trim() {
            "1" => player.hp = (player.hp - 10).max(0),
            "2" => player.hp += 10,
            "3" => player.gold = (player.gold - 100).max(0),
            "4" => player.gold += 100,
            "5" => {
                player.exp += 50;
                if player.exp >= 100 {
                    player.exp -= 100;
                    player.level += 1;
                }
            }
            "6" => {
                player.x += 1.25;
                player.y += 0.75;
            }
            "7" => hint = !hint,
            "0" => break,
            _ => {}
        }
    }
}
