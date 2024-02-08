pub fn strip_ansi(input: String) -> String {
    input
        .replace("\x1B[91m", "")
        .replace("\x1B[32m", "")
        .replace("\x1B[0m", "")
}