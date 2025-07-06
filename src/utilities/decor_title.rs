use colored::*;

pub(crate) fn print_decor_title() {
    let title = r#"      :::::::::  :::::::::: ::::::::: 
     :+:    :+: :+:        :+:    :+: 
    +:+    +:+ +:+        +:+    +:+  
   +#++:++#:  +#++:++#   +#+    +:+   
  +#+    +#+ +#+        +#+    +#+    
 #+#    #+# #+#        #+#    #+#     
###    ### ########## #########      "#;

    let max_line_len = title.lines().map(|line| line.len()).max().unwrap_or(0);
    let border = "=".repeat(max_line_len + 4);

    println!("\n{}", border.red());
    for line in title.lines() {
        let formatted = format!("| {:<width$} |", line, width = max_line_len);
        println!("{}", formatted.red());
    }
    println!("{}", border.red());
}
