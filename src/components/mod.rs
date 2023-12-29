pub mod button;
pub mod text_box;

pub trait Component<T> {
    // If the user want to do some formating with the text they can do it here
    fn display(text_to_display: &str) -> &str {
        text_to_display
    }
    // The closure will run when the Component is clicked.

    fn on_click(closure: T)
    where
        T: Fn(),
    {
        closure()
    }
    // this closure is run every tick if the cursor (TODO) is apon it
    fn on_hover(closure: T)
    where
        T: Fn(),
    {
        closure()
    }
}
