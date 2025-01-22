use std::{clone, fmt::format};

use serde::{Deserialize, Serialize};
use tauri::ipc::private::ResponseKind;
use rand::{seq::SliceRandom, Rng};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_words_from_file,get_file_env ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WordGame {
    word: String,
    image : String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WordGameContainer {
    word_list : Vec<WordGame>
}

// front에게 전달하는 낱말 문제 
#[derive(Serialize, Deserialize, Debug, Clone)]
struct WordProblem {
    problem : String,
    problem_images : Vec<String>
}

// 매개변수로 낱말인지 뭔지를 받아와야할듯,,?


// 반환하는 인터페이스를 만들면 좋을 듯
#[tauri::command]
fn get_words_from_file(target : String) -> WordProblem{

    let mut data = String::new();
    #[cfg(debug_assertions)]
    {
        let path = format!("./assets/{}.json", target);
        data = std::fs::read_to_string(path.as_str()).expect("fail");
    }

    #[cfg(not(debug_assertions))]
    {
        let path = format!("{}.json", target);
        data = std::fs::read_to_string(path.as_str()).expect("fail");
    }
    
    let word_game_container: WordGameContainer = serde_json::from_str(&data).expect("fail");

    let mut rng = rand::thread_rng();

    let choosen_word : Vec<_> = word_game_container.word_list.choose_multiple(&mut rng, 2).collect();

    println!("{:?}", choosen_word);

    let problem = choosen_word.choose(&mut rng).unwrap();

    println!("choose problem : {:?}", problem);

    let image_list : Vec<String> = choosen_word.iter().map(|w| w.image.clone() ).collect();

    println!("image_list : {:?}", image_list);

    let word_problem = WordProblem { problem : problem.word.clone(), problem_images : image_list};

    word_problem
}

#[tauri::command]
fn get_file_env(target: String) -> String {
    let mut path = String::new();
    #[cfg(debug_assertions)]
    {
        // image : /src-tauri/assets/images/
        println!("디버깅 모드입니다.");
        path = format!("/src-tauri/assets/images/{}/", target);
        // path = "/src-tauri/assets/images/".to_string();
    }

    #[cfg(not(debug_assertions))]
    {
        println!("릴리즈 모드입니다.");
        path = format!("./images/{}/", target);
        // path = "./images/".to_string();
    }

    // let path = std::env::current_exe().unwrap().to_string_lossy().to_string();
    // println!("exe path : {}", path);
    path
}

impl ResponseKind for WordGameContainer {

}

impl ResponseKind for WordProblem {
    
}

