import "./index.scss";

import m from "mithril";

interface ISidebarItem {
    icon: string
    title: string
    children: m.Children
}

export default class SidebarItem implements m.ClassComponent<ISidebarItem> {
    view({attrs}: m.CVnode<ISidebarItem>) {
        return m("p.sidebar.header", attrs.title);
    }
}