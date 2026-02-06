use crate::common::harness::EditorTestHarness;
use crossterm::event::{KeyCode, KeyModifiers};

/// Test moving a line up via Alt+Up
/// Issue #914: Move Line Up/Down with Alt+Arrow
#[test]
fn test_move_line_up() {
    let mut harness = EditorTestHarness::new(80, 24).unwrap();

    harness.type_text("first\nsecond\nthird").unwrap();

    // Move to line 2 (second)
    harness
        .send_key(KeyCode::Home, KeyModifiers::CONTROL)
        .unwrap();
    harness.send_key(KeyCode::Down, KeyModifiers::NONE).unwrap();
    harness.render().unwrap();

    // Move line up with Alt+Up
    harness
        .send_key(KeyCode::Up, KeyModifiers::ALT)
        .unwrap();
    harness.render().unwrap();

    let content = harness.get_buffer_content().unwrap();
    assert_eq!(
        content, "second\nfirst\nthird",
        "Second line should have moved up"
    );
}

/// Test moving a line down via Alt+Down
#[test]
fn test_move_line_down() {
    let mut harness = EditorTestHarness::new(80, 24).unwrap();

    harness.type_text("first\nsecond\nthird").unwrap();

    // Move to line 1 (first)
    harness
        .send_key(KeyCode::Home, KeyModifiers::CONTROL)
        .unwrap();
    harness.render().unwrap();

    // Move line down with Alt+Down
    harness
        .send_key(KeyCode::Down, KeyModifiers::ALT)
        .unwrap();
    harness.render().unwrap();

    let content = harness.get_buffer_content().unwrap();
    assert_eq!(
        content, "second\nfirst\nthird",
        "First line should have moved down"
    );
}

/// Test that moving the first line up does nothing
#[test]
fn test_move_first_line_up_noop() {
    let mut harness = EditorTestHarness::new(80, 24).unwrap();

    harness.type_text("first\nsecond\nthird").unwrap();

    // Move to first line
    harness
        .send_key(KeyCode::Home, KeyModifiers::CONTROL)
        .unwrap();
    harness.render().unwrap();

    // Try to move line up (should do nothing)
    harness
        .send_key(KeyCode::Up, KeyModifiers::ALT)
        .unwrap();
    harness.render().unwrap();

    let content = harness.get_buffer_content().unwrap();
    assert_eq!(
        content, "first\nsecond\nthird",
        "First line can't move up, content should be unchanged"
    );
}

/// Test that moving the last line down does nothing
#[test]
fn test_move_last_line_down_noop() {
    let mut harness = EditorTestHarness::new(80, 24).unwrap();

    harness.type_text("first\nsecond\nthird").unwrap();
    // Cursor is already on the last line after typing
    harness.render().unwrap();

    // Try to move line down (should do nothing)
    harness
        .send_key(KeyCode::Down, KeyModifiers::ALT)
        .unwrap();
    harness.render().unwrap();

    let content = harness.get_buffer_content().unwrap();
    assert_eq!(
        content, "first\nsecond\nthird",
        "Last line can't move down, content should be unchanged"
    );
}

/// Test undo after moving a line
#[test]
fn test_move_line_undo() {
    let mut harness = EditorTestHarness::new(80, 24).unwrap();

    harness.type_text("first\nsecond\nthird").unwrap();

    // Move to second line
    harness
        .send_key(KeyCode::Home, KeyModifiers::CONTROL)
        .unwrap();
    harness.send_key(KeyCode::Down, KeyModifiers::NONE).unwrap();

    // Move line up
    harness
        .send_key(KeyCode::Up, KeyModifiers::ALT)
        .unwrap();
    harness.render().unwrap();

    let content = harness.get_buffer_content().unwrap();
    assert_eq!(content, "second\nfirst\nthird");

    // Undo
    harness
        .send_key(KeyCode::Char('z'), KeyModifiers::CONTROL)
        .unwrap();
    harness.render().unwrap();

    let content = harness.get_buffer_content().unwrap();
    assert_eq!(
        content, "first\nsecond\nthird",
        "Undo should restore original order"
    );
}

/// Test move line via command palette (for discoverability)
#[test]
fn test_move_line_via_command_palette() {
    let mut harness = EditorTestHarness::new(80, 24).unwrap();

    harness.type_text("alpha\nbeta\ngamma").unwrap();

    // Move to second line
    harness
        .send_key(KeyCode::Home, KeyModifiers::CONTROL)
        .unwrap();
    harness.send_key(KeyCode::Down, KeyModifiers::NONE).unwrap();
    harness.render().unwrap();

    // Use command palette
    harness
        .send_key(KeyCode::Char('p'), KeyModifiers::CONTROL)
        .unwrap();
    harness
        .wait_until(|h| h.screen_to_string().contains(">command"))
        .unwrap();
    harness.type_text("move line up").unwrap();
    harness
        .send_key(KeyCode::Enter, KeyModifiers::NONE)
        .unwrap();
    harness.render().unwrap();

    let content = harness.get_buffer_content().unwrap();
    assert_eq!(
        content, "beta\nalpha\ngamma",
        "Move Line Up via command palette should work"
    );
}
