# Get Live Captions - Rust

This Rust application captures live captions from Windows 11's Live Caption feature in real-time and saves the content to a specified text file. The application uses Windows Accessibility API, Tokio for asynchronous operations, and Clap for command-line argument parsing.

Inspired by [corbamico/get-livecaptions-rs](https://github.com/corbamico/get-livecaptions-rs).

## Features

- Capture live captions from Windows 11 Live Caption in real-time.
- Save the captured captions to a text file at regular intervals.
- Automatically handle graceful shutdowns, ensuring the last caption is saved.

## Prerequisites

- Rust programming language installed on your machine.
- Windows 11 with Live Caption enabled.

## Usage

```
cmd

Usage: get-livecaptions.exe [OPTIONS] --file <FILE>  
Options: 
	-f, --file <FILE>          Name of the file to output   
	-i, --interval <INTERVAL>  Interval of seconds for one cycle [default: 1]   
	-h, --help                 Print help information   
	-V, --version              Print version information
```

## Example Usage

Run the application using Cargo:

```
bash

cargo run -- --file output.txt --interval 1
```

This command will start the application, which will capture the live captions every second and save them into `output.txt`. The program checks every 10 seconds to ensure the Live Caption window is still running. Press `Ctrl+C` to gracefully shut down the application.