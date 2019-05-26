window.$ = window.jQuery = require('jquery');

const fs = require('fs');
const electron = require('electron').remote
const dialog = electron.dialog

const input = require('./input.js')
const interface = require('./interface.js')
const colorpicker = require('./colorpicker.js')
const generate = require('./generator.js');

$('#generate-button').click(generate);
$('#open-input').click(interface.openInputFile)
$('#open-blacklist').click(interface.openBlacklistFile)
$('#save-button').click(() => {
    let renderedFile = $('#img-out').attr('src')
    if (!renderedFile || !renderedFile.endsWith(".png")) {
        return
    }
    let targetFile = dialog.showSaveDialog({
        filters: [{
            name: 'Image',
            extensions: ['png']
        }]
    })
    if (!targetFile) {
        return
    }

    fs.copyFile(renderedFile, targetFile, (err) => {
        if (err) throw err;
    });
})