import "./index.scss";

import m from "mithril";

import State from "../../state";

import SidebarHeader from "../../components/sidebar-header";

export default class Sidebar implements m.ClassComponent<{}> {
    view({attrs}: m.CVnode<{}>) {
        return [
            m("aside#sidebar", [
                m(SidebarHeader, {title: "Main"}),
                m("button", {
                    onclick: () => {
                        State.view.content = !State.view.content;
                    },
                }, "Toggle")
            ]),
        ];
    }
};