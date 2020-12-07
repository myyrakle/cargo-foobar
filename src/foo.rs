fn main() {
    std::fs::write("foo.txt", "foofoofoo").expect("실패");
}
