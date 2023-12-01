use std::collections::HashMap;

pub fn get_dirsizes(input: &str) -> HashMap<String, u32> {
    let mut dir_stack: Vec<&str> = vec![];
    let mut dir_sizes: HashMap<String, u32> = HashMap::default();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            // directory change => push to stack or pop
            let dir = line.split_whitespace().nth(2).unwrap();
            if dir == ".." {
                dir_stack.pop();
            } else {
                dir_stack.push(dir);
            }
        } else if !line.starts_with('$') && !line.starts_with("dir ") {
            // file listing => parse size and add to all dirs in stack
            let size = line.split(' ').next().unwrap().parse::<u32>().unwrap();

            for index in 0..dir_stack.len() {
                let stack_slice = &dir_stack[0..index + 1];
                let dir_path = stack_slice.join("/");

                let current_size = dir_sizes.get(&dir_path).unwrap_or(&0u32);
                dir_sizes.insert(dir_path, size + current_size);
            }
        }
    }
    dir_sizes
}
