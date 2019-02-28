import "./index.scss";

import * as m from "mithril";

interface ISidebarHeader {
    title: string
}

export default class SidebarHeader implements m.ClassComponent<ISidebarHeader> {
    view({attrs}: m.CVnode<ISidebarHeader>) {
        return m(".sidebar.header", attrs.title);
    }
}