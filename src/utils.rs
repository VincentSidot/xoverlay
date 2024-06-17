use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{
        AtomEnum,
        ConnectionExt, Window as XWindow,
    }
};

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

pub fn get_best_match<C>(conn: &C, root: u32, reference: &String) -> Result<Option<XWindow>, Box<dyn Error>>
where
    C: Connection,
{
    let mut best_match = None;

    match_for_childs(conn, root, &mut best_match, reference)?;

    match best_match {
        Some((child, _, _)) => Ok(Some(child)),
        None => Ok(None),
    }
}

fn match_for_childs<C>(conn: &C, root: u32, best_match: &mut Option<(u32, String, usize)>, reference: &String) -> Result<(), Box<dyn Error>>
where
    C: Connection,
{
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