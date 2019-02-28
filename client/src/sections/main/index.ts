import "./index.scss";

import * as m from "mithril";

import State from "../../state";

import MainItem from "../../components/main-item";

interface IMain {
    items: m.Children
}

export default class Main implements m.ClassComponent<IMain> {
    view({attrs}: m.CVnode<IMain>) {
        return [
            m("section#main", {
                class: State.view.content ? "content" : "",
            }, attrs.items),
        ];
    }
};