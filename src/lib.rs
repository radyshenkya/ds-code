use std::sync::{Arc, Mutex};

use derive_more::{Display, Error};
use docker_api::{opts::ContainerCreateOpts, Docker};
use futures::StreamExt;

const DOCKER_IMAGE_NAME: &str = "python:slim-bullseye";
const COMMAND_RUN_TIMEOUT: &str = "3s";

#[derive(Debug, Display, Error)]
pub struct RunningError {
    pub msg: String,
}

pub async fn run_command_in_container(
    docker: &Docker,
    command: &[&str],
) -> Result<(), RunningError> {
    let containers = docker.containers();

    let new_container = containers
        .create(
            &ContainerCreateOpts::builder()
                .image(DOCKER_IMAGE_NAME)
                .auto_remove(true)
                .attach_stderr(true)
                .attach_stdout(true)
                .command(command)
                .build(),
        )
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;

    let mut stream = new_container
        .attach()
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;
    new_container
        .start()
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;

    while let Some(item) = stream.next().await {
        let item = item.map_err(|e| RunningError { msg: e.to_string() })?;
        match item {
            docker_api::conn::TtyChunk::StdIn(_) => todo!(),
            docker_api::conn::TtyChunk::StdOut(out) => println!(
                "{}",
                String::from_utf8(out).map_err(|e| RunningError { msg: e.to_string() })?
            ),
            docker_api::conn::TtyChunk::StdErr(out) => println!(
                "{}",
                String::from_utf8(out).map_err(|e| RunningError { msg: e.to_string() })?
            ),
        }
    }

    Ok(())
}

const USER_PYTHON_CODE_PATH: &str = "/user_script.py";
pub async fn run_py_script_in_container(
    docker: &Docker,
    script: &str,
) -> Result<String, RunningError> {
    // Creating container
    let containers = docker.containers();

    let new_container = containers
        .create(
            &ContainerCreateOpts::builder()
                .image(DOCKER_IMAGE_NAME)
                .auto_remove(true)
                .attach_stderr(true)
                .attach_stdout(true)
                .command([
                    "timeout",
                    COMMAND_RUN_TIMEOUT,
                    "python3",
                    USER_PYTHON_CODE_PATH,
                ])
                .build(),
        )
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;

    // Adding user code to container
    new_container
        .copy_file_into(USER_PYTHON_CODE_PATH, script.as_bytes())
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;

    // Starting execution
    let mut stream = new_container
        .attach()
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;
    new_container
        .start()
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;

    // TODO: Sending input

    // Collection output
    let output = Arc::new(Mutex::new(String::new()));

    while let Some(item) = stream.next().await {
        let item = item.map_err(|e| RunningError { msg: e.to_string() })?;
        let mut output_lock = output.lock().unwrap();

        match item {
            docker_api::conn::TtyChunk::StdIn(_) => todo!(),
            docker_api::conn::TtyChunk::StdOut(out) => {
                output_lock.push_str(
                    String::from_utf8(out)
                        .map_err(|e| RunningError { msg: e.to_string() })?
                        .as_str(),
                );
            }
            docker_api::conn::TtyChunk::StdErr(out) => {
                output_lock.push_str(
                    String::from_utf8(out)
                        .map_err(|e| RunningError { msg: e.to_string() })?
                        .as_str(),
                );
            }
        };
    }

    let locked_output = output.lock().unwrap();
    let output = String::from(locked_output.as_str());
    Ok(output)
}
