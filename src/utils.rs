//! Utility functions for the overlay library
//! 
//! This module contains utility functions used by the overlay library
//! 
//! # Further optimizations
//! 
//! The current implementation of the levenshtein distance algorithm is not optimized.
//!     - The space complexity is len(a) * len(b)
//!     - The algorithm is working with multi-byte characters
//! 
//! The current window search algorithm is not optimized.
//!     - The algorithm is recursive
//!     - It may be parallelized to speed up the search
//!     - I could also define a minimum distance to stop the search (currently only exact match will stop the search)

use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ConnectionExt, Window as XWindow,
    }
};

x11rb::atom_manager! {
    /// Atoms used to fetch the window name
    /// 
    /// The atoms are used to fetch the window name.
    /// * _NET_WM_NAME: The window name
    /// * UTF8_STRING: The string encoding
    pub Atoms: AtomsCookie {
        _NET_WM_NAME,
        UTF8_STRING,
    }
}

/// Compute the [levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between two strings a and b
/// The algorithm may be optimized (space complexity is len(a) * len(b)).
/// The algorithm is working with multi-byte characters.
/// 
/// # Arguments
/// 
/// * `a` - The first string
/// * `b` - The second string
/// 
/// # Returns
/// 
/// The function returns the levenshtein distance between the two strings.
/// 
fn compute_levensthein_distance_case_insensitive(a: &str, b: &str) -> usize {
    let a = a.to_lowercase();
    let b = b.to_lowercase();
    let a_len = a.chars().count(); // Number of characters in a string (multi-byte characters are counted as one character)
    let b_len = b.chars().count(); // Number of characters in a string (multi-byte characters are counted as one character)
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

/// Get the best match for a window name
/// 
/// This function will search for the best match for a window name in the window tree.
/// It is a recursive function that will search for the best match in the children of the root window.
/// 
/// # Arguments
/// 
/// * `conn` - The X11 connection
/// * `root` - The root window
/// * `reference` - The reference string to match
/// 
/// # Returns
/// 
/// The function returns the best match for the reference string in the window tree.
/// If no match is found, the function will return None.
/// 
/// # Errors
/// 
/// The function may return an error if the X11 connection is not valid.
/// Or if the window tree cannot be fetched.
/// 
pub fn find_window_by_name<C>(conn: &C, root: u32, reference: &str) -> Result<Option<XWindow>, Box<dyn Error>>
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

/// Match for childs
/// 
/// This function will search for the best match in the children of a window.
/// This is the inner recursive function used by get_best_match.
/// 
/// # Arguments
/// 
/// * `conn` - The X11 connection
/// * `root` - The root window
/// * `best_match` - The best match found so far. The tuple contains:
///     - The window id
///     - The window name
///     - The distance between the window name and the reference string
///     - A boolean indicating if the match is perfect (use for early return) 
/// * `reference` - The reference string to match
/// * `atoms` - The atoms used to fetch the window name
/// 
/// # Returns
/// 
/// The function does not return anything.
/// It will update the best_match tuple with the best match found in the children of the root window.
/// 
/// # Errors
/// 
/// The function may return an error if the X11 connection is not valid.
/// Or if the window tree cannot be fetched.
/// 
fn match_for_childs<C>(conn: &C, root: u32, best_match: &mut Option<(u32, String, usize, bool)>, reference: &str, atoms: &Atoms) -> Result<(), Box<dyn Error>>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_levensthein_distance_case_insensitive() {
        assert_eq!(compute_levensthein_distance_case_insensitive("hello", "hello"), 0);
        assert_eq!(compute_levensthein_distance_case_insensitive("HeLLo", "hello"), 0);
        assert_eq!(compute_levensthein_distance_case_insensitive("hello", "world"), 4);
        assert_eq!(compute_levensthein_distance_case_insensitive("hello", "hella"), 1);
        assert_eq!(compute_levensthein_distance_case_insensitive("hello", "hallo"), 1);
        assert_eq!(compute_levensthein_distance_case_insensitive("hello", "holle"), 2);
        assert_eq!(compute_levensthein_distance_case_insensitive("hello", "h"), 4);
    }
}