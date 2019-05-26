const {
  app,
  BrowserWindow,
  globalShortcut
} = require('electron');

let mainWindow = null;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1100,
    height: 600,
    webPreferences: {
      nodeIntegration: true
    },
    resizable: false
  })

  globalShortcut.register('CommandOrControl+Shift+K', () => { 
    mainWindow.openDevTools()
  });

  mainWindow.setMenu(null);
  mainWindow.loadFile('index.html')

  mainWindow.on('closed', function () {
    mainWindow = null
  })
}

app.on('ready', createWindow)

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit()
})

app.on('activate', function () {
  if (mainWindow === null) createWindow()
})