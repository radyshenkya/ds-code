mod languages;
use std::sync::{Arc, Mutex};

use derive_more::{Display, Error};
use docker_api::{opts::ContainerCreateOpts, Docker};
use futures::StreamExt;
use languages::USER_CODE_LANGUAGES_EXEC_ARGS;

const DOCKER_IMAGE_NAME: &str = "ds-code-user-code";

#[derive(Debug, Display, Error, Clone)]
pub struct RunningError {
    pub msg: String,
}

pub async fn run_user_code(
    docker: &Docker,
    user_code: &str,
    user_lang: &str,
) -> Result<String, RunningError> {
    // Getting code land
    let code_lang = USER_CODE_LANGUAGES_EXEC_ARGS.get(user_lang);

    if code_lang.is_none() {
        return Err(RunningError {
            msg: "Unknown language".to_string(),
        });
    }

    let code_lang = code_lang.unwrap();

    let exec_args: Vec<_> = code_lang.0.split(" ").collect();
    let file_name = code_lang.1;

    // Creating container
    let containers = docker.containers();

    let new_container = containers
        .create(
            &ContainerCreateOpts::builder()
                .image(DOCKER_IMAGE_NAME)
                .auto_remove(true)
                .attach_stderr(true)
                .attach_stdout(true)
                .network_mode("none")
                .command(exec_args)
                .build(),
        )
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;

    // Adding user code to container
    new_container
        .copy_file_into(file_name, user_code.as_bytes())
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

    let _ = new_container.delete().await;
    let locked_output = output.lock().unwrap();
    let output = String::from(locked_output.as_str());
    Ok(output)
}
