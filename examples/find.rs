use std::{env, error::Error};

use x11rb::{connection::Connection, protocol::xproto::{AtomEnum, ConnectionExt}};
use xoverlay::x11rb;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <window name>", args[0]);
        return Err("No window provided")?;
    }

    let window_name = &args[1];

    let (conn, screen_num) = x11rb::connect(None)?;

    let root = (&conn).setup().roots[screen_num].root;

    let childs = conn.query_tree(root)?.reply()?;

    let mut childs_formated = vec![];

    for child in childs.children {
        // Fetch window name
        let attr = conn.get_property(
            false,
            child,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            0,
            1024,
        )?.reply()?;

        let name = String::from_utf8(attr.value)?;

        childs_formated.push((child, name));
        
    }

    let len = childs_formated.len();
    childs_formated.sort_by(|a, b| {
        let score_a = compute_levensthein_distance_case_insensitive(&a.1, window_name);
        let score_b = compute_levensthein_distance_case_insensitive(&b.1, window_name);
        score_a.cmp(&score_b)
    });

    for (i, (child, name)) in childs_formated.iter().enumerate() {
        let score = compute_levensthein_distance_case_insensitive(&name, window_name);
        println!("[{}/{}] Window: {:#x} - {} ({})",i, len, child, name, score);
    }


    Ok(())
}

/// Compute the [levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between two strings a and b   
fn compute_levensthein_distance_case_insensitive(a: &str, b: &str) -> usize {
    let a = a.to_lowercase();
    let b = b.to_lowercase();
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    // Intialize the first row and the first column
    for i in 0..=a.len() {
        dp[i][0] = i;
    }
    for j in 0..=b.len() {
        dp[0][j] = j;
    }
    // Compute the distance
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            dp[i + 1][j + 1] = (dp[i][j+1] + 1).min(dp[i+1][j] + 1).min(dp[i][j] + cost);
        }
    }
    dp[a.len()][b.len()]
}