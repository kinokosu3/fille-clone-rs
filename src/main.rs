use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use notify::Event;
use tokio::sync::{Barrier, mpsc};

type FileMap = Arc<Mutex<HashMap<String, usize>>>;


type EventSender = mpsc::UnboundedSender<Event>;
type EventReceiver = mpsc::UnboundedReceiver<Event>;

#[tokio::main]
async fn main() {
    let file_map: FileMap = Arc::new(Mutex::new(HashMap::new()));
    // 创建一个同步信号
    let barrier = Arc::new(Barrier::new(2));
    // 创建事件通道
    let (tx, rx): (EventSender, EventReceiver) = mpsc::unbounded_channel();


    let barrier_clone = Arc::clone(&barrier);
    let file_map_clone = Arc::clone(&file_map);

    // 监控路径和副本路径
    let monitor_path = "path/to/monitor";
    let backup_path = "path/to/backup";


    tokio::spawn(async move { monitor_directories().await; });
}

async fn monitor_directories(tx: EventReceiver) {
    let mut watcher = notify::recommended_watcher()
}