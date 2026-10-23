fn main() {
    slint_build::compile("ui/HomeScreen.slint").expect("Slint build failed");
    
    // Tell cargo to rerun if UI files change
    println!("cargo:rerun-if-changed=ui/HomeScreen.slint");
    println!("cargo:rerun-if-changed=ui/Tile.slint");
    println!("cargo:rerun-if-changed=design-tokens.json");
}