//! Unified UI theme and components for a polished terminal experience.
//!
//! This module provides consistent styling, beautiful box drawing, progress indicators,
//! and celebration effects for the vibelings CLI.

use console::{style, StyledObject, Term};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════════════════════
// THEME COLORS - Consistent color palette across all commands
// ═══════════════════════════════════════════════════════════════════════════════

/// Primary brand color for headers and emphasis
pub fn primary<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).cyan().bold()
}

/// Secondary brand color for accents
pub fn secondary<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).magenta()
}

/// Success color for passed/completed states
pub fn success<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).green().bold()
}

/// Error color for failed states
pub fn error<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).red().bold()
}

/// Warning color for attention-needed states
pub fn warning<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).yellow()
}

/// Muted color for secondary information
pub fn muted<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).dim()
}

/// Accent color for interactive elements and keys
pub fn accent<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).cyan()
}

/// Highlight color for important values
pub fn highlight<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).white().bold()
}

/// Title styling for major headers
pub fn title<D: std::fmt::Display>(text: D) -> StyledObject<D> {
    style(text).cyan().bold()
}

// ═══════════════════════════════════════════════════════════════════════════════
// UNICODE SYMBOLS - Beautiful icons for status and decoration
// ═══════════════════════════════════════════════════════════════════════════════

/// Icon collection for consistent visual language across the CLI.
#[allow(missing_docs)]
pub mod icons {
    // Status indicators
    pub const CHECK: &str = "✔";
    pub const CROSS: &str = "✘";
    pub const WARNING: &str = "⚠";
    pub const INFO: &str = "ℹ";
    pub const QUESTION: &str = "?";

    // Progress indicators
    pub const ARROW_RIGHT: &str = "→";
    pub const ARROW_DOWN: &str = "↓";
    pub const ARROW_UP: &str = "↑";
    pub const BULLET: &str = "•";
    pub const STAR: &str = "★";
    pub const STAR_EMPTY: &str = "☆";
    pub const DIAMOND: &str = "◆";
    pub const DIAMOND_EMPTY: &str = "◇";
    pub const PLAY: &str = "▶";
    pub const PAUSE: &str = "⏸";

    // Status symbols (matching ExerciseStatus)
    pub const PENDING: &str = "○";
    pub const IN_PROGRESS: &str = "◐";
    pub const COMPLETED: &str = "●";
    pub const FLAKY: &str = "◑";
    pub const NEEDS_RERUNS: &str = "◔";
    pub const EXPERIMENTAL: &str = "◇";
    pub const LOCKED: &str = "◌";

    // Section markers
    pub const SECTION_START: &str = "┌";
    pub const SECTION_END: &str = "└";
    pub const VERTICAL: &str = "│";
    pub const HORIZONTAL: &str = "─";
    pub const THICK_HORIZONTAL: &str = "━";

    // Decorative
    pub const SPARKLE: &str = "✨";
    pub const FIRE: &str = "🔥";
    pub const ROCKET: &str = "🚀";
    pub const TARGET: &str = "🎯";
    pub const TROPHY: &str = "🏆";
    pub const MEDAL: &str = "🥇";
    pub const LIGHTBULB: &str = "💡";
    pub const BOOK: &str = "📚";
    pub const GEAR: &str = "⚙";
    pub const MAGNIFIER: &str = "🔍";
    pub const HEART: &str = "❤";
    pub const WAVE: &str = "👋";
    pub const PARTY: &str = "🎉";
    pub const CONFETTI: &str = "🎊";
    pub const DOLLAR: &str = "💰";
    pub const STETHOSCOPE: &str = "🩺";
    pub const REFRESH: &str = "🔄";
    pub const CLOCK: &str = "⏱";
    pub const STOPWATCH: &str = "⏱";
    pub const ZAP: &str = "⚡";
    pub const BRAIN: &str = "🧠";
    pub const CHECKERED_FLAG: &str = "🏁";
    pub const MUSCLE: &str = "💪";
    pub const EYES: &str = "👀";
    pub const WRENCH: &str = "🔧";
    pub const KEY: &str = "🔑";
    pub const LOCK: &str = "🔒";
    pub const UNLOCK: &str = "🔓";
    pub const CLIPBOARD: &str = "📋";
}

// ═══════════════════════════════════════════════════════════════════════════════
// BOX DRAWING - Beautiful panels and frames
// ═══════════════════════════════════════════════════════════════════════════════

/// Box drawing characters for beautiful panels.
#[allow(missing_docs)]
pub mod box_chars {
    // Rounded corners (prettier)
    pub const TOP_LEFT: &str = "╭";
    pub const TOP_RIGHT: &str = "╮";
    pub const BOTTOM_LEFT: &str = "╰";
    pub const BOTTOM_RIGHT: &str = "╯";
    pub const HORIZONTAL: &str = "─";
    pub const VERTICAL: &str = "│";

    // Double-line style for emphasis
    pub const DOUBLE_HORIZONTAL: &str = "═";
    pub const DOUBLE_VERTICAL: &str = "║";
    pub const DOUBLE_TOP_LEFT: &str = "╔";
    pub const DOUBLE_TOP_RIGHT: &str = "╗";
    pub const DOUBLE_BOTTOM_LEFT: &str = "╚";
    pub const DOUBLE_BOTTOM_RIGHT: &str = "╝";

    // Connectors
    pub const T_LEFT: &str = "├";
    pub const T_RIGHT: &str = "┤";
    pub const T_TOP: &str = "┬";
    pub const T_BOTTOM: &str = "┴";
    pub const CROSS: &str = "┼";
}

/// Draw a beautiful box around content
pub fn draw_box(title: &str, content: &[&str], width: usize) {
    use box_chars::*;

    let inner_width = width.saturating_sub(2);

    // Top border with title
    let title_display = if title.is_empty() {
        String::new()
    } else {
        format!(" {} ", title)
    };
    let title_len = console::measure_text_width(&title_display);
    let padding_left = 2;
    let padding_right = inner_width.saturating_sub(padding_left + title_len);

    print!("{}", style(TOP_LEFT).cyan());
    print!("{}", style(HORIZONTAL.repeat(padding_left)).cyan());
    print!("{}", style(&title_display).cyan().bold());
    print!("{}", style(HORIZONTAL.repeat(padding_right)).cyan());
    println!("{}", style(TOP_RIGHT).cyan());

    // Content lines
    for line in content {
        let line_len = console::measure_text_width(line);
        let padding = inner_width.saturating_sub(line_len);
        print!("{} ", style(VERTICAL).cyan());
        print!("{}", line);
        println!("{}{}", " ".repeat(padding), style(VERTICAL).cyan());
    }

    // Bottom border
    print!("{}", style(BOTTOM_LEFT).cyan());
    print!("{}", style(HORIZONTAL.repeat(inner_width)).cyan());
    println!("{}", style(BOTTOM_RIGHT).cyan());
}

/// Draw a simple horizontal divider
pub fn divider(width: usize) {
    println!("{}", style(box_chars::HORIZONTAL.repeat(width)).dim());
}

/// Draw a section header with decorative line
pub fn section_header(title: &str) {
    let term_width = Term::stdout().size().1 as usize;
    let title_with_padding = format!(" {} ", title);
    let title_len = console::measure_text_width(&title_with_padding);
    let remaining = term_width.saturating_sub(title_len).saturating_sub(4);
    let left_pad = 2;
    let right_pad = remaining;

    println!();
    print!("{}", style(box_chars::HORIZONTAL.repeat(left_pad)).cyan());
    print!("{}", style(&title_with_padding).cyan().bold());
    println!("{}", style(box_chars::HORIZONTAL.repeat(right_pad)).cyan());
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROGRESS INDICATORS - Spinners and progress bars
// ═══════════════════════════════════════════════════════════════════════════════

/// Create a beautiful spinner for async operations
pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .expect("Invalid spinner template"),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Create a spinner with dots animation
pub fn create_dots_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["   ", ".  ", ".. ", "...", " ..", "  .", "   "])
            .template("{msg}{spinner}")
            .expect("Invalid spinner template"),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(200));
    pb
}

/// Create a progress bar for known-length operations
pub fn create_progress_bar(len: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.cyan} [{bar:40.cyan/dim}] {pos}/{len} ({eta})")
            .expect("Invalid progress bar template")
            .progress_chars("━━╸"),
    );
    pb.set_message(message.to_string());
    pb
}

/// Create a minimal progress bar
pub fn create_minimal_progress(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  {bar:30.cyan/dim} {pos}/{len}")
            .expect("Invalid progress bar template")
            .progress_chars("●○ "),
    );
    pb
}

// ═══════════════════════════════════════════════════════════════════════════════
// HEADERS AND BANNERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Display the main vibelings header/logo
pub fn print_header() {
    println!();
    println!(
        "  {}",
        style("╭─────────────────────────────────────────╮").cyan()
    );
    println!(
        "  {}                                         {}",
        style("│").cyan(),
        style("│").cyan()
    );
    println!(
        "  {}   {}  {}                   {}",
        style("│").cyan(),
        icons::TARGET,
        style("V I B E L I N G S").cyan().bold(),
        style("│").cyan()
    );
    println!(
        "  {}      {}           {}",
        style("│").cyan(),
        style("Learn to Build Agentic AI").dim(),
        style("│").cyan()
    );
    println!(
        "  {}                                         {}",
        style("│").cyan(),
        style("│").cyan()
    );
    println!(
        "  {}",
        style("╰─────────────────────────────────────────╯").cyan()
    );
    println!();
}

/// Display a compact header for commands
pub fn print_command_header(icon: &str, title: &str) {
    println!();
    println!("  {} {}", icon, style(title).cyan().bold());
    println!();
}

/// Print the watch mode header
pub fn print_watch_header() {
    println!();
    println!(
        "  {} {}  {}",
        icons::TARGET,
        style("VIBELINGS").cyan().bold(),
        style("• Watch Mode").dim()
    );
    println!("  {}", style("━".repeat(48)).cyan().dim());
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXERCISE DISPLAY COMPONENTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Status badge component
pub fn status_badge(status: &crate::ExerciseStatus) -> String {
    match status {
        crate::ExerciseStatus::Pending => format!("{}", style(" PENDING ").on_white().black()),
        crate::ExerciseStatus::InProgress => format!("{}", style(" ACTIVE ").on_cyan().black()),
        crate::ExerciseStatus::Completed => format!("{}", style(" DONE ").on_green().black()),
        crate::ExerciseStatus::Flaky => format!("{}", style(" FLAKY ").on_yellow().black()),
        crate::ExerciseStatus::NeedsReruns => format!("{}", style(" RETRY ").on_yellow().black()),
        crate::ExerciseStatus::Experimental => format!("{}", style(" BETA ").on_magenta().white()),
    }
}

/// Status symbol with color
pub fn status_symbol(status: &crate::ExerciseStatus) -> String {
    match status {
        crate::ExerciseStatus::Pending => format!("{}", style(icons::PENDING).dim()),
        crate::ExerciseStatus::InProgress => format!("{}", style(icons::IN_PROGRESS).cyan()),
        crate::ExerciseStatus::Completed => format!("{}", style(icons::COMPLETED).green()),
        crate::ExerciseStatus::Flaky => format!("{}", style(icons::FLAKY).yellow()),
        crate::ExerciseStatus::NeedsReruns => format!("{}", style(icons::NEEDS_RERUNS).yellow()),
        crate::ExerciseStatus::Experimental => format!("{}", style(icons::EXPERIMENTAL).magenta()),
    }
}

/// Display an exercise card in a beautiful box format
pub fn print_exercise_card(
    id: &str,
    title: &str,
    track: &str,
    description: Option<&str>,
    difficulty: u8,
) {
    use box_chars::*;

    let term_width = Term::stdout().size().1 as usize;
    let width = term_width.clamp(40, 60);
    let inner_width = width - 2;

    // Top border
    println!(
        "  {}{}{}",
        style(TOP_LEFT).cyan(),
        style(HORIZONTAL.repeat(inner_width)).cyan(),
        style(TOP_RIGHT).cyan()
    );

    // ID and difficulty stars
    let stars: String = (0..5)
        .map(|i| {
            if i < difficulty {
                style(icons::STAR).yellow().to_string()
            } else {
                style(icons::STAR_EMPTY).dim().to_string()
            }
        })
        .collect();

    let id_line = format!("  {}  {}", style(id).cyan().bold(), stars);
    let id_display_len = console::measure_text_width(&format!("  {}  {}", id, "★★★★★"));
    let id_padding = inner_width.saturating_sub(id_display_len);
    println!(
        "  {} {}{}{}",
        style(VERTICAL).cyan(),
        id_line,
        " ".repeat(id_padding),
        style(VERTICAL).cyan()
    );

    // Title
    let title_truncated = truncate_str(title, inner_width - 4);
    let title_len = console::measure_text_width(&title_truncated);
    let title_padding = inner_width.saturating_sub(title_len + 2);
    println!(
        "  {}  {}{}{}",
        style(VERTICAL).cyan(),
        style(&title_truncated).white().bold(),
        " ".repeat(title_padding),
        style(VERTICAL).cyan()
    );

    // Track
    let track_line = format!("{} {}", icons::BOOK, track);
    let track_len = console::measure_text_width(&track_line);
    let track_padding = inner_width.saturating_sub(track_len + 2);
    println!(
        "  {}  {}{}{}",
        style(VERTICAL).cyan(),
        style(&track_line).dim(),
        " ".repeat(track_padding),
        style(VERTICAL).cyan()
    );

    // Separator
    println!(
        "  {}{}{}",
        style(T_LEFT).cyan(),
        style(HORIZONTAL.repeat(inner_width)).cyan().dim(),
        style(T_RIGHT).cyan()
    );

    // Description (if any)
    if let Some(desc) = description {
        let desc_lines = wrap_text(desc, inner_width - 4);
        for line in desc_lines {
            let line_len = console::measure_text_width(&line);
            let line_padding = inner_width.saturating_sub(line_len + 2);
            println!(
                "  {}  {}{}{}",
                style(VERTICAL).cyan(),
                style(&line).dim(),
                " ".repeat(line_padding),
                style(VERTICAL).cyan()
            );
        }
    }

    // Bottom border
    println!(
        "  {}{}{}",
        style(BOTTOM_LEFT).cyan(),
        style(HORIZONTAL.repeat(inner_width)).cyan(),
        style(BOTTOM_RIGHT).cyan()
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// KEY HINTS AND CONTROLS
// ═══════════════════════════════════════════════════════════════════════════════

/// Display keyboard shortcut hints in watch mode
pub fn print_key_hints() {
    println!();
    print_key_bar(&[("h", "hint"), ("n", "next"), ("l", "list"), ("q", "quit")]);
}

/// Display a single key hint
pub fn key_hint(key: &str, description: &str) -> String {
    format!(
        "{} {}",
        style(format!("[{}]", key)).cyan(),
        style(description).dim()
    )
}

/// Display a styled key bar with multiple options
pub fn print_key_bar(keys: &[(&str, &str)]) {
    let keys_str: Vec<String> = keys
        .iter()
        .map(|(k, d)| format!("{} {}", style(format!("[{}]", k)).cyan(), style(*d).dim()))
        .collect();
    println!("  {}", keys_str.join("  "));
}

/// Display a footer with key hints in a box
pub fn print_key_footer(keys: &[(&str, &str)]) {
    let keys_str: Vec<String> = keys
        .iter()
        .map(|(k, d)| {
            format!(
                "{}{}",
                style(*k).cyan().bold(),
                style(format!(":{}", d)).dim()
            )
        })
        .collect();
    let content = keys_str.join(" │ ");
    println!();
    println!("  {} {} {}", style("╶").dim(), content, style("╴").dim());
}

/// Display a status line with current mode and info
pub fn print_status_line(mode: &str, info: &str) {
    println!(
        "  {} {}  {}",
        style("●").cyan(),
        style(mode).cyan().bold(),
        style(info).dim()
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESULT DISPLAY
// ═══════════════════════════════════════════════════════════════════════════════

/// Display a success result with celebration
pub fn print_success(message: &str, details: Option<&str>) {
    println!();
    println!("  {} {}", icons::CHECK, style(message).green().bold());
    if let Some(d) = details {
        println!("     {}", style(d).dim());
    }
}

/// Display a failure result
pub fn print_failure(message: &str, details: Option<&str>) {
    println!();
    println!("  {} {}", icons::CROSS, style(message).red().bold());
    if let Some(d) = details {
        println!("     {}", style(d).dim());
    }
}

/// Display a warning message
pub fn print_warning(message: &str) {
    println!();
    println!("  {} {}", icons::WARNING, style(message).yellow());
}

/// Display an info message
pub fn print_info(message: &str) {
    println!("  {} {}", icons::INFO, style(message).dim());
}

// ═══════════════════════════════════════════════════════════════════════════════
// CELEBRATION EFFECTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Display celebration for completing an exercise
pub fn celebrate_pass() {
    println!();
    println!(
        "  {}{}{}  {}",
        icons::SPARKLE,
        icons::PARTY,
        icons::SPARKLE,
        style("PASSED!").green().bold()
    );
    println!("  {}", style("━━━━━━━━━━━━━━━━━━━━━").green().dim());
}

/// Display a compact success message
pub fn print_pass_compact() {
    println!(
        "  {} {}",
        style(icons::CHECK).green().bold(),
        style("PASSED").green().bold()
    );
}

/// Display run statistics in a clean format
pub fn print_run_stats(duration_secs: f64, cost_usd: f64, tokens_in: u32, tokens_out: u32) {
    let duration_display = if duration_secs < 1.0 {
        format!("{:.0}ms", duration_secs * 1000.0)
    } else if duration_secs < 60.0 {
        format!("{:.1}s", duration_secs)
    } else {
        let mins = (duration_secs / 60.0).floor();
        let secs = duration_secs % 60.0;
        format!("{:.0}m {:.0}s", mins, secs)
    };

    let cost_display = if cost_usd < 0.0001 {
        "< $0.0001".to_string()
    } else if cost_usd < 0.01 {
        format!("${:.4}", cost_usd)
    } else {
        format!("${:.3}", cost_usd)
    };

    println!(
        "     {} {}   {} {}",
        style(icons::STOPWATCH).dim(),
        style(&duration_display).white(),
        style(icons::DOLLAR).dim(),
        style(&cost_display).white(),
    );
    println!(
        "     {} {} {} / {} {}",
        style(icons::ARROW_RIGHT).dim(),
        style("Tokens:").dim(),
        style(format_number(tokens_in)).cyan(),
        style(format_number(tokens_out)).cyan(),
        style("(in/out)").dim(),
    );
}

/// Format a number with K/M suffixes for readability
fn format_number(n: u32) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 10_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else if n >= 1_000 {
        format!("{:.2}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

/// Display big celebration for completing all exercises
pub fn celebrate_completion() {
    println!();
    println!(
        "  {}",
        style("╔═══════════════════════════════════════════════╗").green()
    );
    println!(
        "  {}                                               {}",
        style("║").green(),
        style("║").green()
    );
    println!(
        "  {}     {}{}{}{}{}  {}    {}",
        style("║").green(),
        icons::SPARKLE,
        icons::TROPHY,
        icons::PARTY,
        icons::TROPHY,
        icons::SPARKLE,
        style("CONGRATULATIONS!").green().bold(),
        style("║").green()
    );
    println!(
        "  {}                                               {}",
        style("║").green(),
        style("║").green()
    );
    println!(
        "  {}       {}              {}",
        style("║").green(),
        style("You've completed all exercises!").white().bold(),
        style("║").green()
    );
    println!(
        "  {}                                               {}",
        style("║").green(),
        style("║").green()
    );
    println!(
        "  {}     {}                   {}",
        style("║").green(),
        style("You are now an Agentic AI Engineer!").cyan(),
        style("║").green()
    );
    println!(
        "  {}                                               {}",
        style("║").green(),
        style("║").green()
    );
    println!(
        "  {}",
        style("╚═══════════════════════════════════════════════╝").green()
    );
    println!();
}

/// Display a goodbye message
pub fn print_goodbye() {
    println!();
    println!(
        "  {} {}",
        icons::WAVE,
        style("See you next time! Keep learning!").cyan()
    );
    println!();
}

// ═══════════════════════════════════════════════════════════════════════════════
// TABLES
// ═══════════════════════════════════════════════════════════════════════════════

/// Print a table header row
pub fn table_header(columns: &[(&str, usize)]) {
    let mut header = String::from("  ");
    let mut separator = String::from("  ");

    for (name, width) in columns {
        header.push_str(&format!("{:width$}", style(name).bold(), width = width));
        separator.push_str(&box_chars::HORIZONTAL.repeat(*width).to_string());
    }

    println!("{}", header);
    println!("{}", style(separator).dim());
}

/// Print a table row
pub fn table_row(cells: &[(String, usize)]) {
    let mut row = String::from("  ");
    for (content, width) in cells {
        row.push_str(&format!("{:width$}", content, width = width));
    }
    println!("{}", row);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROGRESS VISUALIZATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Display a progress summary bar
pub fn print_progress_bar(completed: usize, total: usize) {
    let percentage = if total > 0 {
        (completed as f64 / total as f64 * 100.0) as usize
    } else {
        0
    };

    let bar_width = 30;
    let filled = (bar_width * completed) / total.max(1);
    let empty = bar_width - filled;

    // Choose color based on progress
    let (bar_color, icon) = if percentage >= 100 {
        ("green", icons::TROPHY)
    } else if percentage >= 75 {
        ("cyan", icons::STAR)
    } else if percentage >= 50 {
        ("cyan", icons::DIAMOND)
    } else if percentage >= 25 {
        ("yellow", icons::ARROW_RIGHT)
    } else {
        ("yellow", icons::BULLET)
    };

    let filled_str: String = "█".repeat(filled);
    let empty_str: String = "░".repeat(empty);

    println!();
    println!(
        "  {} {} {}/{} {}",
        icon,
        style("Progress:").white().bold(),
        style(completed).green().bold(),
        style(total).dim(),
        style(format!("({}%)", percentage)).cyan()
    );

    let styled_bar = match bar_color {
        "green" => format!("[{}{}]", style(filled_str).green(), style(empty_str).dim()),
        "cyan" => format!("[{}{}]", style(filled_str).cyan(), style(empty_str).dim()),
        _ => format!("[{}{}]", style(filled_str).yellow(), style(empty_str).dim()),
    };
    println!("  {}", styled_bar);
}

/// Display a compact inline progress indicator
pub fn inline_progress(completed: usize, total: usize) -> String {
    let percentage = if total > 0 {
        (completed as f64 / total as f64 * 100.0) as usize
    } else {
        0
    };

    let bar_width = 10;
    let filled = (bar_width * completed) / total.max(1);
    let empty = bar_width - filled;

    let filled_str: String = "█".repeat(filled);
    let empty_str: String = "░".repeat(empty);

    format!(
        "{}{} {}%",
        style(filled_str).cyan(),
        style(empty_str).dim(),
        percentage
    )
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Truncate a string to a maximum length, adding ellipsis if needed
pub fn truncate_str(s: &str, max_len: usize) -> String {
    let display_len = console::measure_text_width(s);
    if display_len <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        ".".repeat(max_len)
    } else {
        // This is approximate since we're dealing with graphemes
        let chars: Vec<char> = s.chars().collect();
        let mut result = String::new();
        for (len, c) in chars.into_iter().enumerate() {
            if len + 3 >= max_len {
                break;
            }
            result.push(c);
        }
        result.push_str("...");
        result
    }
}

/// Wrap text to fit within a given width
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_len = 0;

    for word in text.split_whitespace() {
        let word_len = console::measure_text_width(word);

        if current_len == 0 {
            current_line = word.to_string();
            current_len = word_len;
        } else if current_len + 1 + word_len <= width {
            current_line.push(' ');
            current_line.push_str(word);
            current_len += 1 + word_len;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            current_len = word_len;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

/// Clear the terminal screen
pub fn clear_screen() -> std::io::Result<()> {
    Term::stdout().clear_screen()
}

/// Get terminal width
pub fn term_width() -> usize {
    Term::stdout().size().1 as usize
}

/// Print an empty line
pub fn blank() {
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_str() {
        assert_eq!(truncate_str("hello", 10), "hello");
        assert_eq!(truncate_str("hello world", 8), "hello...");
        assert_eq!(truncate_str("hi", 2), "hi");
    }

    #[test]
    fn test_truncate_str_edge_cases() {
        // Exact length
        assert_eq!(truncate_str("hello", 5), "hello");
        // Empty string
        assert_eq!(truncate_str("", 10), "");
        // Very short max (less than ellipsis)
        assert_eq!(truncate_str("hello", 2), "..");
        assert_eq!(truncate_str("hello", 3), "...");
        // Unicode characters
        assert_eq!(truncate_str("héllo", 10), "héllo");
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a test of text wrapping";
        let lines = wrap_text(text, 15);
        assert!(!lines.is_empty());
        for line in &lines {
            assert!(console::measure_text_width(line) <= 15);
        }
    }

    #[test]
    fn test_wrap_text_edge_cases() {
        // Empty string
        let lines = wrap_text("", 10);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "");

        // Single word longer than width
        let lines = wrap_text("superlongword", 5);
        assert!(!lines.is_empty());

        // Multiple spaces between words
        let lines = wrap_text("word1    word2", 20);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("word1"));
        assert!(lines[0].contains("word2"));

        // Single word that fits exactly
        let lines = wrap_text("hello", 5);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "hello");
    }

    #[test]
    fn test_icons_constants() {
        // Verify icon constants have content (using len() to avoid const_is_empty warning)
        assert!(icons::CHECK.len() > 0);
        assert!(icons::CROSS.len() > 0);
        assert!(icons::WARNING.len() > 0);
        assert!(icons::PENDING.len() > 0);
        assert!(icons::COMPLETED.len() > 0);
    }

    #[test]
    fn test_box_chars_constants() {
        // Verify box drawing characters have content
        assert!(box_chars::TOP_LEFT.len() > 0);
        assert!(box_chars::TOP_RIGHT.len() > 0);
        assert!(box_chars::BOTTOM_LEFT.len() > 0);
        assert!(box_chars::BOTTOM_RIGHT.len() > 0);
        assert!(box_chars::HORIZONTAL.len() > 0);
        assert!(box_chars::VERTICAL.len() > 0);
    }

    #[test]
    fn test_status_symbol_all_variants() {
        use crate::ExerciseStatus;

        // All variants should return a non-empty string
        let pending = status_symbol(&ExerciseStatus::Pending);
        let in_progress = status_symbol(&ExerciseStatus::InProgress);
        let completed = status_symbol(&ExerciseStatus::Completed);
        let flaky = status_symbol(&ExerciseStatus::Flaky);
        let needs_reruns = status_symbol(&ExerciseStatus::NeedsReruns);
        let experimental = status_symbol(&ExerciseStatus::Experimental);

        assert!(!pending.is_empty());
        assert!(!in_progress.is_empty());
        assert!(!completed.is_empty());
        assert!(!flaky.is_empty());
        assert!(!needs_reruns.is_empty());
        assert!(!experimental.is_empty());
    }

    #[test]
    fn test_status_badge_all_variants() {
        use crate::ExerciseStatus;

        // All variants should return a non-empty string
        let pending = status_badge(&ExerciseStatus::Pending);
        let in_progress = status_badge(&ExerciseStatus::InProgress);
        let completed = status_badge(&ExerciseStatus::Completed);
        let flaky = status_badge(&ExerciseStatus::Flaky);
        let needs_reruns = status_badge(&ExerciseStatus::NeedsReruns);
        let experimental = status_badge(&ExerciseStatus::Experimental);

        assert!(!pending.is_empty());
        assert!(!in_progress.is_empty());
        assert!(!completed.is_empty());
        assert!(!flaky.is_empty());
        assert!(!needs_reruns.is_empty());
        assert!(!experimental.is_empty());
    }

    #[test]
    fn test_key_hint() {
        let hint = key_hint("h", "help");
        assert!(hint.contains("h"));
        assert!(hint.contains("help"));
    }

    #[test]
    fn test_term_width() {
        // Should return a reasonable width (at least 1)
        let width = term_width();
        assert!(width >= 1);
    }
}
