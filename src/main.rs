use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};

fn r#in(command_args: Vec<String>) {
    let name = &command_args[1].replace(".clock", "");
    let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .append(true)
                    .open(format!("{}.clock", name))
                    .unwrap();
    print!("{name}");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    file.write_all(format!("{now:?},").as_bytes()).unwrap();
}

fn out(command_args: Vec<String>) {
    let name = &command_args[1].replace(".clock", "");
    let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .append(true)
                    .open(format!("{}.clock", name))
                    .unwrap();
    print!("{name}");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    file.write_all(format!("{now:?}\n").as_bytes()).unwrap();
}

fn register(command_args: Vec<String>) {
    let name = &command_args[1].replace(".clock", "");
    File::create(format!("{name}.clock")).unwrap();
}

fn get_hours(command_args: Vec<String>) {
    let name = &command_args[1].replace(".clock", "");

    let mut file = File::open(format!("{name}.clock")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // ["times,times", "times,times"]
    let mut split1 = contents.clone().split("\n").collect::<Vec<&str>>().into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
    split1.remove(split1.len()-1);
    // ["time,time", "time,time"]
    let split2 = split1.into_iter().map(|x| {
        x.to_string().replace("s", "")
    }).collect::<Vec<String>>();
    // [["time", "time"], ["time", "time"]]
    let split3 = split2.into_iter().map(|x| {
        x.clone().split(",").collect::<Vec<&str>>().clone().into_iter().map(|x| {
            x.to_string()
        }).clone().collect::<Vec<String>>()
    }).collect::<Vec<Vec<String>>>();
    // [[[sec, nano], [sec, nano]], [[sec, nano], [sec, nano]]]
    let split4 = split3.into_iter().map(|x| {
        x.into_iter().map(|y| {
            y.split(".").collect::<Vec<&str>>().into_iter().map(|z| z.to_string()).collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>()
    }).collect::<Vec<Vec<Vec<String>>>>();
    // [Duration, Duration]
    let split5 = split4.into_iter().map(|x| {
        let fst = Duration::new(x[0][0].parse().unwrap(), x[0][1].parse().unwrap());
        let snd = Duration::new(x[1][0].parse().unwrap(), x[1][1].parse().unwrap());
        snd - fst
    }).collect::<Vec<Duration>>();
    let time: Duration = split5.into_iter().sum();
    let humantime: humantime::Duration = time.into(); 
    println!("Time Spent: {humantime}");
}

fn main() {
    let command_args = env::args().collect::<Vec<String>>()[1..].to_vec();
    match command_args[0].as_str() {
        "in" => r#in(command_args),
        "out" => out(command_args),
        "register" => register(command_args),
        "get" => get_hours(command_args),
        _ => ()
    }
}
