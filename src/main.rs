use std::sync::mpsc;
use std::{fs::File, thread};
use std::io;

use csv::Writer;

use fake::faker::name::en::*;
use fake::Fake;

use chrono::{Duration, Utc};
use rand::Rng;

fn main() {
    let start_time = Utc::now();
    let file_path = "data.csv";

    println!("Começou em: {}", start_time);

    match generate_billion_file(&file_path) {
        Ok(_) => println!("Arquivo gerado com sucesso!"),
        Err(e) => eprintln!("Erro ao gerar o arquivo: {}", e),
    }

    let end_time = Utc::now();

    let duration = end_time - start_time;
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    let milliseconds = duration.num_milliseconds() % 1000;

    println!(
        "Tempo de execução: {:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    );
}

fn generate_billion_file(file_path: &str) -> io::Result<()> {
    let file = File::create(file_path).expect("Unable to create file");
    let mut writer = Writer::from_writer(file);

    writer.write_record(&["Nome", "Idade", "Data_Nascimento", "Altura", "Peso", "Genero"])?;

    let max_lines = 1_000_000_000;
    let batch_size = 100_000;
    let num_threads = 64;
    let lines_per_thread = max_lines / num_threads;

    let (sender, receiver) = mpsc::sync_channel(num_threads * 2);

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let sender = sender.clone();
        handles.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut batch = Vec::with_capacity(batch_size);

            for _ in 0..lines_per_thread {
                let name: String = Name().fake_with_rng(&mut rng);
                let age: u8 = (18..100).fake_with_rng(&mut rng);
                let birth_date = generate_birth_date(18..100);
                let height: f32 = rng.gen_range(1.5..2.0);
                let weight: f32 = rng.gen_range(50.0..120.0);
                let gender: char = if rng.gen_bool(0.5) { 'M' } else { 'F' };

                batch.push(vec![
                    name,
                    age.to_string(),
                    birth_date,
                    format!("{:.2}", height),
                    format!("{:.2}", weight),
                    gender.to_string(),
                ]);


                if batch.len() >= batch_size {
                    sender.send(batch).expect("Failed to send batch");
                    batch = Vec::with_capacity(batch_size);
                }
            }


            if !batch.is_empty() {
                sender.send(batch).expect("Failed to send final batch");
            }
        }));
    }


    drop(sender);

    for batch in receiver {
        for record in batch {
            writer.write_record(&record)?;
        }
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    writer.flush()?;
    println!("Geração de dados concluída! Arquivo salvo em {}", file_path);

    Ok(())
}

fn generate_birth_date(arg: std::ops::Range<i32>) -> String {
    let now = Utc::now();
    let years_ago = arg.end - arg.start;
    let random_days = rand::thread_rng().gen_range(0..(years_ago * 365));
    let birth_date = now - Duration::days(random_days as i64);

    birth_date.format("%Y-%m-%d").to_string()
}
