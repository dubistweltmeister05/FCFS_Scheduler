# FCFS Scheduler

A Rust implementation of the First-Come, First-Served (FCFS) CPU scheduling algorithm.

## Description

This project implements the First-Come, First-Served (FCFS) CPU scheduling algorithm, which is one of the simplest scheduling algorithms. In FCFS scheduling, processes are executed in the order they arrive in the ready queue. The process that arrives first gets the CPU first.

## Features

- Implements the FCFS scheduling algorithm
- Calculates and displays:
  - Process execution order
  - Start and finish times for each process
  - Average waiting time
  - Average turnaround time
- Handles processes with different arrival times

## Requirements

- Rust (2024 edition)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

```bash
git clone [your-repository-url]
cd fcfs_scheduler
```

2. Build the project:

```bash
cargo build
```

## Usage

Run the program using:

```bash
cargo run
```

The program will execute with a sample set of processes and display:

- The execution order of processes
- Start and finish times for each process
- Average waiting time
- Average turnaround time

## Project Structure

- `src/main.rs`: Contains the main implementation of the FCFS scheduler
  - `Process` struct: Represents a process with ID, arrival time, and burst time
  - `fcfs_scheduling` function: Implements the FCFS algorithm
  - Sample process data in the `main` function

## Example Output

```
Process Execution Order:
Process 1 starts at 0 and finishes at 5
Process 2 starts at 5 and finishes at 8
Process 3 starts at 8 and finishes at 16
Process 4 starts at 16 and finishes at 22

Average Waiting Time: 4.50
Average Turnaround Time: 9.50
```

## Contributing

Feel free to submit issues and enhancement requests!
