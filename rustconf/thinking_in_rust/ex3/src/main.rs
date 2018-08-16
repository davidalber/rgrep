// Remove the `expect`s from these two functions
fn read_config() -> ConfigFile {
    let file = read_file().expect("could not open file");

    write_to_log().expect("could not write to log file");
    file
}

impl Server {
    fn startup(self) -> ListeningServer {
        let config = read_config();
        let port = open_port().expect("could not open port");
        self.configure(config, port)
    }
}

fn main() {
    let server = Server::new();
    let server = server.startup();
    while let Some(_packet) = server.listen() {
        // ...
    }
}

// Helper functions and types, don't change these
fn write_to_log() -> Result<(), DiskError> { Ok(()) }
fn open_port() -> Result<Port, NetworkError> { Ok(Port) }
fn read_file() -> Result<ConfigFile, DiskError> { Ok(ConfigFile) }

struct Server;
struct Port;
#[derive(Debug)]
struct DiskError;
#[derive(Debug)]
struct NetworkError;
struct ConfigFile;
struct ListeningServer;

impl Server {
    fn new() -> Server { Server }
    fn configure(self, _c: ConfigFile, _p: Port) -> ListeningServer { ListeningServer }
}

impl ListeningServer {
    fn listen(&self) -> Option<()> { None }
}
