# Get Live Captions - Rust

This Rust application captures live captions from Windows 11's Live Caption feature in real-time and saves the content to a specified text file. Additionally, it integrates with Slint for a user-friendly UI, providing real-time interaction and enhanced control over the captioning process. The application also includes OpenAI integration for intelligent querying based on the captured text.

Inspired by [corbamico/get-livecaptions-rs](https://github.com/corbamico/get-livecaptions-rs).

## Features

- **Real-Time Caption Capture**: Capture live captions from Windows 11 Live Caption in real-time.
- **Text File Export**: Save the captured captions to a text file at regular intervals.
- **User Interface**: Display live captions in a Slint-based UI with the ability to copy the text.
- **Control Buttons**: Easily start and stop the captioning process with intuitive buttons.
- **OpenAI Integration**: Query OpenAI with the captured text and receive streamed responses in the UI.
- **Custom Queries**: Input custom text queries for OpenAI and display the responses directly in the UI.
- **Pretraining for Interviews**: Pretrain the system with job descriptions and resumes, with options to select the type of interview (intro, technical, or cultural).

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


## TODO List

- [ ]  **Integrate with Slint**: Display live captions in a UI that allows text to be copied.
- [ ]  **Add Control Buttons**: Implement start and stop buttons for captioning.
- [ ]  **OpenAI Integration**: Integrate OpenAI with buttons to start/stop captioning and querying.
- [ ]  **Save Captions**: Continue saving live captions to a text file.
- [ ]  **Text Input for OpenAI**: Add an input field to send custom text to OpenAI and display responses.
- [ ]  **Pretrain with Job Description and Resume**: Integrate a feature to pretrain with the job description and resume, and allow selecting the type of call (intro, tech, culture).