use minmetal::*;
use std::sync::{Arc, mpsc};
use std::time::Duration;

#[test]
fn timestamp_scale_is_finite() {
    let scale = timestamp_nanoseconds_per_tick().unwrap();
    assert!(scale.is_finite() && scale > 0.0);
}

#[test]
#[ignore = "requires Metal 4"]
fn feedback_survives_registration_and_reuses_options() {
    let device = Device::system_default().expect("Metal device");
    let queue = device.new_m4_command_queue().expect("Metal 4 queue");
    let options = M4CommitOptions::new();
    let (sender, receiver) = mpsc::channel();
    let captured = Arc::new(());
    let weak = Arc::downgrade(&captured);
    let handler = M4CommitFeedbackHandler::new(move |feedback| {
        let _keep_alive = &captured;
        sender
            .send(feedback.error_description().map(|e| e.to_string()))
            .unwrap();
    });
    assert!(weak.upgrade().is_some());
    for _ in 0..2 {
        options.add_feedback_handler(&handler);
        let allocator = device.new_m4_command_allocator().unwrap();
        let buffer = device.new_m4_command_buffer().unwrap();
        buffer.begin_command_buffer_with_allocator(&allocator);
        buffer.end_command_buffer();
        queue.commit_with_options(&[buffer], &options);
        assert_eq!(
            receiver.recv_timeout(Duration::from_secs(10)).unwrap(),
            None
        );
    }
    options.add_feedback_handler(&handler);
    drop(handler);
    drop(options);
    // Release queued callback blocks before checking ownership.
    drop(queue);
    assert!(matches!(receiver.recv_timeout(Duration::from_secs(10)), Err(mpsc::RecvTimeoutError::Disconnected)));
    assert!(weak.upgrade().is_none());
}
