import "./index.scss";

import m from "mithril";

interface ISidebarHeader {
    title: string
}

export default class SidebarHeader implements m.ClassComponent<ISidebarHeader> {
    view({attrs}: m.CVnode<ISidebarHeader>) {
        return m(".sidebar.header", attrs.title);
    }
}