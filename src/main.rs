use tokio::time::Duration;
use std::process;

use windows::{
    core::*, Win32::{System::Com::*, UI::{Accessibility::*, WindowsAndMessaging::*}},
};
use clap::Parser;
use log::{error, info};

use anyhow::Result;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to output
    #[arg(short, long)]
    file: String,

    /// interval of seconds for one cycle
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=60))]
    interval: u8,
}

struct Engine {
    automation: IUIAutomation,
    condition: IUIAutomationCondition,
    sfilename: String,
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe { CoUninitialize(); }
    }
}

impl Engine {    
    fn new(sfilename: &str) -> Self {
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok().expect("Failed to initialize Windows COM."); }

        let automation: IUIAutomation = unsafe { 
            CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL).expect("Failed to initialize Windows Accessibility API.")
        };
        let condition = unsafe { automation.CreatePropertyCondition(UIA_AutomationIdPropertyId, &VARIANT::from("CaptionsTextBlock")).unwrap() };
        Self {
            automation,
            condition,
            sfilename: sfilename.to_string(),
        }
    }

    fn get_livecaptions(&self) -> Result<String> {
        let window = unsafe { FindWindowW(w!("LiveCaptionsDesktopWindow"), None) };
        let element = unsafe { self.automation.ElementFromHandle(window) }?;
        let text = unsafe { element.FindFirst(TreeScope_Descendants, &self.condition) }?;
        let text = unsafe { text.CurrentName() }?;
        Ok(text.to_string())
    }

    fn save_current_captions(&self, current: &str) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        // Open the file and truncate (clear) its content before writing the new content
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.sfilename)?;

        file.write_all(current.as_bytes())?;
        file.write(b"\n")?;
        Ok(())
    }

    fn graceful_shutdown(&self) -> Result<()> {
        let text = self.get_livecaptions()?;
        self.save_current_captions(&text)?;
        Ok(())
    }
}

fn is_livecaptions_running() -> bool {   
    return unsafe { FindWindowW(w!("LiveCaptionsDesktopWindow"), None).0 } != 0;
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    info!("get-livecaptions running.");

    if !is_livecaptions_running() {
        error!("livecaptions is not running. program exiting.");
        return;
    }

    let engine = Engine::new(&args.file);

    let mut windows_timer = tokio::time::interval(Duration::from_secs(10)); // Check if live captions is running every 10 seconds
    let mut writefile_timer = tokio::time::interval(Duration::from_secs(args.interval as u64)); // Use the interval in seconds

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    println!("get-livecaptions is running now, and saving content into '{}', every {} seconds. Press Ctrl+C to exit.", args.file, args.interval);
    loop {
        tokio::select! {
            _ = windows_timer.tick() => {
                log::info!("running check every 10s.");
                if !is_livecaptions_running() {
                    println!("livecaptions is not running. program exiting.");
                    let _ = engine.graceful_shutdown();
                    process::exit(0);
                }
            },
            _ = writefile_timer.tick() => {
                log::info!("save content into file every {} seconds.", args.interval);
                let text = engine.get_livecaptions();
                if let Ok(text) = text {
                    engine.save_current_captions(&text).expect("Failed to save file.");
                }
            },
            _ = &mut ctrl_c => {
                let _ = engine.graceful_shutdown();
                process::exit(0);
            }
        };
    };
}
