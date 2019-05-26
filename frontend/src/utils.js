function invertColor(rgb) {
    let r = (255 - rgb.r).toString(16),
        g = (255 - rgb.g).toString(16),
        b = (255 - rgb.b).toString(16);

    return '#' + padZero(r) + padZero(g) + padZero(b);
}

function padZero(str, len) {
    len = len || 2;
    let zeros = new Array(len).join('0');
    return (zeros + str).slice(-len);
}

module.exports = {
    invertColor
}