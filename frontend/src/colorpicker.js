const input = require('./input.js')
const invertColor = require('./utils.js').invertColor

let textColorPicker = new iro.ColorPicker("#text-color-picker-container", {
    width: 200,
    color: "#FFF"
});

let bgColorPicker = new iro.ColorPicker("#bg-color-picker-container", {
    width: 200,
    color: "#1E1E1E"
});

// Update color of buttons
textColorPicker.on('color:change', (color, changes) => updateButtonColor('text', color));
bgColorPicker.on('color:change', (color, changes) => updateButtonColor('bg', color));

function updateButtonColor(button, color) {
    $('#button-open-' + button + '-color-picker').css('background-color', color.hexString);
    $('#button-open-' + button + '-color-picker').css('color', invertColor(color.rgb));
}

// Make buttons usable
$('#button-open-text-color-picker').click(function (e) {
    e.stopPropagation();
    showTextColorPicker()
});
$('#button-open-bg-color-picker').click(function (e) {
    e.stopPropagation();
    showBGColorPicker()
});

let colorpickerVisible = false

updateButtonColor('text', textColorPicker.color);
updateButtonColor('bg', bgColorPicker.color);

function showTextColorPicker() {
    hideColorPicker()
    $('#text-colorpicker-overlay').css("visibility", "visible")
    $(".app-wrapper").css("filter", "blur(2px)")
    colorpickerVisible = true;
}

function showBGColorPicker() {
    hideColorPicker()
    $('#bg-colorpicker-overlay').css("visibility", "visible")
    $(".app-wrapper").css("filter", "blur(2px)")
    colorpickerVisible = true;
}

function hideColorPicker() {
    $('#text-colorpicker-overlay').css("visibility", "hidden")
    $('#bg-colorpicker-overlay').css("visibility", "hidden")
    $(".app-wrapper").css("filter", 'none')
    colorpickerVisible = false
}

// Make colorpicker closable
$(document).click(function (event) {
    let target = $(event.target);

    let clickedOnColorpicker = colorpickerVisible &&
        (target.closest('#text-colorpicker-overlay').length || target.closest('#bg-colorpicker-overlay').length)

    if (!clickedOnColorpicker) {
        hideColorPicker();
    }
});

if (input.transparent()) {
    $("#transparent-bg").trigger("change")
}

module.exports = {
    textColor: function () {
        return textColorPicker.color.hexString
    },
    bgColor: function () {
        return bgColorPicker.color.hexString
    }
};