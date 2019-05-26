module.exports = {
    font: () => $('#font-family').val(),
    fontSize: () => $('#font-size').val(),
    maxWords: () => $('#max-words').val(),
    minWordLength: () => $('#min-word-len').val(),
    rotationPercentage: () => $('#rotation-percentage').val(),
    postRotation: () => $('#post-rotate').val(),
    capitalizeWords: () => $('#capitalize-words').is(':checked'),
    debug: () => $('#debug-mode').is(':checked'),
    transparent: () => $('#transparent-bg').is(':checked')
}

$("#transparent-bg").change(function() {
    if(this.checked) {
        $("#button-open-bg-color-picker").addClass('disabled')
    } else {
        $("#button-open-bg-color-picker").removeClass('disabled')
    }
});