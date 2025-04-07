use std::collections::VecDeque;

struct Process {
    id: usize,
    arrival_time: u32,
    burst_time: u32,
}

fn fcfs_scheduling(processes: Vec<Process>) {
    let mut processes = processes;
    processes.sort_by_key(|p| p.arrival_time);

    let mut current_time = 0;
    let mut waiting_time = 0;
    let mut turnaround_time = 0;

    println!("Process Execution Order:");
    for process in &processes {
        if current_time < process.arrival_time {
            current_time = process.arrival_time;
        }
        println!(
            "Process {} starts at {} and finishes at {}",
            process.id,
            current_time,
            current_time + process.burst_time
        );

        waiting_time += current_time - process.arrival_time;
        turnaround_time += (current_time + process.burst_time) - process.arrival_time;
        current_time += process.burst_time;
    }

    let n = processes.len() as f32;
    println!("\nAverage Waiting Time: {:.2}", waiting_time as f32 / n);
    println!("Average Turnaround Time: {:.2}", turnaround_time as f32 / n);
}

fn main() {
    let processes = vec![
        Process {
            id: 1,
            arrival_time: 0,
            burst_time: 5,
        },
        Process {
            id: 2,
            arrival_time: 1,
            burst_time: 3,
        },
        Process {
            id: 3,
            arrival_time: 2,
            burst_time: 8,
        },
        Process {
            id: 4,
            arrival_time: 3,
            burst_time: 6,
        },
    ];

    fcfs_scheduling(processes);
}
