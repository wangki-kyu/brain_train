use std::{clone, fmt::format, sync::{RwLock}};

use serde::{Deserialize, Serialize};
use tauri::{ipc::private::ResponseKind, utils::config};
use rand::{seq::SliceRandom, Rng};

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Device};
use cpal::platform::Stream;
use std::time::Duration;
use std::{io::{self, stdout}, sync::{Arc, Mutex}, thread};
use std::sync::LazyLock;

use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc;

// tts
use windows_tts::tts::tts;


// static lazy 형태로 가지고 있어야 할듯함 
static AUDIO_DATA: LazyLock<Arc<Mutex<Vec<f32>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

#[derive(Clone)]
struct RecordData {
    stream : Arc<Mutex<Option<Stream>>>, 
}

unsafe impl Send for RecordData {}

static RECORD_DATA: LazyLock<Arc<Mutex<RecordData>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(RecordData { stream : Arc::new(Mutex::new(None))}))
});


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, get_words_from_file,get_file_env, get_record_json_data 
            ,start_record, pause_record, // 녹음 시작, 녹음 중지
            test_func, stream_record, speak_tts
            ])
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

    println!("get_words_from_file called , {}", target);

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

#[tauri::command]
fn get_record_json_data(category : String) -> Vec<String> {
    let mut path = String::new();
    #[cfg(debug_assertions)]    // debug mode
    {
        path = format!("./assets/voice_record.json");
    }

    #[cfg(not(debug_assertions))]   // release mode
    {
        path = format!("voice_record.json");
    }

//  data = std::fs::read_to_string(path.as_str()).expect("fail");
    let data = std::fs::read_to_string(path.as_str()).expect("fail");
    let json: serde_json::Value = serde_json::from_str(data.as_str()).expect("Invalid Json format");
        
    let category_data: Vec<String> = json[category].as_array().expect("fruit is not an array")
        .iter()
        .map(|v| v.as_str().expect("not a string").to_string())
        .collect();

    category_data
}

#[tauri::command] 
fn start_record() {
    // record한 데이터를 가지고 있어야함...
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    let config = device.default_input_config().unwrap();
    let record_audio_data_clone = Arc::clone(&AUDIO_DATA);
    
    {
        record_audio_data_clone.lock().unwrap().clear();
    }

    let record_data = RECORD_DATA.lock().unwrap();
    let record_data_clone = record_data.clone();
    let mut stream_clone = record_data_clone.stream.lock().unwrap();

    *stream_clone = Some(device.build_input_stream(
        &config.config(), 
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut record_audio_vec = record_audio_data_clone.lock().unwrap();
            for &sample in data.iter() {
                record_audio_vec.push(sample);
            }
        }, 
        |err| {
            eprintln!("Error during recording: {:?}", err);
        }, 
        None
        ).unwrap()
    );

    if let Some(ref mut stream) = *stream_clone {
        println!("녹음 시작");
        stream.play().unwrap();
    }
}

#[tauri::command]
fn pause_record() {
    // stream을 가져와야함. 
    let record_data = RECORD_DATA.lock().unwrap();
    let record_stream = record_data.stream.lock().unwrap();

    if let Some(ref stream) = *record_stream {
        println!("녹음 중지");
        stream.pause().unwrap();
    }
}

#[tauri::command]
fn stream_record() -> Result<(), String> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let recorded_data_clone = AUDIO_DATA.clone();
    let audio_data = recorded_data_clone.lock().unwrap().clone();
    let mut audio_data_iter = audio_data.into_iter();

    if audio_data_iter.len() == 0 {
        return Ok(());
    }

    let temp = Arc::new(Mutex::new(false));
    let temp_clone = temp.clone();

    println!("재생 시작함");
    let output_stream = Arc::new(Mutex::new(
        device.build_output_stream(
            &config.config(), 
            move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut data = temp_clone.lock().unwrap();
                for sample in output.iter_mut() {
                    *sample = audio_data_iter.next().unwrap_or(0.0);

                    if (audio_data_iter.len() == 0) {
                        *data = true;
                    }
                }  
            }, 
            |err| {
                eprintln!("Error during playback: {:?}", err);
            }, 
            None)
            .unwrap()
    ));
        
    output_stream.lock().unwrap().play().unwrap();

    let handle = thread::spawn(move || {
        loop {
            if *temp.lock().unwrap() {
                println!("성공적으로 완료했습니다.");
                break;
            }

            thread::sleep(Duration::from_millis(300));
        }
    });

    handle.join().unwrap();

    println!("재생이 완료되었습니다.");

    Ok(())
}

#[tauri::command]
fn test_func() {
    let audio_data = AUDIO_DATA.lock().unwrap();
    println!("{}", (*audio_data).len());
}

#[tauri::command]
fn speak_tts(text : String) {
    tts(text.as_str());
}
