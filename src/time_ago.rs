pub fn time_ago(seconds : u64) -> String {
    let mut seconds = seconds;

    let days : u64 = (seconds as f64 / 86400.0).floor() as u64;
    if days > 31 { return "More than a month ago".to_owned(); }
    else if days >= 1 { return format!("{}d ago", days); }
    
    seconds -= days * 86400;
    let hours : u64 = (seconds as f64 / 3600.0).floor() as u64;

    seconds -= hours * 3600;
    let minutes : u64 = (seconds as f64 / 60.0).floor() as u64;
    if hours > 1 { return format!("{}h {}m ago", hours, minutes); }
    
    seconds -= minutes * 60;
    return format!("{}m {}s ago", minutes, seconds);
}