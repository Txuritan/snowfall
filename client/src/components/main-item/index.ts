import "./index.scss";

import m from "mithril";

interface IMainItem {
    id: string
    icon: string
    title: string
    tags: string[]
    excerpt: string
    domain: string
    link: string
}

export default class MainItem implements m.ClassComponent<IMainItem> {
    view({attrs}: m.CVnode<IMainItem>) {
        return m(".main.item", [
            m("h3", attrs.title),
            m("p", [
                attrs.tags.map(tag => {
                    return m("a.tag", `#${tag} `);
                }),
                attrs.excerpt
            ]),
        ]);
    }
}