pub fn time_ago(seconds : u64) -> String {
    let mut seconds = seconds as f64;

    let days = (seconds / 86400.0).floor();
    if days > 31.0 { return "More than a month ago".to_owned(); }
    if days >= 1.0 { return format!("{}d ago", days); }
    
    seconds -= days * 86400.0;
    let hours = (seconds / 3600.0).floor();

    seconds -= hours * 3600.0;
    let minutes = (seconds / 60.0).floor();
    if hours > 1.0 { return format!("{}h {}m ago", hours, minutes); }
    
    seconds -= minutes * 60.0;
    return format!("{}m {}s ago", minutes, seconds);
}