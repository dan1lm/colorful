use whitelight::*;

fn main() {
    println!("{}", red("This text is red!"));
    println!("{}", green("This text is green!"));
    
    println!("{}", "This text is blue!".blue());
    println!("{}", "This text is yellow!".yellow());
    
    println!("{}", "Bold cyan text".cyan().bold());
    println!("{}", Style::new()
        .fg(Color::Magenta)
        .bg(Color::BrightBlack)
        .italic()
        .paint("Italic magenta text on gray background"));
    
    let error_prefix = "ERROR:".red().bold();
    println!("{} {}", error_prefix, "Something went wrong!".yellow());
    
    let success_prefix = "SUCCESS:".green().bold();
    println!("{} {}", success_prefix, "Operation completed!".green());
    
    print_status("INFO", Color::Blue, "Loading configuration...");
    print_status("WARNING", Color::Yellow, "Disk space is running low");
    print_status("ERROR", Color::Red, "Failed to connect to server");
    print_status("SUCCESS", Color::Green, "Data saved successfully");


    // RGB colors example
    println!("{}", "Deep orange text".rgb(255, 100, 0));
    println!("{}", "Light blue background".on_rgb(100, 200, 255));

    println!("{}", "Custom orange".rgb(255, 165, 0));
    println!("{}", "Deep blue background".on_rgb(0, 0, 128));

    // Combining RGB foreground and background
    println!("{}", Style::new()
    .fg_rgb(255, 50, 50)
    .bg_rgb(20, 20, 50)
    .bold()
    .paint("RGB styling"));

    println!("{}", Style::new()
    .fg_rgb(220, 20, 60)  
    .bg_rgb(25, 25, 25)   
    .bold()
    .italic()
    .paint("Styled RGB text"));
}

fn print_status(level: &str, color: Color, message: &str) {
    let prefix = Style::new()
        .fg(color)
        .bold()
        .paint(format!("[{}]", level));
    println!("{} {}", prefix, message);
}