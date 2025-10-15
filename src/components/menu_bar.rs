use gpui::{Menu, MenuItem, SystemMenuType};
use crate::{Log, Quit};

pub struct MenuBar {
    pub menus: Vec<Menu>,
}

impl Default for MenuBar {
    fn default() -> Self {
        let menus = vec![Menu {
            name: "set_menus".into(),
            items: MenuBarItems::default().items,
        }];
        MenuBar { menus }
    }
}

struct MenuBarItems {
    pub items: Vec<MenuItem>,
}

impl Default for MenuBarItems {
    fn default() -> Self {
        let items = vec![
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("Quit", Quit),
            MenuItem::action("Log", Log),
        ];
        MenuBarItems {items}
    }
}