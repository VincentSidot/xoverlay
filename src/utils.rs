use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ConnectionExt, Window as XWindow,
    }
};

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        _NET_WM_NAME,
        UTF8_STRING,
    }
}

/// Compute the [levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between two strings a and b   
fn compute_levensthein_distance_case_insensitive(a: &str, b: &str) -> usize {
    let a = a.to_ascii_lowercase();
    let b = b.to_ascii_lowercase();
    let a_len = a.chars().count();
    let b_len = b.chars().count();
    let mut dp = vec![vec![0; b_len + 1]; a_len + 1];
    // Intialize the first row and the first column
    for i in 0..=a_len {
        dp[i][0] = i;
    }
    for j in 0..=b_len {
        dp[0][j] = j;
    }
    // Compute the distance
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            dp[i + 1][j + 1] =  (dp[i][j+1] + 1)
                            .min(dp[i+1][j] + 1)
                            .min(dp[i][j] + cost);
        }
    }

    dp[a_len][b_len]
}

pub fn get_best_match<C>(conn: &C, root: u32, reference: &String) -> Result<Option<XWindow>, Box<dyn Error>>
where
    C: Connection,
{
    let mut best_match = None;
    let atoms = Atoms::new(conn)?.reply()?;


    match_for_childs(conn, root, &mut best_match, reference, &atoms)?;

    match best_match {
        Some((child, _, _, _)) => {
            Ok(Some(child))
        },
        None => Ok(None),
    }
}

fn match_for_childs<C>(conn: &C, root: u32, best_match: &mut Option<(u32, String, usize, bool)>, reference: &String, atoms: &Atoms) -> Result<(), Box<dyn Error>>
where
    C: Connection,
{
    if let Some(best) = best_match {
        if best.3 { // If we already found a perfect match, return
            return Ok(());
        }
    }
    let childs = conn.query_tree(root)?.reply()?;

    for child in childs.children {
        // Fetch window name
        let attr = conn.get_property(
            false,
            child,
            atoms._NET_WM_NAME,
            atoms.UTF8_STRING,
            0,
            u32::MAX,
        )?.reply()?;

        let name = String::from_utf8(attr.value)?;
        // If name is empty, skip
        if !name.is_empty() {
            let distance = compute_levensthein_distance_case_insensitive(&name, reference);

            if distance == 0 {
                *best_match = Some((child, name, distance, true));
                break;
            }
    
            match best_match {
                None => {
                    *best_match = Some((child, name, distance, false));
                }
                Some((_, _, best_distance, _)) => {
                    if distance < *best_distance {
                        *best_match = Some((child, name, distance, false));
                    }
                }
            }
        }

        match_for_childs(conn, child, best_match, reference, atoms)?;
    }

    Ok(())

}