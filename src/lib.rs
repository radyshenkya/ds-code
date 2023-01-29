use std::sync::{Arc, Mutex};

use derive_more::{Display, Error};
use docker_api::{
    opts::{ContainerConnectionOpts, ContainerCreateOpts, NetworkCreateOpts, NetworkListOpts},
    Docker, Network,
};
use futures::StreamExt;
use phf::phf_map;

const DOCKER_IMAGE_NAME: &str = "ds-code-user-code";
const DOCKER_NETWORK_NAME: &str = "ds-code-user-code-net";

const USER_CODE_LANGUAGES_EXEC_ARGS: phf::Map<&'static str, (&'static str, &'static str)> = phf_map! {
    // "LANG_CODE" => ("EXEC FUNCS", "NAME_FILE")
    "python" => ("timeout 2s /usr/bin/python3 /user_code.py", "/user_code.py"),
    "py" => ("timeout 2s /usr/bin/python3 /user_code.py", "/user_code.py"),
    "rust" => ("/scripts/run_rust.sh", "/user_code.rs"),
    "rs" => ("/scripts/run_rust.sh", "/user_code.rs"),
    "javascript" => ("timeout 2s /usr/bin/node /user_code.js", "/user_code.js"),
    "js" => ("timeout 2s /usr/bin/node /user_code.js", "/user_code.js"),
    "c" => ("/scripts/run_c.sh", "/user_code.c"),
    "cpp" => ("/scripts/run_cpp.sh", "/user_code.cpp")
};

#[derive(Debug, Display, Error)]
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

    let locked_output = output.lock().unwrap();
    let output = String::from(locked_output.as_str());
    Ok(output)
}

async fn get_usercode_network(docker: &Docker) -> Result<Network, RunningError> {
    let networks = docker
        .networks()
        .list(&NetworkListOpts::builder().build())
        .await
        .map_err(|e| RunningError { msg: e.to_string() })?;

    // Searching network
    let network = networks
        .into_iter()
        .find(|network| network.name == Some(String::from(DOCKER_NETWORK_NAME)));

    if network.is_none() {
        // Creating new network
        let new_network = docker
            .networks()
            .create(
                &NetworkCreateOpts::builder(String::from(DOCKER_NETWORK_NAME))
                    .attachable(true)
                    .internal(true)
                    .build(),
            )
            .await
            .map_err(|e| RunningError { msg: e.to_string() })?;

        return Ok(new_network);
    }
    let network = network.unwrap();
    let network = docker.networks().get(network.id.unwrap());

    Ok(network)
}
