use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    chapter_number: u32,
    title: String,
    questions: Vec<String>,
    choices: Vec<String>,
    answers: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuizData {
    chapters: Vec<Question>,
}

fn clear_screen() {
    execute!(io::stdout(), Clear(ClearType::All)).expect("Failed to clear screen");
    print!("\x1B[H");
    io::stdout().flush().unwrap();
}

fn display_question(chapter: &Question, question_index: usize) {
    clear_screen();

    println!("=== {} ===\n", chapter.title);
    println!("{}\n", chapter.questions[question_index]);

    println!("語群:");
    for (i, choice) in chapter.choices.iter().enumerate() {
        println!("{}. {}", i + 1, choice);
    }

    print!("\n回答を入力してください (1-{}): ", chapter.choices.len());
    io::stdout().flush().unwrap();
}

fn get_user_input() -> usize {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => return num,
            _ => {
                print!("有効な数字を入力してください: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}

fn wait_for_enter() {
    print!("\nEnterキーを押して続行...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

fn run_quiz(chapter: &Question) {
    let mut correct_count = 0;
    let total_questions = chapter.questions.len();

    // Create a vector of indices and shuffle them
    let mut question_indices: Vec<usize> = (0..total_questions).collect();
    question_indices.shuffle(&mut thread_rng());

    for (i, &question_index) in question_indices.iter().enumerate() {
        display_question(chapter, question_index);

        let user_answer = get_user_input();
        let correct_answer = chapter.answers[question_index];

        if user_answer == correct_answer {
            println!("\n✓ 正解！");
            correct_count += 1;
        } else {
            println!("\n✗ 不正解");
            println!(
                "正解: {}. {}",
                correct_answer,
                chapter.choices[correct_answer - 1]
            );
        }

        if i < total_questions - 1 {
            wait_for_enter();
        }
    }

    println!("\n=== {} 終了 ===", chapter.title);
    println!("正解数: {} / {}", correct_count, total_questions);
    println!(
        "正解率: {:.1}%",
        (correct_count as f64 / total_questions as f64) * 100.0
    );
    wait_for_enter();
}

fn select_chapter(quiz_data: &QuizData) -> Option<&Question> {
    clear_screen();

    println!("=== クイズアプリ ===\n");
    println!("章を選択してください:");

    for (i, chapter) in quiz_data.chapters.iter().enumerate() {
        println!(
            "{}. {} ({} 問)",
            i + 1,
            chapter.title,
            chapter.questions.len()
        );
    }

    println!("0. 終了");

    print!("\n選択 (0-{}): ", quiz_data.chapters.len());
    io::stdout().flush().unwrap();

    let choice = get_user_input();

    if choice == 0 {
        None
    } else if choice <= quiz_data.chapters.len() {
        Some(&quiz_data.chapters[choice - 1])
    } else {
        println!("無効な選択です。");
        wait_for_enter();
        select_chapter(quiz_data)
    }
}

fn main() {
    let quiz_data_str =
        fs::read_to_string("questions.json").expect("questions.json ファイルが見つかりません");

    let quiz_data: QuizData =
        serde_json::from_str(&quiz_data_str).expect("JSON データの解析に失敗しました");

    loop {
        match select_chapter(&quiz_data) {
            Some(chapter) => run_quiz(chapter),
            None => {
                clear_screen();
                println!("クイズアプリを終了します。");
                break;
            }
        }
    }
}

