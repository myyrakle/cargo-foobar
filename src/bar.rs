fn main() {
    std::fs::write("bar.txt", "barbarbar").expect("실패");
}
