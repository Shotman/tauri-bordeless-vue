module.exports = {
  configureWebpack: {
    resolve: {
      alias: {
        '@Tauri': '@tauri-apps/api/dist'
      },
      extensions: ['.js', '.vue', '.json']
    }
  }
}
