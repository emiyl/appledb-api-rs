const fs = require('fs')

var args = process.argv.slice(2)

let new_args = []
let total_arg = ''
for (arg of args) {
    total_arg += arg
    if (arg.endsWith('.json')) {
        new_args.push(total_arg)
        total_arg = ''
    } else {
        total_arg += ' '
    }
}

for (arg of new_args) {
    let file = fs.readFileSync(arg, 'utf8')
    let len = fs.statSync(arg).size
    let last_three_characters = file.slice(-3)
    if (last_three_characters == `,]\n`) {
        const fd = fs.openSync(arg, "r+")
        fs.writeSync(fd, `]\n`, len-3, 'utf8')
    }
}