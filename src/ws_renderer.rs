use crate::ipc_reader::Workspace;

pub fn render_workspaces_bar(workspaces: &Vec<Workspace>) -> String {
    let mut workspaces = workspaces.to_owned();
    workspaces.sort_by(|a, b| a.id.cmp(&b.id));

    let mut outstr = r#"(box :class "works" :spacing 10 :orientation "h""#.to_string();
    let mut idx = 1;
    workspaces.iter().for_each(|ws| {
        let mut curr_ws_class = "ws".to_string();
        if ws.status == 1 {
            curr_ws_class.push_str(" ws_active");
        } else if ws.status == 2 {
            curr_ws_class.push_str(" ws_inactive");
        } else if ws.status == 3 {
            curr_ws_class.push_str(" ws_unfocused");
        } else if ws.status == 0 {
            curr_ws_class.push_str(" ws_otherscreen");
        }

        let mut curr_ws_ico = "".to_string();
        if ws.status == 1 || ws.status == 3 {
            curr_ws_ico = "".to_string();
        }

        let curr_ws_string = format!(r#" (button :onclick "hyprctl dispatch workspace {}" :class "{}" "{}")"#, idx, curr_ws_class, curr_ws_ico);

        while idx != ws.id {
            outstr.push_str(&format!(r#" (button :onclick "hyprctl dispatch workspace {}" :class "ws" "")"#, idx));
            idx += 1;
        }
        outstr.push_str(&curr_ws_string);
        idx += 1;
    });

    while idx <= 12 {
        outstr.push_str(&format!(r#" (button :onclick "hyprctl dispatch workspace {}" :class "ws" "")"#, idx));
        idx += 1;
    }
    outstr.push_str(")");

    outstr
}
