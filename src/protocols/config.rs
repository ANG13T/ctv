#[derive(Debug, Serialize, Deserialize)]
struct Config {
    PIPE: String = "│",
    ELBOW: String = "└──",
    TEE: String = "├──",
    PIPE_PREFIX: String = "│   ",
    SPACE_PREFIX: String = "    "
}