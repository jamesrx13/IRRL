# Development Mode

## Start the Project
`cargo build`

## Create a New Recording
`cargo run -- record <recording_name>`

## Run a some record
`cargo run -- replay <recording_name>`

`cargo run -- replay <recording_name> <recording_index>`

- Press the ***Ctrl*** key if you need to make a some action 
- Press the ***Esc*** key if you need to stop the recording

# Run the Python Example

- Build a rust release, execute in the terminal `cargo build --release`
- You must have a recording of the page you want to automate. Copy the recording file into the ***python_example*** folder.

Edit the ***main.py*** file and update the following variables to match your environment:
- `MAIN_APP_URL`
- `PROFILE_PATH`
- `RUST_TOOL_PATH`

And run the example:
- `cd python_example`
- `python ./main.py`

---
## Autor

- **Nombre**: James Rudas
- **GitHub**: [jamesrx13](https://github.com/jamesrx13)
- **Email**: rudasmarinjf@gmail.com
- **Sitio web**: [jamesrudas.com](https://jamesrudas.com/)

