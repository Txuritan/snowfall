import "./index.scss";

import * as m from "mithril";

import State from "../../state";

export default class Content implements m.ClassComponent<{}> {
    view({attrs}: m.CVnode<{}>) {
        return [
            m("section#content", {
                class: State.view.content ? "" : "hidden",
            }),
        ];
    }
};