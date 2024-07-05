use std::cmp::max;

use crate::{
    code::{calc_max_line_number_length, calc_wh},
    config::TakeSnapshotParams,
};
use arboard::Clipboard;
use nvim_oxi::Result;

const SPACE_BOTH_SIDE: usize = 2;

fn optional(component: String, is_view: bool) -> String {
    if is_view {
        component
    } else {
        "".to_string()
    }
}

#[allow(dead_code)]
pub fn copy_ascii(params: TakeSnapshotParams) -> Result<()> {
    let (width, height) = calc_wh(&params.code, 1., 1.);
    let calc_line_number_width =
        |start_line_number: usize| calc_max_line_number_length(height as usize, start_line_number);
    let frame_width = max(width as usize, params.file_path.len()) + SPACE_BOTH_SIDE;
    let frame_width = match params.start_line_number {
        Some(start_line_number) => {
            frame_width + SPACE_BOTH_SIDE + calc_line_number_width(start_line_number)
        }
        None => frame_width,
    };
    let line = format!("│{}│\n", "─".repeat(frame_width));
    let frame_width_with_content = frame_width - 1;
    let top_frame = format!("╭{}╮\n", "─".repeat(frame_width));
    let bottom_frame = format!("╰{}╯", "─".repeat(frame_width));
    let code = params
        .code
        .lines()
        .enumerate()
        .map(|(i, line)| {
            format!(
                "│ {:1$} │\n",
                match params.start_line_number {
                    Some(start_line_number) => format!(
                        "{:1$} {line}",
                        start_line_number + i,
                        calc_line_number_width(start_line_number),
                    ),
                    None => line.to_string(),
                },
                frame_width_with_content - 1
            )
        })
        .collect::<String>();
    let text_line = |text: &str| format!("│ {:1$}│\n", text, frame_width_with_content);
    let breadcrumbs = optional(
        format!("{}{line}", text_line(&params.file_path)),
        params.has_breadcrumbs,
    );
    let ascii_snapshot = format!("{top_frame}{breadcrumbs}{code}{bottom_frame}");

    Clipboard::new().unwrap().set_text(ascii_snapshot).unwrap();

    Ok(())
}