const path = require("path");

const {CheckerPlugin} = require("awesome-typescript-loader");
const MiniCssExtractPlugin  = require("mini-css-extract-plugin");
const UglifyJsPlugin = require("uglifyjs-webpack-plugin");
const OptimizeCSSAssetsPlugin = require("optimize-css-assets-webpack-plugin");

const devMode = process.env.NODE_ENV !== "production";

module.exports = {
    mode: devMode ? "development" : "production",

    entry: path.join(__dirname, "src", "index.ts"),
    output: {
        path: path.join(__dirname, "public"),
        filename: "js/app.js",
    },

    // Currently we need to add ".ts" to the resolve.extensions array.
    resolve: {
        extensions: [".ts", ".js", ".tsx", ".jsx", ".css", ".sass", ".scss"]
    },

    plugins: devMode ? [
        new CheckerPlugin(),
        new MiniCssExtractPlugin ({
            filename: "css/app.css",
        }),
    ] : [
        new MiniCssExtractPlugin ({
            filename: "css/app.css",
        }),
        new OptimizeCSSAssetsPlugin(),
    ],

    optimization: devMode ? {} : {
        minimizer: [new UglifyJsPlugin()],
    },

    devServer: devMode ? {
        contentBase: path.join(__dirname, "public"),
        historyApiFallback: true,
        inline: true,
        open: false,
        proxy: {
            "/graphql": "http://localhost:9002",
        },
    } : {},

    module: {
        rules: [{
            test: /\.tsx?$/,
            loader: "awesome-typescript-loader",
          }, {
            test: /\.(sa|sc|c)ss$/,
            use: [
                devMode ? "style-loader" : {
                    loader: MiniCssExtractPlugin.loader,
                },
                "css-loader",
                "postcss-loader",
                "sass-loader",
            ],
        }],
    },
};
