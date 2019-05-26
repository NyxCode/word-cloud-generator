const electron = require('electron').remote
const dialog = electron.dialog
const spawn = require('child_process').spawn;
const randomstring = require("randomstring")
const path = require('path')

const input = require('./input.js');
const colorpicker = require('./colorpicker.js')

const appDir = path.dirname(require.main.filename)
const rcsDir = path.join(appDir, 'rcs')
const imgDir = path.join(appDir, 'generated_images')
const executable = path.join(rcsDir, "wcgrs.exe")
const inputFile = path.join(rcsDir, 'input.txt')
const blacklistFile = path.join(rcsDir, 'blacklist.txt')

const progressBar = new ldBar('#progress-bar');

console.log("EXE: " + executable);


function generate() {
    $('#img-out').attr("src", "")
    $('#generate-button').addClass('disabled')
    $('#progress-indicator').css("visibility", "visible")

    let outputFileName = randomstring.generate() + ".png"
    let outputFile = path.join(imgDir, outputFileName)

    let args = [
        '--input-file=' + inputFile,
        '--blacklist=' + blacklistFile,
        '--word-limit=' + input.maxWords(),
        '--font-color=' + colorpicker.textColor(),
        '--font=' + input.font(),
        '--resolution=' + input.fontSize(),
        '--post-processing-rotation=' + input.postRotation(),
        '--rotate-percentage=' + input.rotationPercentage(),
        '--min-word-length=' + input.minWordLength(),
        '--output-file=' + outputFile
    ]

    if (!input.transparent()) args.push('--background-color=' + colorpicker.bgColor());
    if (input.capitalizeWords()) args.push('--capitalize-words');

    let prc = spawn(executable, args);

    let completeGeneration = () => {
        $('#generate-button').removeClass('disabled')
        $('#img-out').attr("src", outputFile)
        $('#progress-indicator').css("visibility", "hidden")
    };

    prc.stdout.on('data', (data) => {
        let completed = data.toString().split('\n')
            .filter(line => line.length != 0)
            .map(processProgress)
            .some(completed => completed);

        if (completed) completeGeneration();
    });

    prc.stderr.on('data', (data) => {
        prc.kill();
        let message = typeof data == "Uint8Array" ? new TextDecoder("utf-8").decode(data) : data.toString();
        dialog.showErrorBox("Generation failed", message);
        completeGeneration();
        runningTasks = [];
    });
}


let runningTasks = [];

function processProgress(line) {
    let type = line.substring(0, 3)

    switch (type) {
        case "-->":
            runningTasks.push({
                task: line.substring(4)
            });
            break;
        case "...":
            let progress = line.substring(4).split("/");
            let percentage = parseInt(progress[0]) / parseInt(progress[1]);
            runningTasks[runningTasks.length - 1].progress = percentage;
            break;
        case "<--":
            runningTasks.pop();
            break;
        case "!!!":
            console.warn("WARNING: " + line.substring(4));
            break;
        default:
            console.warn("could not interpret: " + type);
            break;
    }

    let task = runningTasks[runningTasks.length - 1];

    if (task == null) {
        return true;
    }

    $('#current-task').html(task.task);
    let progress = task.progress != null ? parseFloat(task.progress) * 100 : "60";
    progressBar.set(progress);
    progressBar.text = "LEL"

    return false;
}

module.exports = generate;