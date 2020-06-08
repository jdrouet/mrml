use bollard::container::{Stats, StatsOptions};
use bollard::Docker;
use byte_unit::{Byte, ByteUnit};
use futures_util::stream::TryStreamExt;
use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

fn prepare_data() -> PathBuf {
    let content = include_str!("../../test/mj-button.mjml");
    let encoded =
        "Content-Disposition: form-data; name=\"template\"; filename=\"template.mjml\";\r\n\r\n";
    let encoded = format!(
        "--1234567890\r\n{}{}\r\n--1234567890--\r\n",
        encoded, content
    );
    let mut filename = temp_dir();
    filename.push("form.data");
    let mut output = File::create(filename.clone()).unwrap();
    write!(output, "{}", encoded).unwrap();
    filename.clone()
}

fn get_cpu_percent(stat: &Stats) -> Option<f64> {
    let cpu_delta =
        (stat.cpu_stats.cpu_usage.total_usage - stat.precpu_stats.cpu_usage.total_usage) as f64;
    let system_delta = match stat.cpu_stats.system_cpu_usage.and_then(|notpre| {
        stat.precpu_stats
            .system_cpu_usage
            .and_then(move |pre| Some(notpre - pre))
    }) {
        Some(value) => value as f64,
        None => return None,
    };
    let online_cpus = match stat.cpu_stats.online_cpus.or_else(|| {
        stat.cpu_stats
            .cpu_usage
            .percpu_usage
            .as_ref()
            .and_then(|list| Some(list.len() as u64))
    }) {
        Some(value) => value as f64,
        None => return None,
    };

    if cpu_delta > 0.0 && system_delta > 0.0 {
        let cpu_usage = (cpu_delta / system_delta) * online_cpus * 100.0;
        Some(cpu_usage)
    } else {
        None
    }
}

fn get_memory_usage(stat: &Stats) -> Option<u64> {
    stat.memory_stats.usage.and_then(|value| {
        stat.memory_stats
            .stats
            .and_then(move |sub| Some(value - sub.cache))
    })
}

struct Event {
    pub cpu_percent: Option<f64>,
    pub memory_usage: Option<u64>,
}

impl From<Stats> for Event {
    fn from(stat: Stats) -> Event {
        Event {
            cpu_percent: get_cpu_percent(&stat),
            memory_usage: get_memory_usage(&stat),
        }
    }
}

struct Collector {
    container: String,
    data: Mutex<Vec<Event>>,
    running: Mutex<bool>,
}

impl Collector {
    pub fn new(container: &str) -> Arc<Collector> {
        Arc::new(Collector {
            container: container.into(),
            data: Mutex::new(vec![]),
            running: Mutex::new(false),
        })
    }

    fn set_running(&self, value: bool) {
        let mut running = self.running.lock().unwrap();
        *running = value;
    }

    fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    fn push_event(&self, event: Event) {
        let mut data = self.data.lock().unwrap();
        data.push(event);
    }

    pub fn start(current: Arc<Collector>) {
        std::thread::spawn(move || {
            let mut rt = Runtime::new().unwrap();
            rt.block_on(current.run());
        });
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    pub fn stop(current: Arc<Collector>) {
        println!("watcher stopping...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        current.set_running(false);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    async fn run(&self) {
        self.set_running(true);
        println!("watcher starting...");
        let docker = Docker::connect_with_local_defaults().unwrap();
        let mut stream = docker.stats(self.container.as_str(), Some(StatsOptions { stream: true }));
        println!("watcher started");
        loop {
            if !self.is_running() {
                break;
            }
            if let Ok(Some(stats)) = stream.try_next().await {
                self.push_event(Event::from(stats));
            }
        }
        println!("watcher stopped");
    }

    fn analyze_cpu(&self, data: &Vec<Event>) {
        let cpu_usages: Vec<f64> = data
            .iter()
            .filter(|item| item.cpu_percent.is_some())
            .map(|item| item.cpu_percent.unwrap())
            .collect();
        let min_cpu_usage =
            cpu_usages.iter().fold(
                std::f64::MAX,
                |res, value| if res < *value { res } else { *value },
            );
        let max_cpu_usage =
            cpu_usages.iter().fold(
                std::f64::MIN,
                |res, value| if res > *value { res } else { *value },
            );
        println!("cpu: {:.2}% < {:.2}%", min_cpu_usage, max_cpu_usage);
    }

    fn analyze_memory(&self, data: &Vec<Event>) {
        let memory_usages: Vec<u64> = data
            .iter()
            .filter(|item| item.memory_usage.is_some())
            .map(|item| item.memory_usage.unwrap())
            .collect();
        let min_memory_usage =
            memory_usages.iter().fold(
                std::u64::MAX,
                |res, value| if res < *value { res } else { *value },
            );
        let avg_memory_usage =
            memory_usages.iter().fold(0u64, |res, item| res + item) / (memory_usages.len() as u64);
        let max_memory_usage =
            memory_usages.iter().fold(
                std::u64::MIN,
                |res, value| if res > *value { res } else { *value },
            );
        let min_memory_usage = Byte::from_unit(min_memory_usage as f64, ByteUnit::B)
            .unwrap()
            .get_appropriate_unit(false);
        let avg_memory_usage = Byte::from_unit(avg_memory_usage as f64, ByteUnit::B)
            .unwrap()
            .get_appropriate_unit(false);
        let max_memory_usage = Byte::from_unit(max_memory_usage as f64, ByteUnit::B)
            .unwrap()
            .get_appropriate_unit(false);
        println!(
            "memory: {} < {} < {}",
            min_memory_usage.to_string(),
            avg_memory_usage.to_string(),
            max_memory_usage.to_string()
        );
    }

    fn analyze(&self) {
        println!("# result for {}", self.container);
        let data = self.data.lock().unwrap();
        self.analyze_cpu(&data);
        self.analyze_memory(&data);
    }
}

fn ab(port: u32, file: &PathBuf) -> std::process::Output {
    println!("benchmark starting...");
    let filename = file.as_path().to_str().unwrap();
    let url = format!("http://localhost:{}/render", port);
    let output = Command::new("ab")
        .args(&[
            "-n",
            "10000",
            "-c",
            "10",
            "-T",
            "multipart/form-data; boundary=1234567890",
            "-p",
            filename,
            url.as_str(),
        ])
        .output()
        .expect("failed to execute process");
    println!("benchmark done");
    output
}

fn main() {
    let form_file = prepare_data();
    println!("data file: {:?}", form_file);

    let collector = Collector::new("mjml-express");
    Collector::start(collector.clone());
    let output = ab(3000, &form_file);
    Collector::stop(collector.clone());
    collector.analyze();
    println!(
        "Apache benchmark result:\n{}\n",
        String::from_utf8(output.stdout).unwrap()
    );

    let collector = Collector::new("mrml-actix");
    Collector::start(collector.clone());
    let output = ab(3001, &form_file);
    Collector::stop(collector.clone());
    collector.analyze();
    println!(
        "Apache benchmark result:\n{}\n",
        String::from_utf8(output.stdout).unwrap()
    );
}
