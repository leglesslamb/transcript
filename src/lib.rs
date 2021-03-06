pub fn latin_to_elder_futhark(string: &str) -> String {
    string
        .to_uppercase()
        .replace("AE", "ᛇ")
        .replace("NG", "ᛜ")
        .replace("TH", "ᚦ")
        .replace("A", "ᚨ")
        .replace("B", "ᛒ")
        .replace("C", "ᚲ")
        .replace("D", "ᛞ")
        .replace("E", "ᛖ")
        .replace("F", "ᚠ")
        .replace("G", "ᚷ")
        .replace("H", "ᚺ")
        .replace("I", "ᛁ")
        .replace("J", "ᛃ")
        .replace("K", "ᚲ")
        .replace("L", "ᛚ")
        .replace("M", "ᛗ")
        .replace("N", "ᚾ")
        .replace("O", "ᛟ")
        .replace("P", "ᛈ")
        .replace("Q", "ᚲ")
        .replace("R", "ᚱ")
        .replace("S", "ᛊ")
        .replace("T", "ᛏ")
        .replace("U", "ᚢ")
        .replace("V", "ᚢ")
        .replace("W", "ᚹ")
        .replace("Y", "ᛁ")
        .replace("Z", "ᛉ")
}
