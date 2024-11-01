use crate::widgets;
use renderer::window_renderer::Text;
use renderer::window_renderer::TypeOfBorder;
use storage::Node;

pub struct LabelInfo {
    text: Vec<Text>, // Here each tuple is a different struct. (String, u32): Content,
    // Line number
    height: u32,
    width: u32,
    type_of_border: TypeOfBorder,
    column: u32,
    line_number: u32,
}

trait LabelWidget: Node {
    fn get_info(&self) -> LabelInfo;
    fn get_name(&self) -> String;
}

impl<T: LabelWidget> widgets::Widget for T {
    fn render(&self) -> renderer::sub_win::SubWindow {
        let info = self.get_info();

        let window = renderer::sub_win::NestedWindow::new(
            info.text
                .iter()
                .map(|x| renderer::sub_win::TextType::Text(x.clone()))
                .collect(),
            info.height,
            info.width,
            info.type_of_border,
        );

        let sub_win = renderer::sub_win::SubWindow::new(window, info.line_number, info.column);

        sub_win
    }
}
