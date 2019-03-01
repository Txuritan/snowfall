import "./index.scss";

import m from "mithril";

import State from "../../state";

import MainItem from "../../components/main-item";

import Sidebar from "../../sections/sidebar";
import Main from "../../sections/main";
import Content from "../../sections/content";

interface IResponse {
    data: IAllBookmarks
}

interface IAllBookmarks {
    allBookmarks: IBookmark[]
}

interface IBookmark {
    id: number
    title: string
    domain: string
    link: string
    tags: ITag[]
}

interface ITag {
    id: number
    name: string
}

interface IItems {
    list: IBookmark[],
    fetch: Function,
}

let Items: IItems = {
    list: [],
    fetch: () => {
        m.request({
            method: "POST",
            url: `${State.debug ? "http://localhost:9002" : ""}/graphql`,
            headers: { "Content-Type": "application/json" },
            data: {
                operationName: null,
                query: "{ allBookmarks { id title domain link type tags { id name } } }",
                variables: null,
            },
        })
        .then((items: IResponse) => {
            Items.list = items.data.allBookmarks;
        })
        .catch((e) => {
            console.log(e);
        });
    },
};

export default class Home implements m.ClassComponent<{}> {
    oninit(vnode: m.Vnode) {
        Items.fetch();
    }

    view({attrs}: m.CVnode<{}>) {
        return [
            m(Sidebar),
            m(Main, {
                items: Items.list.map((bookmark) => {
                    return m(MainItem, {
                        id: String(bookmark.id),
                        title: bookmark.title,
                        tags: bookmark.tags.map((tag) => {
                            return tag.name;
                        }),
                        domain: bookmark.domain,
                        link: bookmark.link,
                        icon: "example",
                        excerpt: "example",
                    })
                }),
            }),
            m(Content),
        ];
    }
};
