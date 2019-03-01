class State {
    debug: boolean
    view: {
        content: boolean,
    }

    constructor() {
        this.debug = process.env.NODE_ENV === "development";
        this.view = {
            content: false,
        };
    }
}

export default new State();