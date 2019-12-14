const getBinary = require('./getBinary');

const binary = getBinary();

if (process.argv.includes('uninstall')) {
	binary.uninstall();
}

if (process.argv.includes('install')) {
	binary.install();
}

