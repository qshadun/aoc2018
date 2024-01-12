use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug)]
enum Event{
    WakeUp,
    FallAsleep,
    ChangeGuard(u32),
}

#[derive(Debug)]
struct Log<'a> {
    day: &'a str,
    hour: u32,
    minute: u32,
    event: Event,
}

impl <'a> Log<'a>{
    fn from_str(line: &str) -> Log {
        let parts: Vec<_> = line.split(" ").collect();
        let day = parts[0];
        let time_parts: Vec<_> = parts[1].split(":").collect();
        let hour: u32 = time_parts[0].parse().unwrap();
        let minute = time_parts[1];
        let minute = &minute[..minute.len()-1];
        let minute: u32 = minute.parse().unwrap();
        let event = if parts[2] == "falls" {
            Event::FallAsleep
        } else if parts[2] == "wakes" {
            Event::WakeUp
        } else {
            let guard = &parts[3][1..];
            let guard: u32 = guard.parse().unwrap();
            Event::ChangeGuard(guard)
        };

        Log {
            day,
            hour,
            minute,
            event
        }
    }
}

fn main() {
    
    let input = read_to_string("inputs/input4.txt").unwrap();
    let mut logs: Vec<Log> = input.lines().map(Log::from_str).collect();
    logs.sort_by_key(|log| (log.day, log.hour, log.minute));
    let mut sleeping_times: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut cur_guard = 0;
    let mut sleep = 0;
    
    for log in logs {
        match log.event {
            Event::ChangeGuard(g) => cur_guard = g,
            Event::FallAsleep => sleep = log.minute,
            Event::WakeUp => {
                let wake = log.minute;
                let minutes = sleeping_times.entry(cur_guard).or_insert(vec![0; 60]);
                for m in sleep..wake {
                    minutes[m as usize] += 1;
                }
            }
        }
    }
    part1(&sleeping_times);
    part2(&sleeping_times);
}

fn part1(sleeping_times: &HashMap<u32, Vec<u32>>) {
    let mut most_sleep_guard: u32 = 0;
    let mut most_sleep_time: u32 = 0;
    for (guard, minutes) in sleeping_times {
        let total: u32 = minutes.iter().sum();
        if total > most_sleep_time {
            most_sleep_time = total;
            most_sleep_guard = *guard;
        }
    }

    let minutes = sleeping_times.get(&most_sleep_guard).unwrap();
    let most_frequent_minute = minutes.iter().enumerate().max_by(|(_, a), (_, b) | a.cmp(b)).map(|(index, _)| index).unwrap();
    let ans = most_frequent_minute * most_sleep_guard as usize;
    println!("{} {} {}", ans, most_sleep_guard, most_frequent_minute);
}

fn part2(sleeping_times: &HashMap<u32, Vec<u32>>) {
    let mut most_frequent_minute: usize = 0;
    let mut most_frequent_guard: u32 = 0;
    let mut most_frequent_times: u32 = 0;
    for (&guard, minutes) in sleeping_times {
        for (minute, &count) in minutes.iter().enumerate() {
            if count > most_frequent_times {
                most_frequent_times = count;
                most_frequent_guard = guard;
                most_frequent_minute = minute;
            }
        }
    }
    println!("{}", most_frequent_minute * most_frequent_guard as usize);
}


