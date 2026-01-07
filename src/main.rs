use actix_files::{Files, NamedFile};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, get,
    web::{self, Data},
};
use clap::{Parser, Subcommand};
use futures::{StreamExt, lock::Mutex};
use notify::{Event, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    ops::Deref,
    path::Path,
    sync::{Arc, mpsc},
};
use tokio::sync::{broadcast, watch};

#[derive(Parser, Debug)]
#[command(name = "dev-server")]
#[command(about = "A simple static file server", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build the project without serving
    Build {
        /// Source directory
        #[arg(default_value = "./raw")]
        source: String,

        /// Output directory
        #[arg(short, long, default_value = "./dist")]
        output: String,
    },
    /// Serve files without hot reload
    Serve {
        /// Directory to serve
        #[arg(default_value = "main.mtx")]
        entry: String,

        #[arg(default_value = "./dist")]
        dir: String,
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Host to bind to
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,
    },
    /// Serve files with hot reload enabled
    Watch {
        /// Directory to serve and watch
        #[arg(default_value = "main.mtx")]
        entry: String,

        #[arg(short, long, value_name = "dir", default_value = "./dist")]
        dir: String,

        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Host to bind to
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,
    },
}

struct ReloadStatus {
    pub hot_reload_enable: bool,
    pub needs_reload: bool,
}

async fn needs_reload(data: web::Data<Mutex<ReloadStatus>>) -> impl Responder {
    let mut status = data.lock().await;
    let needs_reload = status.needs_reload;
    let hot_reload_enable = status.hot_reload_enable;

    if hot_reload_enable && needs_reload {
        println!("/status -> Forcing Page Reload!");
        status.needs_reload = false; // Reset after checking
    }

    HttpResponse::Ok().json(
        serde_json::json!({ "hot_reload_enabled": hot_reload_enable,  "reload": needs_reload }),
    )
}

async fn build_project(directory: &str, entry_path: &str) -> () {}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let (entr, build_directory, port, host, reload_enabled) = match args.command {
        Commands::Build { source, output } => {
            println!("🔨 Building project...");
            println!("📂 Source: {}", source);
            println!("📦 Output: {}", output);

            // TODO: Implement build logic here
            todo!("Build functionality not yet implemented");
        }
        Commands::Serve {
            entry,
            dir,
            port,
            host,
        } => (entry, dir, port, host, false),
        Commands::Watch {
            entry,
            dir,
            port,
            host,
        } => (entry, dir, port, host, true),
    };

    let bind_addr = format!("{}:{}", host, port);

    println!("🚀 Server running at http://{}:{}", host, port);
    println!(
        "📁 Serving Static: {}",
        std::fs::canonicalize(&build_directory)?.display()
    );

    if reload_enabled {
        println!("🔥 Hot reload: ENABLED");
    } else {
        println!("⚪ Hot reload: DISABLED");
    }

    let data = web::Data::new(Mutex::new(ReloadStatus {
        needs_reload: false,
        hot_reload_enable: reload_enabled,
    }));

    // Directory Watcher (only if reload is enabled)
    if reload_enabled {
        let dir_clone = build_directory.clone();
        let data_clone = data.clone();

        tokio::spawn(async move {
            use notify::Watcher;

            let (tx, mut rx) = tokio::sync::mpsc::channel(100);

            // Create the watcher with a closure that sends events to the channel
            let mut watcher =
                notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
                    if let Ok(event) = res {
                        let _ = tx.blocking_send(event);
                    }
                })
                .expect("Failed to create watcher");

            watcher
                .watch(Path::new(&dir_clone), RecursiveMode::Recursive)
                .expect("Failed to watch directory");

            println!("👀 Watcher started for: {}", &dir_clone);

            // Keep the watcher alive and process events
            while let Some(event) = rx.recv().await {
                let mut status = data_clone.lock().await;
                match event.kind {
                    notify::EventKind::Access(access_kind) => {
                        // DO NOTHING
                        println!("📝 File Accessed: {:?}", event.paths)
                    }
                    notify::EventKind::Create(create_kind) => {
                        // NEEDS_RELOAD

                        status.needs_reload = true;
                        println!("📝 File Created: {:?}... Needs Reload!", event.paths)
                    }
                    notify::EventKind::Modify(modify_kind) => {
                        // NEEDS_RELOAD
                        status.needs_reload = true;
                        println!("📝 File changed: {:?}... Needs Reload", event.paths)
                    }
                    notify::EventKind::Remove(remove_kind) => {
                        // DOESN'T NEED RELOAD
                        println!("📝 File removed: {:?}, recreating...", event.paths)
                    }
                    _ => (),
                };
            }
        });
    }
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/status", web::get().to(needs_reload))
            .service(
                Files::new("/", &build_directory)
                    .show_files_listing() // Optional: displays a directory listing
                    .index_file("index.html") // Optional: sets the default file for the directory
                    .use_last_modified(true), // Optional: uses Last-Modified header
            )
    })
    .bind(&bind_addr)?
    .run()
    .await
}
