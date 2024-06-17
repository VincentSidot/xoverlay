use std::{env, error::Error};

use x11rb::{connection::Connection, protocol::xproto::{Atom, AtomEnum, ConnectionExt}, rust_connection::RustConnection};
use xoverlay::x11rb;

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        _NET_WM_NAME,
        UTF8_STRING,
    }
}

fn match_for_childs(conn: &RustConnection, root: u32, best_match: &mut Option<(u32, String, usize)>, reference: &String) -> Result<(), Box<dyn Error>> {
    let childs = conn.query_tree(root)?.reply()?;

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

        let distance = compute_levensthein_distance_case_insensitive(&name, reference);

        match best_match {
            None => {
                *best_match = Some((child, name, distance));
            }
            Some((_, _, best_distance)) => {
                if distance < *best_distance {
                    *best_match = Some((child, name, distance));
                }
            }
        }

        match_for_childs(conn, child, best_match, reference)?;
    }

    Ok(())

}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <window name>", args[0]);
        return Err("No window provided")?;
    }

    let window_name = &args[1];

    let (conn, screen_num) = x11rb::connect(None)?;

    let root = (&conn).setup().roots[screen_num].root;

    let mut best_match = None;

    match_for_childs(&conn, root, &mut best_match, window_name)?;

    if let Some((child, name, score)) = best_match {
        println!("Found window: {:#x} with name: {} (Score: {})", child, name, score);
    } else {
        println!("No window found");
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