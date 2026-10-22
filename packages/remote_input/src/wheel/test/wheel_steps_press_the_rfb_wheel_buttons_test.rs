use crate::WheelStep;

#[test]
fn test_wheel_steps_press_the_rfb_wheel_buttons() {
    assert_eq!(WheelStep::Up.button_bit(), 8, "wheel up is button 4, bit 3 of the mask");
    assert_eq!(WheelStep::Down.button_bit(), 16, "wheel down is button 5, bit 4 of the mask");
    assert_eq!(WheelStep::Left.button_bit(), 32, "wheel left is button 6, bit 5 of the mask");
    assert_eq!(WheelStep::Right.button_bit(), 64, "wheel right is button 7, bit 6 of the mask");
}
