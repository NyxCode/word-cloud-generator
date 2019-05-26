const spawn = require('child_process').spawn;
const exec = require('child_process').exec;
const electron = require('electron').remote
const dialog = electron.dialog
const opn = require('opn')
const path = require('path')
const input = require('./input.js')
const colorpicker = require('./colorpicker.js')
const randomstring = require("randomstring")
const rimraf = require("rimraf")
const fs = require("fs")
const processProgress = require("./generator.js");

const appDir = path.dirname(require.main.filename)
const rcsDir = path.join(appDir, 'rcs')
const imgDir = path.join(appDir, 'generated_images')
const inputFile = path.join(rcsDir, 'input.txt')
const blacklistFile = path.join(rcsDir, 'blacklist.txt')
const executable = path.join(rcsDir, "wcgrs.exe")

let commandLine = (() => {
    switch (process.platform) {
        case 'darwin':
            return 'open';
        case 'win32':
            return 'explorer';
        case 'win64':
            return 'explorer';
        default:
            return 'xdg-open';
    }
})();

if (fs.existsSync(imgDir)) rimraf.sync(imgDir);
fs.mkdirSync(imgDir);

module.exports = {
    openInputFile: () => {
        let cmd = commandLine + ' "' + inputFile + '"';
        console.log('$ ' + cmd)
        exec(cmd);
    },
    openBlacklistFile: () => {
        let cmd = commandLine + ' "' + blacklistFile + '"';
        console.log('$ ' + cmd)
        exec(cmd);
    }
}