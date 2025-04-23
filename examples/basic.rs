use colorful::*;

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
}

fn print_status(level: &str, color: Color, message: &str) {
    let prefix = Style::new()
        .fg(color)
        .bold()
        .paint(format!("[{}]", level));
    println!("{} {}", prefix, message);
}