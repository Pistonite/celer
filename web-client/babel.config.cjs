// babel config for transpiling code for jest to run
module.exports = {
    presets: [
        [
            "@babel/preset-env",
            {
                targets: {
                    node: "current"
                }
            }
        ],
        "@babel/preset-react",
        "@babel/preset-typescript",
    ]
};
