*Project developed in Rust to perform automations where cursor interaction is necessary.
It can be used to resolve actions marked with reCaptchaV3 or similar, as it reproduces previously recorded cursor movements and can have multiple recordings from multiple pages.*

# Development Mode

## Start the Project
`cargo build`

## Create a New Recording
`cargo run -- record <recording_name>`

- Press the ***Ctrl*** key if you need to make a some action 
- Press the ***Esc*** key if you need to stop the recording
  
## Run a some record
`cargo run -- replay <recording_name>`
`cargo run -- replay <recording_name> <recording_index>`

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
#### IRRL (Input Recording Replay Logger)
---
### Autor

- **Name**: James Rudas
- **GitHub**: [jamesrx13](https://github.com/jamesrx13)
- **Email**: rudasmarinjf@gmail.com
- **Web**: [jamesrudas.com](https://jamesrudas.com/)

