use docker_api::Docker;
use ds_code::run_py_script_in_container;
use std::env::var;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let docker = Docker::new(var("DOCKER_HOST")?)?;
    let python_script = "print(\"Hello World!\")
i = 0
while True:
    print(i)
    i += 1";
    
    println!(
        "{}",
        run_py_script_in_container(&docker, python_script).await?
    );

    Ok(())
}
