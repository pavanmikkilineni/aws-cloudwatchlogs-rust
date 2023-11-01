extern crate chrono;

use chrono::Utc;
use aws_sdk_cloudwatchlogs as cloudwatchlogs;
use cloudwatchlogs::types::InputLogEvent;
use std::process::*;

#[::tokio::main]
async fn main() -> Result<(), cloudwatchlogs::Error> {
    // Load AWS configuration from environment variables
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_cloudwatchlogs::Client::new(&config);

    // Define constants for the log group and log stream names
    const DISTI_LOG_GROUP_NAME: &str = "DistiLogs";
    const DISTI_LOG_STREAM_NAME: &str = "ApplicationLogs";

    // Create or retrieve the log group, and add tags
    let disti_log_group = client
        .create_log_group()
        .log_group_name(DISTI_LOG_GROUP_NAME)
        .tags("Environment", "Test");
    
    // Send the request to create the log group
    match disti_log_group.send().await {
        Ok(response) => {
            println!("Log group created or already exists: {:?}", response);
        }
        Err(err) => {
            println!("Error creating log group: {:?}", err);
            exit(1);
        }
    };

    // Create or retrieve the log stream
    let disti_log_stream = client
        .create_log_stream()
        .log_group_name(DISTI_LOG_GROUP_NAME)
        .log_stream_name(DISTI_LOG_STREAM_NAME);

    // Send the request to create the log stream
    match disti_log_stream.send().await {
        Ok(response) => {
            println!("Log stream created or already exists: {:?}", response);
        }
        Err(err) => {
            println!("Error creating log stream: {:?}", err);
            exit(1);
        }
    }

    // Create a log event
    let disti_log_event = InputLogEvent::builder()
        .timestamp(Utc::now().timestamp_millis())
        .message("Hello, World!")
        .build();

    // Put the log event into the log stream
    let disti_put_log_event = client.put_log_events()
        .log_group_name(DISTI_LOG_GROUP_NAME)
        .log_stream_name(DISTI_LOG_STREAM_NAME)
        .log_events(disti_log_event);

    // Send the request to put the log event
    match disti_put_log_event.send().await {
        Ok(response) => {
            println!("Log event sent successfully: {:?}", response);
        }
        Err(err) => {
            println!("Error sending log event: {:?}", err);
            exit(1);
        }
    }

    Ok(())
}



