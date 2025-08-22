// to run the native macos code via cli for debugging purposes.

fn main() {
    println!("screen_info");
    let screen_info = lib_swift::get_screen_info();
    println!("Screen Info: {:?}", screen_info);

    println!("network_info");
    let network_info = lib_swift::get_network_info();
    println!("Network Info: {:?}", network_info);
}
