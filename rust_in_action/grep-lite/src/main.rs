mod numeric_literals;

fn main() {
    let context_lines = 2;
    let search_term = "picture";
    let quote = "
    every day is a
    new
    day
    every face is
    a 
    picture,
    every food
    is 
    yum";

    // hold line numbers of matches
    let mut tags: Vec<usize> = Vec::new();
    // hold context lines for each match
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    for (idx, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            tags.push(idx);

            let v = Vec::with_capacity(1 + 2 * context_lines);
            ctx.push(v);
        }
    }

    if tags.len() == 0 {
        return;
    }

    for (i, line) in quote.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            // subtraction that returns 0 on integer underflow
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
    // println!("{}: {}", line_num, line);
}
