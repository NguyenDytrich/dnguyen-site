module.exports = {
  // For staging
  // publicPath: process.env.NODE_ENV === "production" ? "/test/dnguyen" : "/",
  chainWebpack: config => {
    config.plugin("html").tap(args => {
      args[0].title = "Dytrich Nguyen";
      return args;
    });
  }
};
